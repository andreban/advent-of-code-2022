use std::str::FromStr;

const SAMPLE_INPUT: &str = r"30373
25512
65332
33549
35390";

#[derive(Debug)]
struct TreeGrid {
    pub width: usize,
    pub height: usize,
    pub tree_heights: Vec<usize>,
}

impl TreeGrid {
    pub fn height_at(&self, row: usize, col: usize) -> usize {
        if row >= self.height || col >= self.width {
            panic!("Received row or col outside bounds");
        }

        self.tree_heights[row * self.width + col]
    }

    pub fn visible(&self, row: usize, col: usize) -> bool {
        self.visible_north(row, col)
            || self.visible_south(row, col)
            || self.visible_east(row, col)
            || self.visible_west(row, col)
    }

    pub fn visible_north(&self, row: usize, col: usize) -> bool {
        if row == 0 || col == 0 || row >= self.height || col >= self.width {
            return true;
        }

        let main_tree_height = self.height_at(row, col);

        for row in 0..row {
            if self.height_at(row, col) >= main_tree_height {
                return false;
            }
        }
        true
    }

    pub fn visible_south(&self, row: usize, col: usize) -> bool {
        if row == 0 || col == 0 || row >= self.height || col >= self.width {
            return true;
        }

        let main_tree_height = self.height_at(row, col);

        for row in row + 1..self.height {
            if self.height_at(row, col) >= main_tree_height {
                return false;
            }
        }
        true
    }

    pub fn visible_west(&self, row: usize, col: usize) -> bool {
        if row == 0 || col == 0 || row >= self.height || col >= self.width {
            return true;
        }

        let main_tree_height = self.height_at(row, col);

        for col in 0..col {
            if self.height_at(row, col) >= main_tree_height {
                return false;
            }
        }
        true
    }

    pub fn visible_east(&self, row: usize, col: usize) -> bool {
        if row == 0 || col == 0 || row >= self.height || col >= self.width {
            return true;
        }

        let main_tree_height = self.height_at(row, col);
        for col in col + 1..self.width {
            if self.height_at(row, col) >= main_tree_height {
                return false;
            }
        }
        true
    }

    pub fn scenic_score(&self, row: usize, col: usize) -> usize {
        self.scenic_score_north(row, col)
            * self.scenic_score_south(row, col)
            * self.scenic_score_east(row, col)
            * self.scenic_score_west(row, col)
    }

    pub fn scenic_score_north(&self, row: usize, col: usize) -> usize {
        if row == 0 {
            return 0;
        }

        let main_tree_height = self.height_at(row, col);
        let mut num_trees = 0;

        for row in (0..row).rev() {
            num_trees += 1;
            if self.height_at(row, col) >= main_tree_height {
                break;
            }
        }   
        num_trees
    }

    pub fn scenic_score_south(&self, row: usize, col: usize) -> usize {
        if row == self.height - 1 {
            return 0;
        }

        let main_tree_height = self.height_at(row, col);
        let mut num_trees = 0;

        for row in row+1..self.height {
            num_trees += 1;
            if self.height_at(row, col) >= main_tree_height {
                break;
            }
        }   
        num_trees
    }

    pub fn scenic_score_east(&self, row: usize, col: usize) -> usize {
        if col == 0 {
            return 0;
        }

        let main_tree_height = self.height_at(row, col);
        println!("Origin Height: {}", main_tree_height);
        let mut num_trees = 0;

        for col in (0..col).rev() {
            println!("row: {}, col: {}, height: {}", row, col, self.height_at(row, col));
            num_trees += 1;
            if self.height_at(row, col) >= main_tree_height {
                break;
            }
        }   
        num_trees
    }

    pub fn scenic_score_west(&self, row: usize, col: usize) -> usize {
        if col == self.width - 1 {
            return 0;
        }

        let main_tree_height = self.height_at(row, col);
        let mut num_trees = 0;

        for col in col+1..self.height {
            num_trees += 1;
            if self.height_at(row, col) >= main_tree_height {
                break;
            }
        }   
        num_trees
    }
}

impl FromStr for TreeGrid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().unwrap().len(); // Use length of first line as width.
        let mut height = 0;
        let mut tree_heights = vec![];
        for line in s.lines() {
            height += 1;
            for char in line.chars() {
                tree_heights.push(char.to_digit(10).unwrap() as usize);
            }
        }
        Ok(TreeGrid {
            width,
            height,
            tree_heights,
        })
    }
}

fn main() {
    let tree_grid = include_str!("day8.txt").parse::<TreeGrid>().unwrap();
    // let tree_grid = SAMPLE_INPUT.parse::<TreeGrid>().unwrap();
    let mut visible_count = 0;
    let mut highest_scenic_score = 0;
    for row in 0..tree_grid.height {
        for col in 0..tree_grid.width {
            if tree_grid.visible(row, col) {
                visible_count += 1;
            }

            let scenic_score = tree_grid.scenic_score(row, col);
            if scenic_score > highest_scenic_score {
                highest_scenic_score = scenic_score;
            }
        }
    }

    println!("Part 1: {}", visible_count);
    println!("Part 2: {}", highest_scenic_score);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn visible_north() {
        let tree_grid = SAMPLE_INPUT.parse::<TreeGrid>().unwrap();
        assert!(tree_grid.visible_north(1, 1));
        assert!(tree_grid.visible_north(1, 2));
    }

    #[test]
    fn visible_south() {
        let tree_grid = SAMPLE_INPUT.parse::<TreeGrid>().unwrap();
        assert!(!tree_grid.visible_south(1, 1));
        assert!(!tree_grid.visible_south(1, 2));
    }

    #[test]
    fn visible_west() {
        let tree_grid = SAMPLE_INPUT.parse::<TreeGrid>().unwrap();
        assert!(tree_grid.visible_west(1, 1));
        assert!(!tree_grid.visible_west(1, 2));
    }

    #[test]
    fn visible_east() {
        let tree_grid = SAMPLE_INPUT.parse::<TreeGrid>().unwrap();
        assert!(!tree_grid.visible_east(1, 1));
        assert!(tree_grid.visible_east(1, 2));
    }

    #[test]
    fn scenic_score_north() {
        let tree_grid = SAMPLE_INPUT.parse::<TreeGrid>().unwrap();
        assert_eq!(tree_grid.scenic_score_north(1, 2), 1);
        assert_eq!(tree_grid.scenic_score_north(3, 2), 2);
    }

    #[test]
    fn scenic_score_south() {
        let tree_grid = SAMPLE_INPUT.parse::<TreeGrid>().unwrap();
        assert_eq!(tree_grid.scenic_score_south(1, 2), 2);
        assert_eq!(tree_grid.scenic_score_south(3, 2), 1);
    }

    #[test]
    fn scenic_score_east() {
        let tree_grid = SAMPLE_INPUT.parse::<TreeGrid>().unwrap();
        assert_eq!(tree_grid.scenic_score_east(1, 2), 1);
        assert_eq!(tree_grid.scenic_score_east(3, 2), 2);
    }

    #[test]
    fn scenic_score_west() {
        let tree_grid = SAMPLE_INPUT.parse::<TreeGrid>().unwrap();
        assert_eq!(tree_grid.scenic_score_west(1, 2), 2);
        assert_eq!(tree_grid.scenic_score_west(3, 2), 2);
    }

    #[test]
    fn scenic_score() {
        let tree_grid = SAMPLE_INPUT.parse::<TreeGrid>().unwrap();
        assert_eq!(tree_grid.scenic_score(1, 2), 4);
        assert_eq!(tree_grid.scenic_score(3, 2), 8);
    }
}
