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
    let mut count = 0;
    // height of the puzzle is number of lines
    let h = puzzle.len();
    // width of the puzzle is number of chars in each line
    let w = puzzle.first().unwrap().len();

    for i in 0..h {
        for j in 0..w {
            count += count_matches(puzzle, word, i, j);
        }
    }

    count
}

/// Count number of matches for the word from the given starting cell
fn count_matches(puzzle: &Puzzle, word: &str, row: usize, col: usize) -> usize {
    let mut found_count = 0;

    let chars: Vec<char> = word.chars().collect();
    let width = puzzle.first().unwrap().len();
    let height = puzzle.len();

    // if the first char doesn't match, return early
    if puzzle[row][col] != chars[0] {
        return 0;
    }

    // we can index both x and y with a direction, and know if we should move in either direction,
    // e.g. index 0 (north), don't move on the x-axis, move up on the y-axis = [0, 1]
    // index 3 (south-east) move right on the x-axis, move down on the y-axis [1, 1]
    // index 6 (west) move left on the x-axis, don't move on the y-axis [-1, 0]
    //                                    N   NE  E SE  S  SW   W  NW
    #[rustfmt::skip] let x: [isize; 8] = [ 0,  1, 1, 1, 0, -1, -1, -1]; // -1 left, 1 right
    #[rustfmt::skip] let y: [isize; 8] = [-1, -1, 0, 1, 1,  1,  0, -1]; // -1 up, 1 down

    for direction in 0..8 {
        let mut next_row = row as isize;
        let mut next_col = col as isize;

        let mut found = false;

        // start from 1 as we already checked the 0th char
        for i in 1..chars.len() {
            next_row += y[direction as usize];
            next_col += x[direction as usize];

            // out-of-bounds check
            if next_row < 0
                || next_col < 0
                || next_row >= height as isize
                || next_col >= width as isize
            {
                break;
            }

            // char match check
            let curr = puzzle[next_row as usize][next_col as usize];
            let expected_char = chars[i];
            if curr != expected_char {
                break;
            }

            // if we get here, and the iterator is equal to the word length, we found the whole word
            found = i + 1 == chars.len();
        }

        if found {
            found_count += 1;
        }
    }

    found_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_word_not_found() {
        let word = "TEST";
        let input = "
AAAA
AAAA
AAAA
AAAA";
        let puzzle = create_puzzle(input.trim());

        assert_eq!(count_matches(&puzzle, word, 0, 0), 0);
    }

    #[test]
    fn test_find_word_north() {
        let word = "TEST";
        let input = "
TAAA
SAAA
EAAA
TAAA";
        let puzzle = create_puzzle(input.trim());

        assert_eq!(count_matches(&puzzle, word, 3, 0), 1);
    }

    #[test]
    fn test_find_word_ne() {
        let word = "TEST";
        let input = "
AAAT
AASA
AEAA
TAAA
";
        let puzzle = create_puzzle(input.trim());

        assert_eq!(count_matches(&puzzle, word, 3, 0), 1);
    }

    #[test]
    fn test_find_word_east() {
        let word = "TEST";
        let input = "
TEST
AAAA
AAAA
AAAA
";
        let puzzle = create_puzzle(input.trim());

        assert_eq!(count_matches(&puzzle, word, 0, 0), 1);
    }

    #[test]
    fn test_find_word_partial() {
        let word = "TEST";
        let input = "
TESA
AAAA
ATES
ESTA";
        let puzzle = create_puzzle(input.trim());

        assert_eq!(count_matches(&puzzle, word, 0, 0), 0);
    }

    #[test]
    fn test_multiples_in_one_row() {
        let word = "TEST";
        let input = "
TESTTEST
AAAAAAAA";
        let puzzle = create_puzzle(input.trim());

        assert_eq!(count_matches(&puzzle, word, 0, 0), 1);
        assert_eq!(count_matches(&puzzle, word, 0, 4), 1);
    }

    #[test]
    fn test_overlapping_different_directions() {
        let word = "TEST";
        let input = "
TEST
EAAA
SAAA
TAAA";
        let puzzle = create_puzzle(input.trim());

        assert_eq!(count_matches(&puzzle, word, 0, 0), 2);
    }

    #[test]
    fn test_word_search_count() {
        let word = "XMAS";
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let puzzle = create_puzzle(input.trim());
        assert_eq!(word_search_count(&puzzle, word), 18);
    }
}
