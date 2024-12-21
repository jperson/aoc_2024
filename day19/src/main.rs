use anyhow::{anyhow, Error};
use memoize::memoize;
use rustc_hash::{FxHashMap, FxHashSet};

fn main() {
    let input = include_str!("../../input/day19/input.txt");

    //println!("part 1: {}", part1(input).unwrap());
    println!("part 2: {}", part2_memo(input).unwrap());
}

fn parse_input<'a>(src: &'a str) -> (Vec<&'a str>, Vec<&'a str>) {
    let (a, b) = src.split_once("\n\n").unwrap();

    (
        a.trim().split(", ").collect::<Vec<_>>(),
        b.trim().lines().collect::<Vec<_>>(),
    )
}

fn count(d: &str, ts: &[&str]) -> i64 {
    let mut ds: Vec<i64> = vec![0; d.len() + 1];
    ds[0] = 1;

    for i in 0..d.len() {
        for t in ts {
            if d[i..].starts_with(t) {
                ds[i + t.len()] += ds[i];
            }
        }
    }
    ds[d.len()]
}

fn count_memo<'a>(d: &'a str, cache: &mut FxHashMap<&'a str, i64>, ts: &[&str]) -> i64 {
    if d.is_empty() {
        return 1;
    }
    if let Some(c) = cache.get(&d) {
        return *c;
    }

    let mut total: i64 = 0;
    for t in ts {
        if d.starts_with(t) {
            let ds = count_memo(&d[t.len()..], cache, ts);
            *cache.entry(&d[t.len()..]).or_default() = ds;
            total += ds;
        }
    }
    total
}

fn part1_memo(src: &str) -> Result<i64, Error> {
    let (towels, designs) = parse_input(src);
    let mut total = 0;
    let mut cache: FxHashMap<&str, i64> = FxHashMap::default();

    for d in designs {
        if count_memo(d, &mut cache, &towels) > 0 {
            total += 1;
        }
    }

    Ok(total)
}

fn part1(src: &str) -> Result<i64, Error> {
    let (towels, designs) = parse_input(src);
    let mut total = 0;

    for d in designs {
        if count(d, &towels) > 0 {
            total += 1;
        }
    }

    Ok(total)
}

fn part2_memo(src: &str) -> Result<i64, Error> {
    let (towels, designs) = parse_input(src);
    let mut cache: FxHashMap<&str, i64> = FxHashMap::default();
    let mut total = 0;

    for d in designs {
        total += count_memo(d, &mut cache, &towels);
        //total += count_rc(&towels, d);
    }
    Ok(total)
}

fn part2(src: &str) -> Result<i64, Error> {
    let (towels, designs) = parse_input(src);
    let mut cache: FxHashMap<&str, i64> = FxHashMap::default();
    let mut total = 0;

    for d in designs {
        total += count(d, &towels);
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let src = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        assert_eq!(6, part1(src).unwrap());
    }

    #[test]
    fn test_part_1_1() {
        let src = "guwgbuug, bubbb, grw, wwrbbgug, wrruwb, wgw, wgubbrwu, rrgb, gbrr, gbbgb, rrr, grbrgrur, bgrrub, uubrbwwr, ubwug, rgggg, gwggb, rruwbbbw, uubr, gugwwru, rwru, bgg, rrbgug, brgrw, brwwgrg, ubrrb, wugb, ggrg, buu, bwu, urru, rguw, uuwu, wbr, wuw, rwubww, uwgggb, gbb, guubur, ruw, bwwgrbur, bbbrbbbw, wbrbg, rwgrw, bwgb, uwubwug, bur, bbuwgbug, uurw, rwwbrr, ggur, ugr, wwu, uwbwbu, brg, bw, urg, ubwgguu, wwbuwug, bbgbw, ubrgu, wub, rruur, ggg, rgwgr, wgrrg, wwbr, gbgbru, uuugru, ggbwrbw, rbbrg, bgwrb, rgrgubb, wwbug, wrrrwbwr, b, gurb, rurwg, wuwb, rrw, rbub, gwr, bwr, wuwuwbw, uugb, ugggu, gw, ugubbw, wurwwr, ubw, bbw, brugru, wuwuu, buruw, rwrru, wwurubr, gwu, ugrr, wuu, wgbgg, ugrw, bwgbwu, rwuu, uwbb, bgggr, wwgbu, uwgwwuwr, bwb, gwrwg, wbrb, bgbuug, gwwwgb, uwu, ggr, ubuguu, gbuurb, ugbuu, wrbgww, rrbugug, ruru, rruwr, guug, rwb, gbwurg, uuurgwwg, ruwrb, gbg, guur, gwb, gb, uwwbrb, gww, ggbwrg, grr, uubu, urbb, ugu, ggwb, buurw, bgr, rbw, uwr, urw, bgugrrg, rguuwb, wrggg, brw, urgbbw, wwrb, bgrw, ugug, brrbbb, wbwwuu, bbbu, bww, wrg, bwbgu, rgu, urrwr, gugwu, rrub, buw, ubb, rrgu, ubu, grrug, ubugbrb, uruub, gwwrg, ugg, wuuuurr, bubwwu, gbr, rwgurw, wrw, uruw, rbr, grgb, wwg, rr, ugrb, rgb, ggrw, wg, wrgwrw, rgwb, rbb, rgw, bwgw, gggg, ruu, grww, rgur, gwuu, rwwg, bwgwr, rg, bgrgu, bu, uru, ubg, gur, ubbur, rb, ggwuugw, gg, wruu, wbb, gwrrgr, bgb, bug, ruuub, ug, brr, uwg, urgub, rwu, wubr, bbbbur, bg, rub, wgu, bbwbbgw, uur, bbb, rwr, g, wbwb, urgrurgr, wgg, wbgug, ugbbb, bbr, uug, rgg, gguub, bgwg, rwurubgw, wwuwu, ubgr, bbu, bgww, uuu, bgugugg, wbbruwug, rwgg, uuurb, bwubw, bb, urrbb, grbbbb, buru, gbbg, wruuw, wwbwug, urwbrbwg, gwrb, rgug, bru, wrrbwrg, www, rur, gggw, wwgwww, grb, rgwg, wgur, wwrwrr, gru, grwug, bwuw, rru, wuruw, guw, uu, brrbgg, ggrr, ww, bwbbrw, ururgrwb, wu, rwbg, gbw, wgr, rgru, gbrg, wbub, ggb, brww, ggu, brwgu, bbwbrrg, gbwgrub, rgr, gubwrb, rbgg, rrbu, rbrw, guuuub, bwwwb, uwggww, rrg, wwwub, rrwwu, rbbr, ur, rubrurg, brb, rrwubww, rrwru, gbru, u, wr, ruwu, uruuggg, gu, uubwg, bggr, wurggw, uuwwurb, rurbuuu, wbu, gug, bbuw, urrubb, bugwrwr, urb, bggbb, ugrrrg, ugwwgg, rwg, wwrwbb, wbg, rrbrg, uwuw, rug, ugrwgbw, gwbgu, rgubbw, wbw, gwww, brruu, gwgw, uugbrub, gwwruwr, rbg, gwwwg, ru, grg, wwurg, uubbb, ugw, gruwr, uubg, gubw, wru, rwwwg, urrgwg, ugb, wbbbuug, urgrrur, ugwwguw, w, uww, urrwwu, bwgr, ugbbbwg, uub, urbww, uwwb, gubrub, uwrg, urr, gwru, uugg, uwb, uuwb, burbbu, wb, uurr, gubuu, uwuu, rbwu, wrr, wwrub, bbg, brgrb, ggw, rbu, wwbburur, wrggbwww, gbu, ugrbrr, ruurrwgu, grwrwb, uwruru, uuw, guu, gbur, ub, gbwwg, wguu, gr, ubuubr, uwgu, bub, rbwgruu, wbwrgrr, ururu, bwg, rbggg, gbubrw, wrb, buwbr, grrr, gwgbu, buug, rbrbrwg, bruurrb, brrw, ubbb, wwr, rw, bwbw, grrb, wrwbuu, ruuugru, wgwwgwg, wwb, urbw, rbrwbb, uwub, gbugg, gwbr, rwrrb, urwruuw, buuwg, rww, guburw, bggubg, wug, urggbr, rrwwg, bgw, gwg, ubrgwur, gub

    ubwwwrggbwwwburgrwbugggubwrgwwrwuwwgrbrgwuwwurwrggrbggubr";
        assert_eq!(1, part1(src).unwrap());
    }

    #[test]
    fn test_part_2() {
        let src = "";
        assert_eq!(1, part2(src).unwrap());
    }
}

//400
//327
