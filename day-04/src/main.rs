use std::fs;

// puzzle is a 2d grid, e.g.
/*
   [
       ['X', 'M', 'A'],
       ['M', 'A', 'S'],
       ['A', 'S', 'X'],
   ]
*/
type Puzzle = Vec<Vec<char>>;
type Coord = (usize, usize);

fn main() {
    let path = "input.txt";
    let contents = fs::read_to_string(&path).expect("input.txt not found");

    let puzzle = create_puzzle(&contents);

    let word = "XMAS";
    let result = word_search_count(&puzzle, word);

    println!("{}", result);
}

fn create_puzzle(input: &str) -> Puzzle {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect()
}

/// Count number of times given word appears in the puzzle, searching in 8 directions from each cell
fn word_search_count(puzzle: &Puzzle, word: &str) -> usize {
    let mut found_instances: Vec<Coord> = Vec::new();
    // height of the puzzle is number of lines
    let h = puzzle.len();
    // width of the puzzle is number of chars in each line
    let w = puzzle.first().unwrap().len();

    for i in 0..h {
        for j in 0..w {
            match find_word(puzzle, word, i, j) {
                true => found_instances.push((i, j)),
                _ => {}
            }
        }
    }

    found_instances.len()
}

fn find_word(puzzle: &Puzzle, word: &str, row: usize, col: usize) -> bool {
    let chars: Vec<char> = word.chars().collect();

    if puzzle[row][col] != chars[0] {
        return false;
    }

    // 0..7 indicates a direction, e.g. 0 is north, 1 is north-east, 2 is east, 3 south-east, etc
    let directions = 8;

    // we can index both x and y with a direction, and know if we should move in either direction,
    // e.g. index 0 (north), don't move on the x-axis, move up on the y-axis = [0, 1]
    // index 3 (south-east) move right on the x-axis, move down on the y-axis [1, 1]
    // index 6 (west) move left on the x-axis, don't move on the y-axis [-1, 0]
    //               N NE  E  SE   S  SW  W   NW
    let x = [0, 1, 1, 1, 0, -1, -1, -1]; // -1 move left, 0 don't move, 1 move right
    let y = [1, 1, 0, -1, -1, -1, 0, 1]; // 1 move up, 0 don't move, -1 move down

    for i in 0..directions {}

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_word_not_found() {
        let word = "TEST";
        let puzzle = create_puzzle(&"ABC\nDEF\nGHI");

        assert_eq!(find_word(&puzzle, word, 0, 0), false);
    }
}
