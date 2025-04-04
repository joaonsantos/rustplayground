use std::{fmt, io};

use rep::args::parse_args;

#[derive(Clone, Copy)]
enum Color {
    RED = 31,
    GREEN = 32,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            c => write!(f, "{}", u32::from(*c)),
        }
    }
}

impl From<Color> for u32 {
    fn from(c: Color) -> u32 {
        match c {
            Color::RED => Color::RED as u32,
            Color::GREEN => Color::GREEN as u32,
        }
    }
}

fn colorize(s: &str, c: Color) -> String {
    format!("\x1b[{}m{}\x1b[0m", c, s)
}

fn join<T: AsRef<str>>(v: &[T], sep: &str) -> String {
    let mut result = String::new();
    for item in v {
        if !result.is_empty() {
            result.push_str(sep);
        }
        result.push_str(item.as_ref());
    }
    result
}

fn read_lines<T: io::BufRead>(r: T) -> Vec<String> {
    r.lines().map(|v| v.unwrap()).collect()
}

fn matching_lines<T: AsRef<str>>(l: &[T], pat: &str) -> Vec<u32> {
    l.iter()
        .enumerate()
        .filter_map(|(i, v)| match v.as_ref().contains(pat) {
            true => Some(i as u32),
            false => None,
        })
        .collect()
}

fn create_intervals(l: &[u32], context: (u32, u32)) -> Vec<(usize, usize)> {
    l.iter()
        .map(|i| {
            (
                i.saturating_sub(context.0) as usize,
                i.saturating_add(context.1) as usize,
            )
        })
        .collect()
}

fn merge_intervals(mut intervals: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    intervals.dedup_by(|a, b| {
        if b.1 < a.0 {
            false
        } else {
            b.1 = a.1;
            true
        }
    });
    intervals
}

fn print_intervals(l: Vec<String>, intervals: Vec<(usize, usize)>, pat: &str, number: bool) {
    let index = 0;
    let l_iter = l.iter();
    for (start, end) in intervals {
        let skip = start - index;
        let take = (end - start) + 1;
        for (i, j) in l_iter.as_ref().iter().enumerate().skip(skip).take(take) {
            let mut output = Vec::new();

            if number {
                let num_str = colorize(&i.to_string(), Color::GREEN) + ":";
                output.push(num_str);
            }

            let parts: Vec<&str> = j.split(&pat).collect();
            let parts = join(&parts, &colorize(&pat, Color::RED));
            output.push(parts);
            println!("{}", output.join(""));
        }
    }
}

fn main() {
    let args = parse_args();
    let lines = read_lines(io::stdin().lock());
    let matching_lines = matching_lines(&lines, &args.pattern);
    let intervals = create_intervals(&matching_lines, args.context);
    let intervals = merge_intervals(intervals);

    print_intervals(lines, intervals, &args.pattern, args.number);
}
