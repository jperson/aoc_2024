#![feature(iter_advance_by)]
use anyhow::{anyhow, Error, Result};
//use thiserror::Error;

// #[derive(Error, Debug)]
// pub enum PError<'p> {
//     #[error("parse failed")]
//     ParseFailed(&'p str),
//     #[error("unknown data store error")]
//     Unknown,
// }

pub type PResult<I, O, E = anyhow::Error> = Result<(I, O), E>;

fn next_boundary<'p>(s: &'p str) -> usize {
    let mut i = 1;
    while i < s.len() {
        if s.is_char_boundary(i) {
            return i;
        } else {
            i += 1;
        }
    }
    s.len()
}

pub fn peek<'p>(s: &'p str) -> PResult<&'p str, char, Error> {
    if s.is_empty() {
        Err(anyhow!("peek failed, empty str"))
    } else {
        let (c, _) = s.split_at(next_boundary(s));
        Ok((s, c.chars().next().unwrap()))
    }
}

pub fn next<'p>(s: &'p str) -> PResult<&'p str, char, Error> {
    if s.is_empty() {
        Err(anyhow!("peek failed, empty str"))
    } else {
        let (c, p) = s.split_at(next_boundary(s));
        Ok((p, c.chars().next().unwrap()))
    }
}

pub fn lit<'p>(l: &'p str) -> impl Fn(&'p str) -> PResult<&'p str, &'p str, Error> {
    return move |s| {
        let mut source = s.char_indices();
        let mut lit = l.char_indices();
        if !s.is_empty()
            && lit
                .by_ref()
                .zip(source.by_ref())
                .fold(true, |acc, ((_, x), (_, y))| acc & (x == y))
        {
            if let Some((i, _)) = source.next() {
                let (p, r) = s.split_at(i);
                Ok((r, p))
            } else {
                Ok(("", s))
            }
        } else {
            Err(anyhow!("Failed to match lit"))
        }
    };
}

pub fn take_max_n_while<'p, P>(
    max: usize,
    predicate: P,
) -> impl Fn(&'p str) -> PResult<&'p str, &'p str, Error>
where
    P: Fn(&char) -> bool,
{
    return move |s| {
        let mut source = s.char_indices();
        let mut i = 0;
        let mut ni = 0;

        while i < max {
            if let Some((n, c)) = source.next() {
                if !predicate(&c) {
                    let (p, r) = s.split_at(n);
                    return Ok((r, p));
                } else {
                    i += 1;
                    ni = n;
                }
            } else {
                break;
            }
        }

        if let Some((nn, _)) = source.next() {
            ni = nn;
        }
        let (p, r) = s.split_at(ni);
        Ok((r, p))
    };
}

pub fn take_min_n_while<'p, P>(
    min: usize,
    predicate: P,
) -> impl Fn(&'p str) -> PResult<&'p str, &'p str, Error>
where
    P: Fn(&char) -> bool,
{
    return move |s| {
        let (p, r) = take_while(&predicate)(s)?;
        if r.len() < min {
            Err(anyhow!("Failed to take min"))
        } else {
            Ok((p, r))
        }
    };
}

pub fn take_while<'p, P>(predicate: P) -> impl Fn(&'p str) -> PResult<&'p str, &'p str, Error>
where
    P: Fn(&char) -> bool,
{
    return move |s| {
        let mut source = s.char_indices();

        loop {
            if let Some((n, c)) = source.next() {
                if !predicate(&c) {
                    let (p, r) = s.split_at(n);
                    return Ok((r, p));
                }
            } else {
                //BUGBUGBUG???
                return Ok(("", s));
            }
        }
    };
}

pub fn take_until<'p, F>(op: F) -> impl Fn(&'p str) -> PResult<&'p str, &'p str, Error>
where
    F: Fn(&'p str) -> PResult<&'p str, &'p str, Error>,
{
    return move |s| loop {
        let mut ns: &'p str = s;
        let mut i: usize = 0;

        loop {
            if let Ok(_) = op(ns) {
                let (p, r) = s.split_at(i);
                return Ok((r, p));
            } else {
                if let Ok((p, _)) = next(ns) {
                    ns = p;
                    i += 1;
                } else {
                    return Err(anyhow!("Failed to parse"));
                }
            }
        }
    };
}

pub fn skip_while<'p, P>(predicate: &'p P) -> impl Fn(&'p str) -> PResult<&'p str, (), Error>
where
    P: Fn(&char) -> bool + 'p,
{
    return move |s| {
        let (p, _) = take_while(predicate)(s)?;
        Ok((p, ()))
    };
}

pub fn left<'p, LO, RO, LF, RF>(lf: LF, rf: RF) -> impl Fn(&'p str) -> PResult<&'p str, LO, Error>
where
    LF: Fn(&'p str) -> PResult<&'p str, LO, Error>,
    RF: Fn(&'p str) -> PResult<&'p str, RO, Error>,
{
    return move |s| {
        if let Ok((p, res)) = lf(s) {
            if let Ok((pr, _)) = rf(p) {
                Ok((pr, res))
            } else {
                Err(anyhow!("Failed to parse right side"))
            }
        } else {
            Err(anyhow!("Failed to parse left side"))
        }
    };
}

pub fn right<'p, LO, RO, LF, RF>(lf: LF, rf: RF) -> impl Fn(&'p str) -> PResult<&'p str, RO, Error>
where
    LF: Fn(&'p str) -> PResult<&'p str, LO, Error>,
    RF: Fn(&'p str) -> PResult<&'p str, RO, Error>,
{
    return move |s| {
        if let Ok((p, _)) = lf(s) {
            if let Ok((p, r)) = rf(p) {
                Ok((p, r))
            } else {
                Err(anyhow!("Failed to parse right"))
            }
        } else {
            Err(anyhow!("Failed to parse left"))
        }
    };
}

pub fn many0<'p, O, F>(op: F) -> impl Fn(&'p str) -> PResult<&'p str, Vec<O>, Error>
where
    F: Fn(&'p str) -> PResult<&'p str, O, Error>,
{
    return move |s| {
        let mut res: Vec<O> = Vec::new();
        let mut ss = s;

        while let Ok((p, r)) = op(ss) {
            res.push(r);
            ss = p;
        }

        Ok((ss, res))
    };
}

pub fn many1<'p, O, F>(op: F) -> impl Fn(&'p str) -> PResult<&'p str, Vec<O>, Error>
where
    F: Fn(&'p str) -> PResult<&'p str, O, Error>,
{
    return move |s| {
        let mut res: Vec<O> = Vec::new();
        let mut ss = s;

        if let Ok((p, r)) = op(ss) {
            res.push(r);
            ss = p;

            if let Ok((p, mut vs)) = many0(&op)(ss) {
                res.append(&mut vs);
                Ok((p, res))
            } else {
                Ok((ss, res))
            }
        } else {
            Err(anyhow!("Failed to parse"))
        }
    };
}

pub fn opt<'p, O, F>(fp: F) -> impl Fn(&'p str) -> PResult<&'p str, O, Error>
where
    F: Fn(&'p str) -> PResult<&'p str, O, Error>,
    O: std::default::Default,
{
    return move |s| fp(s).or(Ok((s, Default::default())));
}

#[macro_export]
macro_rules! seq {
    ($src:ident => $($ops:expr), +) => {
        //fn __f<'p>(__s: &'p str) -> PResult<&'p str, (), Error> {
        (|| -> PResult<&str, (), Error> {
            let __s = $src;
            $( let (__s, _) = $ops(__s)?; )*
            Ok((__s, ()))
        })()
    };
}

pub fn one_of<'p, O, F>(ops: Vec<F>) -> impl Fn(&'p str) -> PResult<&'p str, O, Error>
where
    F: Fn(&'p str) -> PResult<&'p str, O, Error>,
{
    return move |s| {
        for op in &ops {
            let v = op(s);
            if v.is_ok() {
                return v;
            }
        }
        Err(anyhow!("Failed to parse one_of"))
    };
}

#[macro_export]
macro_rules! one_of {
    ($($ops:expr),*) => {
        one_of(vec![$($ops),*])
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lit() {
        let src = "fo捒o: bar";
        //let src = "捒";
        let (p, r) = lit("fo捒o")(src).unwrap();

        assert_eq!(r, "fo捒o");
        assert_eq!(p, ": bar");

        let src = "ff";
        //let src = "捒";
        let (p, r) = lit("ff")(src).unwrap();

        assert_eq!(r, "ff");
        assert_eq!(p, "");

        let src = "";
        let x = lit("foo")(src);
        println!("{:?}", x);
    }

    #[test]
    fn test_peek() {
        let src = "test";
        //let src = "捒";
        let (p, c) = peek(src).unwrap();

        assert_eq!(c, 't');
        assert_eq!(p, "test");
    }

    #[test]
    fn test_next() {
        let src = "test";
        let (p, c) = next(src).unwrap();

        assert_eq!(c, 't');
        assert_eq!(p, "est");
    }

    #[test]
    fn test_left() {
        let src = "foobartest";
        let (p, c) = left(lit("foo"), lit("bar"))(src).unwrap();
        assert_eq!(c, "foo");
        assert_eq!(p, "test");
    }

    #[test]
    fn test_right() {
        let src = "foobartest";
        let (p, c) = right(lit("foo"), lit("bar"))(src).unwrap();
        assert_eq!(c, "bar");
        assert_eq!(p, "test");
    }

    #[test]
    fn test_take_while() {
        let src = "1234捒a";
        let (p, s) = take_while(|c: &char| c.is_ascii_digit())(src).unwrap();

        assert_eq!(p, "捒a");
        assert_eq!(s, "1234");
    }

    #[test]
    fn test_take_max_n_while() {
        let src = "123";
        let (p, s) = take_max_n_while(2, |c: &char| c.is_ascii_digit())(src).unwrap();
        assert_eq!(p, "3");
        assert_eq!(s, "12");

        let src = "ab123";
        let (p, s) = take_max_n_while(2, |c: &char| c.is_ascii_digit())(src).unwrap();
        assert_eq!(p, "ab123");
        assert_eq!(s, "");

        let src = "1a23";
        let (p, s) = take_max_n_while(2, |c: &char| c.is_ascii_digit())(src).unwrap();
        assert_eq!(p, "a23");
        assert_eq!(s, "1");
    }

    #[test]
    fn test_take_min_n_while() {
        let src = "123";
        let (p, s) = take_min_n_while(2, |c: &char| c.is_ascii_digit())(src).unwrap();
        assert_eq!(p, "");
        assert_eq!(s, "123");

        let src = "ab123";
        let res = take_min_n_while(2, |c: &char| c.is_ascii_digit())(src);
        assert!(res.is_err());

        let src = "1a23";
        let res = take_min_n_while(2, |c: &char| c.is_ascii_digit())(src);
        assert!(res.is_err());

        let src = "123";
        let res = take_min_n_while(4, |c: &char| c.is_ascii_digit())(src);
        assert!(res.is_err());
    }

    #[test]
    fn test_opt() {
        let src = "123";
        let (p, s) = opt(lit("a"))(src).unwrap();
        println!("{:?}, {:?}", p, s);
    }

    #[test]
    fn test_many0() {
        let src = "foofoofoobar";
        let (p, s) = many0(lit("foo"))(src).unwrap();
        assert_eq!(p, "bar");
        assert_eq!(s, ["foo", "foo", "foo"].to_vec());

        let src = "foofoofoo";
        let (p, s) = many0(lit("bar"))(src).unwrap();
        assert_eq!(p, "foofoofoo");
        assert!(s.is_empty());
    }

    #[test]
    fn test_many1() {
        let src = "foofoofoobar";
        let (p, s) = many1(lit("foo"))(src).unwrap();
        assert_eq!(p, "bar");
        assert_eq!(s, ["foo", "foo", "foo"].to_vec());

        let src = "foofoofoo";
        let res = many1(lit("bar"))(src);
        assert!(res.is_err());
    }

    #[test]
    fn test_one_of() {
        let src = "foobar";
        let (p, s) = one_of(vec![lit("bar"), lit("foo")])(src).unwrap();
        assert_eq!(p, "bar");
        assert_eq!(s, "foo");

        let src = "foobar";
        let (p, s) = one_of!(lit("bar"), lit("foo"))(src).unwrap();
        assert_eq!(p, "bar");
        assert_eq!(s, "foo");
    }

    #[test]
    fn test_take_until() {
        let src = "barbarfoo\r\n";
        let (p, s) = take_until(lit("\r\n"))(src).unwrap();
        assert_eq!(p, "\r\n");
        assert_eq!(s, "barbarfoo");
    }

    #[test]
    fn test_parse_data_field() {
        let src = "data: foofoo\r\nbarbar";
        let (p, f) = left(take_while(|c| *c != ':'), lit(":"))(src).unwrap();
        let (p, d) = take_until(lit("\r\n"))(p).unwrap();
        println!("{} - {}: {}", p, f, d.trim());
    }

    #[test]
    fn test_seq() {
        let src = "foo,bar,baztest";
        let (p, _) = seq!(src => lit("foo,"), lit("bar,"), lit("baz")).unwrap();
        assert_eq!(p, "test");
    }
}
