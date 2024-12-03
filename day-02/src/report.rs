pub enum ReportResult {
    Safe,
    Unsafe,
}

pub fn check_report(report: &str) -> ReportResult {
    let values = report
        .split_whitespace()
        .map(|s| s.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    // check if original report is safe
    match check_is_safe(&values) {
        ReportResult::Safe => return ReportResult::Safe,
        _ => {}
    }

    // if not, see if removing any single element makes it safe
    let permutations = values
        .iter()
        .enumerate()
        .map(|(i, _)| {
            let mut new = values.clone();
            new.remove(i);

            new
        })
        .collect::<Vec<Vec<_>>>();

    for permutation in permutations {
        match check_is_safe(&permutation) {
            ReportResult::Safe => return ReportResult::Safe,
            _ => continue,
        }
    }

    ReportResult::Unsafe
}

fn check_is_safe(values: &Vec<i32>) -> ReportResult {
    let min_step = 1;
    let max_step = 3;
    let dir = match get_direction(values[0], values[1]) {
        Some(dir) => dir,
        None => return ReportResult::Unsafe,
    };

    for i in 1..values.len() {
        let curr = values[i];
        let prev = values[i - 1];

        // Check if the direction matches the overall direction
        match get_direction(prev, curr) {
            Some(curr_dir) if curr_dir == dir => {}
            _ => return ReportResult::Unsafe,
        }

        match (prev - curr).abs() {
            diff if diff < min_step => return ReportResult::Unsafe,
            diff if diff > max_step => return ReportResult::Unsafe,
            _ => {}
        };
    }

    ReportResult::Safe
}

#[derive(PartialEq, Eq)]
enum Direction {
    Inc,
    Dec,
}

fn get_direction(a: i32, b: i32) -> Option<Direction> {
    match a - b {
        diff if diff > 0 => Some(Direction::Inc),
        diff if diff < 0 => Some(Direction::Dec),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_report_safe() {
        let input = "7 6 4 2 1";
        let result = check_report(input);
        assert!(matches!(result, ReportResult::Safe));
    }

    #[test]
    fn check_report_unsafe_large_increase() {
        let input = "1 2 7 8 9";
        let result = check_report(input);
        assert!(matches!(result, ReportResult::Unsafe));
    }

    #[test]
    fn check_report_unsafe_large_decrease() {
        let input = "9 7 6 2 1";
        let result = check_report(input);
        assert!(matches!(result, ReportResult::Unsafe));
    }

    #[test]
    fn test_check_report_unsafe_multiple_inc_and_dec() {
        let input = "1 3 2 4 5";
        let result = check_report(input);
        assert!(matches!(result, ReportResult::Unsafe));
    }

    #[test]
    fn test_check_report_safe_single_inc_and_dec() {
        let input = "1 3 2 4 5";
        let result = check_report(input);
        assert!(matches!(result, ReportResult::Unsafe));
    }

    #[test]
    fn test_check_report_unsafe_multiple_repeat_values() {
        let input = "8 4 4 4 1";
        let result = check_report(input);
        assert!(matches!(result, ReportResult::Unsafe));
    }

    #[test]
    fn test_check_report_safe_single_repeat_value() {
        let input = "8 6 4 4 1";
        let result = check_report(input);
        assert!(matches!(result, ReportResult::Safe));
    }
}
