# killer-sudoku-helper
A small command-line tool to help compute possible values for [killer sudoku](https://en.wikipedia.org/wiki/Killer_sudoku#Rules) cages.

## Quick start
1. Install [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)
2. Clone the repo
3. Run `cargo run -- --help`


## Usage

```
killer-sudoku-helper 0.1.0
Rose Peck <rosepeck1997@gmail.com>
A small command-line tool to help compute possible values for killer sudoku cages.

USAGE:
    killer-sudoku-helper [OPTIONS] <CELL_COUNT> [TOTAL]

ARGS:
    <CELL_COUNT>    Number of cells in the killer cage
    <TOTAL>         When provided, compute all possible combinations of cell values that sum to
                    the given total

OPTIONS:
    -c, --max-cell-value <MAX_CELL_VALUE>
            The maximum value of a cell in the grid. For a standard sudoku puzzle, this is 9
            [default: 9]

    -h, --help
            Print help information

    -n, --minimum
            Whether to output the minimum possible sum for a cage of the given size [aliases: mN]

    -V, --version
            Print version information

    -x, --maximum
            Whether to output the maximum possible sum for a cage of the given size [aliases: mX]
```