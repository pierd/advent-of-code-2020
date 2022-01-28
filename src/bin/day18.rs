use logos::Logos;

#[derive(Logos, Clone, Copy, Debug, PartialEq)]
enum Token {
    #[token("+")]
    Add,

    #[token("*")]
    Mul,

    #[token("(")]
    Open,

    #[token(")")]
    Close,

    #[regex("[0-9]+", |lex| lex.slice().parse())]
    Num(usize),

    #[error]
    #[regex(r"[ \t\n\f]+", logos::skip)]
    Error,
}

#[derive(Clone, Copy, Debug)]
enum Operation {
    Add,
    Mul,
}

fn eval_equal_priority(tokens: &[Token]) -> usize {
    let mut stack: Vec<(usize, Operation)> = Vec::new();
    let mut current = 0;
    let mut current_operation = Operation::Add;
    for token in tokens {
        match token {
            Token::Add => current_operation = Operation::Add,
            Token::Mul => current_operation = Operation::Mul,
            Token::Open => {
                stack.push((current, current_operation));
                current = 0;
                current_operation = Operation::Add;
            }
            Token::Close => {
                let (prev_current, prev_operation) = stack.pop().unwrap();
                current = match prev_operation {
                    Operation::Add => prev_current + current,
                    Operation::Mul => prev_current * current,
                }
            }
            Token::Num(n) => {
                current = match current_operation {
                    Operation::Add => current + n,
                    Operation::Mul => current * n,
                }
            }
            Token::Error => panic!("failed to parse"),
        }
    }
    assert!(stack.is_empty());
    current
}

fn eval_add_before_mul(tokens: &[Token]) -> usize {
    let mut stack: Vec<Token> = Vec::new();
    for token in tokens {
        let peek = stack.last();
        match (peek, token) {
            (_, Token::Close) => {
                if let Some(Token::Num(mut product)) = stack.pop() {
                    // process muls within parens
                    while stack.last() != Some(&Token::Open) {
                        assert_eq!(stack.pop(), Some(Token::Mul));
                        if let Some(Token::Num(n)) = stack.pop() {
                            product *= n;
                        } else {
                            panic!("it should have been a number");
                        }
                    }
                    assert_eq!(stack.pop(), Some(Token::Open));

                    // handle "+" before the open paren
                    if stack.last() == Some(&Token::Add) {
                        stack.pop(); // pop Add
                        if let Some(Token::Num(other)) = stack.pop() {
                            product += other;
                        } else {
                            panic!("there should have been another num on the stack");
                        }
                    }
                    stack.push(Token::Num(product));
                } else {
                    panic!("it should have been a number before close paren");
                }
            }
            (Some(Token::Add), Token::Num(n)) => {
                stack.pop(); // pop Add
                if let Some(Token::Num(other)) = stack.pop() {
                    stack.push(Token::Num(other + *n));
                } else {
                    panic!("there should have been another num on the stack");
                }
            }
            (_, Token::Add | Token::Mul | Token::Open | Token::Num(_)) => stack.push(*token),
            (_, Token::Error) => panic!("failed to parse"),
        }
    }
    // resolve pending Muls (stack should be Num Mul Num Mul ... Num)
    let mut result = 1;
    for (idx, token) in stack.iter().enumerate() {
        if idx % 2 == 0 {
            if let Token::Num(n) = *token {
                result *= n;
            } else {
                panic!("invalid stack");
            }
        } else {
            assert_eq!(*token, Token::Mul);
        }
    }
    result
}

fn main() {
    let tokens: Vec<Vec<Token>> = include_str!("../../inputs/day18.txt")
        .lines()
        .map(|line| Token::lexer(line).collect())
        .collect();
    println!(
        "Part 1: {}",
        tokens
            .iter()
            .map(|tokens_line| eval_equal_priority(tokens_line))
            .sum::<usize>()
    );
    println!(
        "Part 2: {}",
        tokens
            .iter()
            .map(|tokens_line| eval_add_before_mul(tokens_line))
            .sum::<usize>()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2_samples() {
        assert_eq!(
            eval_add_before_mul(&Token::lexer("1 + (2 * 3) + (4 * (5 + 6))").collect::<Vec<_>>()),
            51
        );
        assert_eq!(
            eval_add_before_mul(&Token::lexer("2 * 3 + (4 * 5)").collect::<Vec<_>>()),
            46
        );
        assert_eq!(
            eval_add_before_mul(&Token::lexer("5 + (8 * 3 + 9 + 3 * 4 * 3)").collect::<Vec<_>>()),
            1445
        );
        assert_eq!(
            eval_add_before_mul(
                &Token::lexer("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))").collect::<Vec<_>>()
            ),
            669060
        );
        assert_eq!(
            eval_add_before_mul(
                &Token::lexer("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2")
                    .collect::<Vec<_>>()
            ),
            23340
        );
    }
}
