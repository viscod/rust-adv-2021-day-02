pub mod parser;
pub mod submarine;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let mut submarine = parser::parse_submarine("./input_part1.txt").unwrap_or_default();
        submarine.start_trip_part1();
        assert_eq!(submarine.get_coordinate(), 1714680);
    }

    #[test]
    fn test_part_two() {
        let mut submarine = parser::parse_submarine("./input_part2.txt").unwrap_or_default();
        submarine.start_trip_part2();
        assert_eq!(submarine.get_coordinate(), 1963088820);
    }
}
