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

pub fn split_lines_vec(lines: &Vec<String>) -> Vec<Vec<&str>> {
    lines
        .iter()
        .map(|l| l.split_whitespace().collect::<Vec<&str>>())
        .collect()
}

pub fn remove_ith<T: Clone>(vs: &Vec<T>) -> Vec<Vec<T>> {
    (0..vs.len())
        .into_iter()
        .map(|i| {
            vs[..i]
                .iter()
                .cloned()
                .chain(vs[(i + 1)..].iter().cloned())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
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

        let expected: Vec<(&str, &str)> = vec![
            ("123", "3456"),
            ("43234", "23889234"),
            ("980234", "2309823"),
        ];

        let result: Vec<(&str, &str)> = split_lines_ws(&lines).unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn test_split_lines_vec() {
        let lines = vec![String::from("1 2 3 4 5"), String::from("2 3 4 5 6")];
        let expected: Vec<Vec<String>> = vec![
            vec![1, 2, 3, 4, 5]
                .into_iter()
                .map(|v: i32| v.to_string())
                .collect(),
            vec![2, 3, 4, 5, 6]
                .into_iter()
                .map(|v: i32| v.to_string())
                .collect(),
        ];
        assert_eq!(expected, split_lines_vec(&lines));
    }

    #[test]
    fn test_remove_ith() {
        let vs: Vec<i32> = vec![7, 6, 4, 2, 1];
        let expected = vec![
            vec![6, 4, 2, 1],
            vec![7, 4, 2, 1],
            vec![7, 6, 2, 1],
            vec![7, 6, 4, 1],
            vec![7, 6, 4, 2],
        ];

        let result = remove_ith(&vs);

        assert_eq!(expected, result);
    }
}
