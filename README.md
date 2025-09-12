
https://github.com/user-attachments/assets/b47ada6d-8741-4b74-9f26-319dd6e09b7b

# pixel-editor
A tool for pixel writing.

## Table of contents
1. [Description](#description)
2. [Compilation](#compilation)
3. [Planned features](#planned-features)
4. [Usage](#usage)
   1. [Movement](#movement)
   2. [Zooming](#zooming)
   3. [Drawing](#drawing)
   4. [Keys](#keys)
   5. [Data file](#data-file)

## Description
This tools is designed to let the user import a set of 5x5 *patterns* that can be searched by keywords.
You search the pattern you want, you click it and then you click where you want to paste it.

## Compilation
Just like any cargo package.
```bash
cargo build --realease
```

## Planned features
- [ ] Save drawing
- [ ] Import drawing
- [ ] Export drawing
- [ ] Settings file
- [ ] Settings GUI
- [ ] Colors
- [ ] Select cells
- [ ] Delete selection
- [ ] Display selection's meaning

## Usage
### Movement
- Click and drag with the middle button to move the canvas.
### Zooming
- Zoom in and out with the mouse wheel.
### Drawing
- Left click to switch a pixel on and off.
- Left click and drag will copy the resulting color after switching to the pixels you pass by.
### Keys
- `/` to focus the search bar.
- `Esc` to unselect a pattern without pasting it.
### Data file
Create a file called `data.csv` with your patterns in the following format (no headers):
```csv
pattern1, word1, word2, word3
pattern2, word4, it can be a phrase, word5, wor6
[...]
```
As for the pattern, it's a binary representation of it, from left to right, top to bottom, msb first.
If you pattern is:
```
01001
10100
10101
10101
```
Valid representations would be: `0b01001101001010110101`, `0x4d2b5` and `316085`.
