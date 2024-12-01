use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use itertools::traits::HomogeneousTuple;
use itertools::Itertools;

pub fn read_lines(f: &str) -> Result<Vec<String>, std::io::Error> {
    let file = File::open(f).expect("File does not exist");
    BufReader::new(file)
        .lines()
        .into_iter()
        .collect::<Result<Vec<String>, std::io::Error>>()
}

pub fn split_lines_ws<'a, T>(lines: &'a Vec<String>) -> Option<Vec<T>>
where
    T: HomogeneousTuple<Item = &'a str> + 'a,
{
    lines
        .iter()
        .map(|l| l.split_whitespace().collect_tuple())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_lines_ws() {
        let lines: Vec<String> = vec![
            String::from("123   3456"),
            String::from("43234 23889234"),
            String::from("980234   2309823"),
        ];

        let result: Vec<(&str, &str)> = split_lines_ws(&lines).unwrap();

        let expected: Vec<(&str, &str)> = vec![
            ("123", "3456"),
            ("43234", "23889234"),
            ("980234", "2309823"),
        ];
        assert_eq!(result, expected);
    }
}
