pub mod part1;
pub mod part2;
pub mod file_reader;

#[cfg(test)]
mod tests {

    use part1::*;
    use part2::*;
    use super::*;

    #[test]
    fn test_part_one() {
        let input = file_reader::read_from_file("./input_part1.txt");
        let result = increase_count(&input);
        assert_eq!(result, 1832);
    }

    #[test]
    fn test_part_two() {
        let input = file_reader::read_from_file("./input_part2.txt");

        let result = increase_count_window(&input);
        assert_eq!(result, 1858);
    }
}
