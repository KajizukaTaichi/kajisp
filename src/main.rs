use std::{
    io::{self, Write},
    process::exit,
};

fn main() {
    println!("Kajisp - simple Lisp dialects\n(c) 2024 梶塚太智. All rights reserved");
    loop {
        let mut code = String::new();
        loop {
            let enter = input("> ");
            code += &format!("{enter}\n");
            if enter.is_empty() {
                break;
            }
        }
        if !code.trim().is_empty() {
            let program = parse(code);
            println!("{}", execute(program).get_symbol().trim());
        }
    }
}

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut result = String::new();
    io::stdin().read_line(&mut result).ok();
    result.trim().to_string()
}

#[derive(Clone, Debug)]
enum SExpression {
    Number(f64),
    Bool(bool),
    Symbol(String),
    String(String),
    List(Vec<SExpression>),
    Nil,
}

impl SExpression {
    fn get_number(&self) -> f64 {
        match self {
            SExpression::Number(i) => *i,
            SExpression::Bool(b) => {
                if *b {
                    1f64
                } else {
                    0f64
                }
            }
            SExpression::Symbol(s) => s.parse().unwrap_or(0f64),
            SExpression::String(s) => s.parse().unwrap_or(0f64),
            SExpression::List(l) => l.len() as f64,
            SExpression::Nil => 0f64,
        }
    }

    fn get_symbol(&self) -> String {
        match self {
            SExpression::Number(i) => i.to_string(),
            SExpression::Bool(b) => b.to_string(),
            SExpression::Symbol(s) => s.to_string(),
            SExpression::String(s) => format!("\"{s}\""),
            SExpression::List(l) => format!(
                "({})",
                l.iter()
                    .map(|x| x.get_symbol())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            SExpression::Nil => "nil".to_string(),
        }
    }

    fn get_string(&self) -> String {
        match self {
            SExpression::Number(i) => i.to_string(),
            SExpression::Bool(b) => b.to_string(),
            SExpression::Symbol(s) => s.to_string(),
            SExpression::String(s) => s.to_string(),
            SExpression::List(l) => format!(
                "({})",
                l.iter()
                    .map(|x| x.get_symbol())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            SExpression::Nil => String::new(),
        }
    }

    fn get_bool(&self) -> bool {
        match self {
            SExpression::Number(i) => *i != 0f64,
            SExpression::Bool(b) => *b,
            SExpression::Symbol(s) => !s.is_empty(),
            SExpression::String(s) => !s.is_empty(),
            SExpression::List(l) => !l.is_empty(),
            SExpression::Nil => false,
        }
    }

    fn get_list(&self) -> Vec<SExpression> {
        match self {
            SExpression::Number(i) => vec![SExpression::Number(*i)],
            SExpression::Bool(b) => vec![SExpression::Bool(*b)],
            SExpression::Symbol(s) => vec![SExpression::Symbol(s.clone())],
            SExpression::String(s) => vec![SExpression::String(s.clone())],
            SExpression::List(l) => l.clone(),
            SExpression::Nil => vec![],
        }
    }
}

fn execute(program: SExpression) -> SExpression {
    if let SExpression::List(list) = program {
        let list: Vec<SExpression> = list.iter().map(|i| execute(i.clone())).collect();
        let function: SExpression = if let Some(i) = list.get(0) {
            i.clone()
        } else {
            return SExpression::Nil;
        };
        let params: Vec<SExpression> = list[1..list.len()].to_vec();

        match function.get_symbol().as_str() {
            "+" => SExpression::Number({
                let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();

                let mut result: f64 = *params.get(0).expect("The paramater is deficiency");
                for i in params[1..params.len()].to_vec().iter() {
                    result += i;
                }
                result
            }),
            "-" => SExpression::Number({
                let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();

                let mut result: f64 = *params.get(0).expect("The paramater is deficiency");
                for i in params[1..params.len()].to_vec().iter() {
                    result -= i;
                }
                result
            }),
            "*" => SExpression::Number({
                let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();

                let mut result: f64 = *params.get(0).expect("The paramater is deficiency");
                for i in params[1..params.len()].to_vec().iter() {
                    result *= i;
                }
                result
            }),
            "/" => SExpression::Number({
                let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();

                let mut result: f64 = *params.get(0).expect("The paramater is deficiency");
                for i in params[1..params.len()].to_vec().iter() {
                    result /= i;
                }
                result
            }),
            "%" => SExpression::Number({
                let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();

                let mut result: f64 = *params.get(0).expect("The paramater is deficiency");
                for i in params[1..params.len()].to_vec().iter() {
                    result %= i;
                }
                result
            }),
            "=" => SExpression::Bool({
                let params: Vec<String> = params.iter().map(|i| i.get_symbol()).collect();

                match params.first() {
                    Some(first) => params.iter().all(|x| x == first),
                    None => true, // ベクタが空の場合はtrueとする
                }
            }),
            ">" => SExpression::Bool({
                let params: Vec<String> = params.iter().map(|i| i.get_symbol()).collect();
                params.windows(2).all(|window| window[0] > window[1])
            }),
            ">=" => SExpression::Bool({
                let params: Vec<String> = params.iter().map(|i| i.get_symbol()).collect();
                params.windows(2).all(|window| window[0] >= window[1])
            }),
            "<" => SExpression::Bool({
                let params: Vec<String> = params.iter().map(|i| i.get_symbol()).collect();
                params.windows(2).all(|window| window[0] < window[1])
            }),
            "<=" => SExpression::Bool({
                let params: Vec<String> = params.iter().map(|i| i.get_symbol()).collect();
                params.windows(2).all(|window| window[0] <= window[1])
            }),
            "&" => SExpression::Bool({
                let params: Vec<bool> = params.iter().map(|i| i.get_bool()).collect();
                params.iter().all(|&x| x)
            }),
            "|" => SExpression::Bool({
                let params: Vec<bool> = params.iter().map(|i| i.get_bool()).collect();
                params.iter().any(|&x| x)
            }),
            "!" => SExpression::Bool({
                !params
                    .get(0)
                    .expect("The paramater is deficiency")
                    .get_bool()
            }),
            "concat" => SExpression::String(params.iter().map(|i| i.get_string()).collect()),
            "println" => {
                println!(
                    "{}",
                    params
                        .get(0)
                        .expect("The paramater is deficiency")
                        .get_string()
                );
                SExpression::Nil
            }
            "print" => {
                print!(
                    "{}",
                    params
                        .get(0)
                        .expect("The paramater is deficiency")
                        .get_string()
                );
                SExpression::Nil
            }
            "input" => SExpression::String(input(
                &params
                    .get(0)
                    .expect("The paramater is deficiency")
                    .get_string(),
            )),
            "eval" => execute({
                let params = params
                    .get(0)
                    .expect("The paramater is deficiency")
                    .get_list();
                SExpression::List(params[1..params.len()].to_vec())
            }),
            "if" => {
                let condition = params
                    .get(0)
                    .expect("The paramater is deficiency")
                    .get_bool();
                let params = if condition {
                    params.get(1)
                } else {
                    params.get(2)
                }
                .expect("The paramater is deficiency");
                if let SExpression::List(list) = params {
                    execute(SExpression::List(list[1..list.len()].to_vec()))
                } else {
                    params.clone()
                }
            }
            "symbol" => SExpression::List(
                [vec![SExpression::Symbol("symbol".to_string())], params].concat(),
            ),
            "exit" => exit(0),
            _ => panic!("This function is undefined"),
        }
    } else {
        return program;
    }
}

fn parse(source: String) -> SExpression {
    let chars: Vec<char> = source.trim().chars().collect();
    if chars[0] == '(' && chars[chars.len() - 1] == ')' {
        let inner_list = String::from_iter(chars[1..chars.len() - 1].iter());
        SExpression::List(
            tokenize(inner_list)
                .iter()
                .map(|x| parse(x.clone()))
                .collect::<Vec<SExpression>>(),
        )
    } else if chars[0] == '"' && chars[chars.len() - 1] == '"' {
        let inner_string: String = String::from_iter(chars[1..chars.len() - 1].iter());
        SExpression::String(inner_string)
    } else {
        if let Ok(i) = source.parse::<f64>() {
            SExpression::Number(i)
        } else if let Ok(b) = source.parse::<bool>() {
            SExpression::Bool(b)
        } else if source == "nil" {
            SExpression::Nil
        } else {
            SExpression::Symbol(source)
        }
    }
}

fn tokenize(input: String) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut in_parentheses: usize = 0;
    let mut in_quote = false;

    for c in input.chars() {
        match c {
            '(' if !in_quote => {
                if in_parentheses != 0 {
                    in_parentheses += 1;
                    current_token.push(c);
                } else {
                    if !current_token.is_empty() {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                    in_parentheses += 1;
                    current_token.push(c);
                }
            }
            ')' if !in_quote => {
                if in_parentheses != 0 {
                    current_token.push(c);
                    in_parentheses -= 1;
                    if in_parentheses == 0 {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                } else {
                    panic!("Syntax error: invalid end of parentheses")
                }
            }
            '"' => {
                if in_parentheses == 0 {
                    if in_quote {
                        current_token.push(c);
                        in_quote = false;
                        tokens.push(current_token.clone());
                        current_token.clear();
                    } else {
                        in_quote = true;
                        current_token.push(c);
                    }
                } else {
                    current_token.push(c);
                }
            }
            ' ' | '\n' | '\t' | '\r' | '　' => {
                if in_parentheses != 0 || in_quote {
                    current_token.push(c);
                } else {
                    if !current_token.is_empty() {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                }
            }
            _ => {
                current_token.push(c);
            }
        }
    }

    if in_parentheses != 0 {
        panic!("Syntax error: There isn't end of parentheses");
    }
    if in_quote {
        panic!("Syntax error: There isn't end of quote");
    }

    if !current_token.is_empty() {
        tokens.push(current_token);
    }

    tokens
}
