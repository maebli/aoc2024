#[derive(Debug)]
enum Token {
    Number(usize),
    Multiply,
}

fn main() {
    let input = include_str!("input.txt");

    let chars = input.chars();

    let mut tokens = Vec::new();

    let mut chars = chars.into_iter().peekable();

    let mut enabled = true;
    while let Some(c) = chars.next() {
        match c {
            'm' => {
                if !enabled {
                    continue;
                }
                if let Some('u') = chars.next() {
                    if let Some('l') = chars.next() {
                        if let Some('(') = chars.next() {
                            tokens.push(Token::Multiply);
                            let mut number = 0;
                            while chars.peek().unwrap().is_digit(10) {
                                if let Some(c) = chars.next() {
                                    if c.is_digit(10) {
                                        number = number * 10 + c.to_digit(10).unwrap() as usize;
                                    }
                                }
                            }
                            if chars.next() != Some(',') {
                                tokens.pop();
                                continue;
                            }
                            tokens.push(Token::Number(number));

                            let mut number = 0;
                            while chars.peek().unwrap().is_digit(10) {
                                if let Some(c) = chars.next() {
                                    if c.is_digit(10) {
                                        number = number * 10 + c.to_digit(10).unwrap() as usize;
                                    }
                                }
                            }
                            if chars.next() != Some(')') {
                                tokens.pop();
                                tokens.pop();
                                continue;
                            }
                            tokens.push(Token::Number(number));
                        }
                    }
                }
            }
            'd' => {
                if enabled
                    && chars.next() == Some('o')
                    && chars.next() == Some('n')
                    && chars.next() == Some('\'')
                    && chars.next() == Some('t')
                    && chars.next() == Some('(')
                    && chars.next() == Some(')')
                {
                    enabled = false;
                } else if !enabled
                    && chars.next() == Some('o')
                    && chars.next() == Some('(')
                    && chars.next() == Some(')')
                {
                    enabled = true;
                }
            }
            _ => (),
        }
    }

    let x = tokens.windows(3).fold(0, |acc, x| match x {
        [Token::Multiply, Token::Number(a), Token::Number(b)] => acc + a * b,
        _ => acc,
    });

    println!("Result: {:?}", x);
}
