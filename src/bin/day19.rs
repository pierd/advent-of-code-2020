use std::{collections::HashMap, str::FromStr};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
enum Char {
    A,
    B,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Expr {
    Or(Vec<Expr>),
    Concat(Vec<Expr>),
    Const(Char),
    Ref(usize),
}

impl Expr {
    fn matched_len(&self, msg: &[Char], exprs: &HashMap<usize, Expr>) -> Option<usize> {
        match self {
            Expr::Or(sub_exprs) => sub_exprs
                .iter()
                .map(|expr| expr.matched_len(msg, exprs))
                .find(Option::is_some)
                .flatten(),
            Expr::Concat(sub_exprs) => {
                let mut len = 0;
                let mut msg = msg;
                for expr in sub_exprs {
                    if let Some(matched) = expr.matched_len(msg, exprs) {
                        len += matched;
                        msg = msg.split_at(matched).1;
                    } else {
                        return None;
                    }
                }
                Some(len)
            }
            Expr::Const(c) => {
                if msg.first().map_or(false, |msg_c| c == msg_c) {
                    Some(1)
                } else {
                    None
                }
            }
            Expr::Ref(idx) => exprs.get(idx).and_then(|expr| expr.matched_len(msg, exprs)),
        }
    }

    fn matched_consec_lens(
        con_exprs: &[Expr],
        msg: &[Char],
        exprs: &HashMap<usize, Expr>,
    ) -> Vec<usize> {
        let mut results = Vec::new();
        if let Some((first, rest)) = con_exprs.split_first() {
            for first_len in first.matched_lens(msg, exprs) {
                results.extend(
                    Expr::matched_consec_lens(rest, msg.split_at(first_len).1, exprs)
                        .iter()
                        .map(|rest_len| first_len + rest_len),
                );
            }
        } else {
            results.push(0);
        }
        results
    }

    fn matched_lens(&self, msg: &[Char], exprs: &HashMap<usize, Expr>) -> Vec<usize> {
        match self {
            Expr::Or(sub_exprs) => sub_exprs
                .iter()
                .flat_map(|expr| expr.matched_lens(msg, exprs))
                .collect(),
            Expr::Concat(sub_exprs) => Expr::matched_consec_lens(sub_exprs, msg, exprs),
            Expr::Const(c) => {
                if msg.first().map_or(false, |msg_c| c == msg_c) {
                    vec![1]
                } else {
                    vec![]
                }
            }
            Expr::Ref(idx) => exprs[idx].matched_lens(msg, exprs),
        }
    }
}

impl FromStr for Expr {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "\"a\"" => Ok(Expr::Const(Char::A)),
            "\"b\"" => Ok(Expr::Const(Char::B)),
            _ if s.contains('|') => Ok(Expr::Or(
                s.split('|')
                    .map(|sub| sub.trim().parse::<Expr>())
                    .collect::<Result<Vec<_>, _>>()?,
            )),
            _ => Ok(Expr::Concat(
                s.split(' ')
                    .map(|sub| sub.parse::<usize>().map(Expr::Ref))
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|_| ())?,
            )),
        }
    }
}

fn parse_input(input: &str) -> (HashMap<usize, Expr>, Vec<Vec<Char>>) {
    let mut exprs = HashMap::new();
    let mut lines = input.lines();

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let mut parts = line.split(':');
        let idx = parts.next().unwrap().parse::<usize>().unwrap();
        let rest = parts.next().unwrap().trim();
        let expr = rest.parse::<Expr>().expect("should parse");
        exprs.insert(idx, expr);
    }

    let messages = lines
        .map(|line| {
            line.chars()
                .map(|c| if c == 'a' { Char::A } else { Char::B })
                .collect()
        })
        .collect();
    (exprs, messages)
}

fn main() {
    let (mut exprs, messages) = parse_input(include_str!("../../inputs/day19.txt"));
    let matched_messages_count = messages
        .iter()
        .filter(|msg| {
            exprs[&0]
                .matched_len(msg, &exprs)
                .map_or(false, |len| len == msg.len())
        })
        .count();
    println!("Part 1: {matched_messages_count}");

    exprs.insert(
        8,
        Expr::Or(vec![
            Expr::Ref(42),
            Expr::Concat(vec![Expr::Ref(42), Expr::Ref(8)]),
        ]),
    );
    exprs.insert(
        11,
        Expr::Or(vec![
            Expr::Concat(vec![Expr::Ref(42), Expr::Ref(31)]),
            Expr::Concat(vec![Expr::Ref(42), Expr::Ref(11), Expr::Ref(31)]),
        ]),
    );
    let matched_messages_count = messages
        .iter()
        .filter(|msg| {
            exprs[&0]
                .matched_lens(msg, &exprs)
                .into_iter()
                .any(|len| len == msg.len())
        })
        .count();
    println!("Part 2: {matched_messages_count}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matched_lens_concat() {
        let mut exprs = HashMap::new();
        exprs.insert(
            0,
            Expr::Concat(vec![
                Expr::Const(Char::A),
                Expr::Const(Char::A),
                Expr::Const(Char::A),
            ]),
        );
        assert_eq!(exprs[&0].matched_lens(&[], &exprs), vec![]);
        assert_eq!(exprs[&0].matched_lens(&[Char::A], &exprs), vec![]);
        assert_eq!(exprs[&0].matched_lens(&[Char::A, Char::A], &exprs), vec![]);
        assert_eq!(
            exprs[&0].matched_lens(&[Char::A, Char::A, Char::A], &exprs),
            vec![3]
        );
        assert_eq!(
            exprs[&0].matched_lens(&[Char::A, Char::A, Char::A, Char::A], &exprs),
            vec![3]
        );
    }

    #[test]
    fn test_matched_lens_or() {
        let mut exprs = HashMap::new();
        exprs.insert(
            0,
            Expr::Or(vec![Expr::Const(Char::A), Expr::Const(Char::B)]),
        );
        assert_eq!(exprs[&0].matched_lens(&[Char::A], &exprs), vec![1]);
        assert_eq!(exprs[&0].matched_lens(&[Char::B], &exprs), vec![1]);
        assert_eq!(exprs[&0].matched_lens(&[Char::A, Char::A], &exprs), vec![1]);
    }
}
