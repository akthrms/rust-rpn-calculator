#[derive(Debug, PartialEq, PartialOrd)]
pub enum Expr {
    Num(i32),
    Add,
    Sub,
    Mul,
    Div,
}

// TODO: 戻り値の型は Result<T, E> の方がいい
pub fn parse(s: &str) -> Vec<Expr> {
    let words = s.split_ascii_whitespace().collect::<Vec<&str>>();

    let mut result = Vec::new();
    for word in words {
        match word {
            "+" => result.push(Expr::Add),
            "-" => result.push(Expr::Sub),
            "*" => result.push(Expr::Mul),
            "/" => result.push(Expr::Div),
            _ => {
                if let Ok(num) = word.parse::<i32>() {
                    result.push(Expr::Num(num))
                } else {
                    panic!()
                }
            }
        }
    }

    result
}

// TODO: 戻り値の型は Result<T, E> の方がいい
pub fn eval(expr: Vec<Expr>) -> i32 {
    let mut stack = Vec::new();
    for e in expr {
        match e {
            Expr::Add => {
                let r = stack.pop().unwrap();
                let l = stack.pop().unwrap();
                stack.push(l + r)
            }
            Expr::Sub => {
                let r = stack.pop().unwrap();
                let l = stack.pop().unwrap();
                stack.push(l - r)
            }
            Expr::Mul => {
                let r = stack.pop().unwrap();
                let l = stack.pop().unwrap();
                stack.push(l * r)
            }
            Expr::Div => {
                let r = stack.pop().unwrap();
                let l = stack.pop().unwrap();
                stack.push(l / r)
            }
            Expr::Num(num) => stack.push(num),
        }
    }

    if stack.len() != 1 {
        // 最終的にスタックの長さは1になる
        panic!()
    }

    stack.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::{eval, parse, Expr};

    #[test]
    fn test_parse() {
        assert_eq!(
            parse("1 2 3 + - * /"),
            vec![
                Expr::Num(1),
                Expr::Num(2),
                Expr::Num(3),
                Expr::Add,
                Expr::Sub,
                Expr::Mul,
                Expr::Div
            ]
        );
    }

    #[test]
    #[should_panic]
    fn test_parse_should_panic() {
        parse("a");
    }

    #[test]
    fn test_eval() {
        assert_eq!(eval(vec![Expr::Num(1), Expr::Num(2), Expr::Add]), 3);
        assert_eq!(eval(vec![Expr::Num(1), Expr::Num(2), Expr::Sub]), -1);
        assert_eq!(eval(vec![Expr::Num(1), Expr::Num(2), Expr::Mul]), 2);
        assert_eq!(eval(vec![Expr::Num(1), Expr::Num(2), Expr::Div]), 0);
    }

    #[test]
    #[should_panic]
    fn test_eval_should_panic() {
        eval(vec![Expr::Num(1), Expr::Num(2), Expr::Add, Expr::Num(3)]);
    }

    #[test]
    fn test_rpn() {
        assert_eq!(eval(parse("3 4 + 1 2 - *")), -7)
    }
}
