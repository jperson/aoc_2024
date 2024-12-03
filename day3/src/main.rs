use anyhow::Error;
use tinyparse::*;

#[derive(Debug)]
enum Command {
    MUL(i32),
    DO,
    DONT,
}

fn main() {
    let input = include_str!("../../input/day3/input.txt");

    println!("part 1: {}", part1(input).unwrap());
    println!("part 2: {}", part2(input).unwrap());
}

fn part1(mut src: &str) -> Result<i32, Error> {
    let mut total: i32 = 0;
    while !src.is_empty() {
        let (p, ns) = many0(parse_mul)(src).unwrap();
        for n in ns {
            match n {
                Command::MUL(v) => total += v,
                _ => continue,
            }
        }
        (src, _) = next(p)?;
    }
    Ok(total)
}

fn part2(mut src: &str) -> Result<i32, Error> {
    let mut total = 0;
    let mut state: bool = true;

    while !src.is_empty() {
        let (p, ns) = many0(one_of!(parse_mul, parse_dont, parse_do))(src).unwrap();
        for n in ns {
            match n {
                Command::MUL(v) if state => total += v,
                Command::MUL(_) => continue,
                Command::DO => state = true,
                Command::DONT => state = false,
            }
        }
        (src, _) = next(p)?;
    }

    Ok(total)
}

fn parse_mul(s: &str) -> Result<(&str, Command), Error> {
    let (p, a) = right(
        lit("mul("),
        take_max_n_while(3, |c: &char| c.is_ascii_digit()),
    )(s)?;

    let (p, b) = right(lit(","), take_max_n_while(3, |p: &char| p.is_ascii_digit()))(p)?;
    let (p, _) = lit(")")(p)?;

    let (a, b) = (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap());
    Ok((p, Command::MUL(a * b)))
}

fn parse_do(s: &str) -> Result<(&str, Command), Error> {
    let (p, _) = lit("do")(s)?;
    Ok((p, Command::DO))
}

fn parse_dont(s: &str) -> Result<(&str, Command), Error> {
    let (p, _) = lit("don't")(s)?;
    Ok((p, Command::DONT))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let src = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = part1(src).unwrap();
        assert_eq!(161, result);
    }

    #[test]
    fn test_part_2() {
        let src = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let result = part2(src).unwrap();
        assert_eq!(48, result);
    }
}
