// TODO: Infinite grid
const GRID_SIZE: usize = 100;
pub type GridIndex = usize;
#[derive(Clone)]
pub struct Grid<T> {
    grid: Vec<Vec<T>>
}

impl<T> Default for Grid<T> where T: Default + Clone {
    fn default() -> Self {
        Self {
            grid: vec![vec![T::default(); GRID_SIZE]; GRID_SIZE]
        }
    }
}

impl<T> Grid<T> where T: Default + Copy {
    pub fn get(&self, x: GridIndex, y: GridIndex) -> T {
        if x < GRID_SIZE && y < GRID_SIZE {
            self.grid[y][x]
        } else {
            T::default()
        }
    }

    pub fn set(&mut self, x: GridIndex, y: GridIndex, val: T) {
        if x < GRID_SIZE && y < GRID_SIZE {
            self.grid[y][x] = val;
        } else {
            println!("Warning: Tried to set out-of-bounds grid cell ({}, {})", x, y);
        }
    }
}

#[derive(Clone, Debug)]
pub struct Atom {
    pub words: Vec<String>,

    // Pattern is stored as a bitmask. The n-th bit (lsb is 0) encodes the
    // (n%5)th pixel from the right in the (n/5)th row from the bottom.
    pub pattern: u32,
}

impl Atom {
    pub fn new(words: Vec<String>, pattern: u32) -> Self {
        Self { words, pattern }
    }
    
    // Csv format description:
    // First element is the pattern represented as a u32 as described above.
    //   Base 10 with no prefix, base 2 with "0b" prefix, or base 16 with "0x" prefix.
    // The next elements are words associated with the pattern.
    pub fn from_csv_record(record: &csv::StringRecord) -> Self {
        let pattern_str = &record[0];
        let radix = if pattern_str.starts_with("0b") {
            2
        } else if pattern_str.starts_with("0x") {
            16
        } else {
            10
        };
        let number_no_prefix = if radix == 10 {
            pattern_str
        } else {
            &pattern_str[2..]
        };
        let pattern = u32::from_str_radix(number_no_prefix, radix)
            .expect("Invalid pattern format");
        let words = record
            .iter()
            .skip(1)
            .map(|s| s.trim().to_string())
            .collect();
        Self::new(words, pattern)
    }
    
    pub fn contains(&self, query: &str) -> bool {
        self.words.iter().any(|word| word.to_lowercase().contains(&query.to_lowercase()))
    }
    
    pub fn nth_bit(&self, n: usize) -> bool {
        if n >= 25 {
            panic!("Bit index out of range");
        }
        (self.pattern >> (24 - n)) & 1 != 0
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    SearchInputChanged(String),
    FocusSearchInput,
    CellClicked(GridIndex, GridIndex),
    CursorMovedToCell(GridIndex, GridIndex),
    MouseReleased,
    SelectAtom(Atom),
    UnselectAtom,
}