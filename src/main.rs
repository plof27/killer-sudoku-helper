use clap::Parser;
use solver::KillerCage;

mod solver;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Number of cells in the killer cage
    cell_count: u32,

    /// When provided, compute all possible combinations of cell values that sum to the given total. 
    total: Option<u32>,

    /// The maximum value of a cell in the grid. For a standard sudoku puzzle, this is 9.
    #[clap(short = 'c', long, default_value_t = 9)]
    max_cell_value: u32,

    /// Whether to output the minimum possible sum for a cage of the given size.
    #[clap(short = 'n', visible_alias = "mN", long)]
    minimum: bool,
    
    /// Whether to output the maximum possible sum for a cage of the given size.
    #[clap(short = 'x', visible_alias = "mX", long)]
    maximum: bool,
}

fn main() {
    let args = Args::parse();

    let cage = KillerCage::new(args.max_cell_value, args.cell_count);

    if !args.minimum && !args.maximum && args.total.is_none() {
        println!("Warning: [TOTAL] was ommitted, and neither the -n nor -x options were provided. This is a no-op. This program will now exit.")
    } else {
        if args.minimum {
            println!("Minimum sum: {}", cage.minimum_value());
        }
    
        if args.maximum {
            println!("Maximum sum: {}", cage.maximum_value());
        }
        
        if args.total.is_some() {
            let solutions = cage.find_combinations(args.total.unwrap());

            if solutions.len() == 0 {
                println!("No solutions found.")
            } else {
                for solution in solutions.iter() {
                    println!("{:?}", solution);
                }
            }
        }
    }
}
