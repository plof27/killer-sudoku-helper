/// A struct representing a single cage in a killer sudoku puzzle.
#[derive(PartialEq, Debug)]
pub struct KillerCage {

    /// The maximum value of a cell in the grid. For a standard sudoku puzzle, this is 9.
    max_cell_value: u32,

    /// The number of cells in the cage.
    cell_count: u32,
}

impl KillerCage {

    /// Creates a new `KillerCage`.
    pub fn new(max_cell_value: u32, cell_count: u32) -> Self {

        if cell_count > max_cell_value {
            panic!(
                "`cell_count` must be less than or equal to `max_cell_value`. Got {{max_cell_value: {}, cell_count: {}}}",
                max_cell_value,
                cell_count
            );
        }

        KillerCage { 
            max_cell_value, 
            cell_count 
        }
    }

    /// Computes and returns the maximum valid value for the cell at `index` in a sorted list without repeats.
    /// For example, with 3 total cells, the cell at index 1 can only have a maximum value of `MAX_CELL_VALUE - 1`,
    /// since the final cell (index 2) would need to be larger than it, and it can't exceed the `MAX_CELL_VALUE`.
    fn max_positional_value(&self, index: usize) -> u32 {
        self.max_cell_value - ((self.cell_count-1) - (index as u32))
    }
    
    /// Finds all combinations of cell values that sum to the given total. In standard killer sudoku rules, 
    /// digits cannot repeat within a cage. Additionally, results given are sorted in ascending order.
    pub fn find_combinations(&self, total: u32) -> Vec<Vec<u32>> {
        let mut values: Vec<u32> = (1..(self.cell_count+1)).collect();
        let mut solutions: Vec<Vec<u32>> = Vec::new();
    
        // We need to check the inital value, before it starts geting incremented
        if values.iter().sum::<u32>() == total {
            solutions.push(values.clone());
        }
        
        while values[0] < self.max_positional_value(0) {
            values[(self.cell_count-1) as usize] += 1;
    
            // Iterate backwards, propagaing "carry over" whenever a cell exceeds it's maximum value
            // In both of these loops, it is not necessary to check index 0, since it is checked in the outer loop.
            for i in (1..(self.cell_count as usize)).rev() {
                if values[i] == self.max_positional_value(i) + 1 {
                    values[i-1] += 1;
                }
            }
    
            // Iterate forwards, resetting less significant digits to be 1 greater than their predecessor
            for i in 1..(self.cell_count as usize) {
                if values[i] == self.max_positional_value(i) + 1 {
                    values[i] = values[i-1] + 1;
                }
            }
    
            if values.iter().sum::<u32>() == total {
                solutions.push(values.clone());
            }
        }

        solutions
    }

    /// The minimum possible total of this cage. This is equal to the triangular number for `cell_count`.
    pub fn minimum_value(&self) -> u32 {
        (self.cell_count * (self.cell_count + 1)) / 2
    }

    /// The maximum possible total of this cage. This is formed when the first cell of the cage is set
    /// to `max_cell_value`, then successive cells are filled with descending digits. (For example, the
    /// maximum value of a 3 cell cage would be [9, 8, 7] = 24.)
    pub fn maximum_value(&self) -> u32 {
        let min_value = self.minimum_value();
        min_value + self.cell_count*(self.max_cell_value-self.cell_count)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_valid_cage() {
        assert_eq!(
            KillerCage::new(9, 3),
            KillerCage {
                max_cell_value: 9,
                cell_count: 3,
            }
        )
    }

    #[test]
    #[should_panic]
    fn create_invalid_cage() {
        KillerCage::new(3, 9);
    }

    #[test]
    fn compute_minimum_value() {
        // max_cell_value, cell_count, result
        let test_cases = [
            (9, 1, 1),
            (9, 2, 3),
            (9, 3, 6),
            (9, 4, 10),
            (9, 5, 15),
            (9, 6, 21),
            (9, 7, 28),
            (9, 8, 36),
            (9, 9, 45),
        ];

        for case in test_cases.iter() {
            let cage = KillerCage::new(case.0, case.1);
            assert_eq!(cage.minimum_value(), case.2);
        }
    }

    #[test]
    fn compute_maximum_value() {
        // max_cell_value, cell_count, result
        let test_cases = [
            (9, 1, 9),
            (9, 2, 17),
            (9, 3, 24),
            (9, 4, 30),
            (9, 5, 35),
            (9, 6, 39),
            (9, 7, 42),
            (9, 8, 44),
            (9, 9, 45),
            (13, 1, 13),
            (13, 2, 25),
            (13, 3, 36),
            (1, 1, 1),
        ];

        for case in test_cases.iter() {
            let cage = KillerCage::new(case.0, case.1);
            assert_eq!(cage.maximum_value(), case.2);
        }
    }

    #[test]
    fn compute_max_positional_value() {
        // max_cell_value, cell_count, index, result
        let test_cases = [
            (9, 3, 2, 9),
            (9, 3, 1, 8),
            (9, 3, 0, 7),
            (9, 2, 1, 9),
            (9, 2, 0, 8),
            (9, 8, 7, 9),
            (9, 8, 5, 7),
            (9, 8, 1, 3),
            (9, 8, 0, 2),
            (12, 3, 2, 12),
            (12, 3, 1, 11),
            (12, 3, 0, 10),
        ];

        for case in test_cases.iter() {
            let cage = KillerCage::new(case.0, case.1);
            assert_eq!(cage.max_positional_value(case.2), case.3);
        }
    }

    #[test]
    fn compute_cage_values_for_1_cage() {
        let empty_vec: Vec<Vec<u32>> = Vec::new();

        let cage = KillerCage::new(9, 1);

        // total, results
        let cases = [
            (1, vec![[1]]),
            (2, vec![[2]]),
            (3, vec![[3]]),
            (5, vec![[5]]),
            (9, vec![[9]]),
        ];

        // totals where there are no solutions
        let empty_cases = [0, 10];

        for case in cases.iter() {
            assert_eq!(
                cage.find_combinations(case.0),
                case.1
            )
        }
        for case in empty_cases.iter() {
            assert_eq!(
                cage.find_combinations(*case),
                empty_vec
            )
        }
    }

    #[test]
    fn compute_cage_values_for_2_cage() {
        let empty_vec: Vec<Vec<u32>> = Vec::new();

        let cage = KillerCage::new(9, 2);

        // total, results
        let cases = [
            (3, vec![[1, 2]]),
            (4, vec![[1, 3]]),
            (5, vec![[1, 4], [2, 3]]),
            (6, vec![[1, 5], [2, 4]]),
            (10, vec![[1, 9], [2, 8], [3, 7], [4, 6]]),
            (17, vec![[8, 9]])
        ];

        // totals where there are no solutions
        let empty_cases = [1, 2, 20];

        for case in cases.iter() {
            assert_eq!(
                cage.find_combinations(case.0),
                case.1
            )
        }
        for case in empty_cases.iter() {
            assert_eq!(
                cage.find_combinations(*case),
                empty_vec
            )
        }
    }

    #[test]
    fn compute_cage_values_with_alternate_max_cell_value() {
        let empty_vec: Vec<Vec<u32>> = Vec::new();

        let cage = KillerCage::new(4, 2);

        // total, results
        let cases = [
            (3, vec![[1, 2]]),
            (4, vec![[1, 3]]),
            (5, vec![[1, 4], [2, 3]]),
            (6, vec![[2, 4]]),
            (7, vec![[3, 4]])
        ];

        // totals where there are no solutions
        let empty_cases = [1, 2, 8];

        for case in cases.iter() {
            assert_eq!(
                cage.find_combinations(case.0),
                case.1
            )
        }
        for case in empty_cases.iter() {
            assert_eq!(
                cage.find_combinations(*case),
                empty_vec
            )
        }
    }
}