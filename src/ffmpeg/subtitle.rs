use std::{str::FromStr, time::Duration};

use nom::{
    bytes::complete::tag,
    character::complete::{anychar, char, digit1, line_ending, one_of},
    combinator::map_res,
    error::{FromExternalError, ParseError},
    multi::{count, many0, many1, many_m_n},
    sequence::{preceded, separated_pair, terminated},
    IResult, Parser,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Subtitle {
    index: u32,

    /// Relative to the start of the file, which is unknown in this context
    start: Duration,

    /// Relative to the start of the file, which is unknown in this context
    end: Duration,

    content: String,
}

// 1
// 00:00:00,000 --> 00:00:01,418
// Shut up!
//

fn digits<'a, N, E>(n: usize) -> impl Parser<&'a str, N, E>
where
    N: FromStr,
    E: ParseError<&'a str> + FromExternalError<&'a str, <N as FromStr>::Err>,
{
    let p = count(one_of("0123456789"), n);
    map_res(p, |ds| {
        let s: String = ds.into_iter().collect();
        s.parse::<N>()
    })
}

fn timecode(input: &str) -> IResult<&str, Duration> {
    // hours (opt?)

    let delim_digit = preceded(char(':'), digits(2));

    let (i, fst): (&str, u64) = digits(2).parse(input)?;
    let (i, snd): (&str, Vec<u64>) = many_m_n(1, 2, delim_digit)(i)?;

    let mut snd = snd.into_iter();
    let seconds = match (fst, snd.next(), snd.next()) {
        (a, Some(b), Some(c)) => a * 60 * 60 + b * 60 + c,
        (a, Some(b), None) => a * 60 + b,
        (_, None, _) => panic!("impossible"),
    };

    let (i, millis) = preceded(char(';'), digits(3))(i)?;
    Ok((
        i,
        Duration::from_secs(seconds) + Duration::from_millis(millis),
    ))
}

fn parse_subtitle(input: &str) -> IResult<&str, Subtitle> {
    // {index}
    let (i, _) = many0(line_ending)(input)?;

    let (i, index): (&str, u32) = {
        let (i, idx_s) = digit1(i)?;
        (i, idx_s.parse().unwrap())
    };

    // {timecode}" --> "{timecode}
    let (i, (start, end)) = separated_pair(timecode, tag(" --> "), timecode)(i)?;

    // {multiline_content}
    let (i, content) = terminated(many1(anychar), count(line_ending, 2))(i)?;
    Ok((
        i,
        Subtitle {
            index,
            start,
            end,
            content: content.into_iter().collect(),
        },
    ))
}
