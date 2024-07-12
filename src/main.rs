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
    Atom(Type),
    List(Vec<SExpression>),
}

#[derive(Clone, Debug)]
enum Type {
    Number(f64),
    Bool(bool),
    Symbol(String),
    String(String),
    Nil,
}

impl Type {
    fn get_number(&self) -> f64 {
        match self {
            Type::Number(i) => *i,
            Type::Bool(b) => {
                if *b {
                    1f64
                } else {
                    0f64
                }
            }
            Type::Symbol(s) => s.parse().unwrap_or(0f64),
            Type::String(s) => s.parse().unwrap_or(0f64),
            Type::Nil => 0f64,
        }
    }

    fn get_symbol(&self) -> String {
        match self {
            Type::Number(i) => i.to_string(),
            Type::Bool(b) => b.to_string(),
            Type::Symbol(s) => s.to_string(),
            Type::String(s) => format!("\"{s}\""),
            Type::Nil => "nil".to_string(),
        }
    }

    fn get_string(&self) -> String {
        match self {
            Type::Number(i) => i.to_string(),
            Type::Bool(b) => b.to_string(),
            Type::Symbol(s) => s.to_string(),
            Type::String(s) => s.to_string(),
            Type::Nil => String::new(),
        }
    }

    fn get_bool(&self) -> bool {
        match self {
            Type::Number(i) => *i != 0f64,
            Type::Bool(b) => *b,
            Type::Symbol(s) => !s.is_empty(),
            Type::String(s) => !s.is_empty(),
            Type::Nil => false,
        }
    }
}

fn execute(program: SExpression) -> Type {
    if let SExpression::Atom(value) = program {
        return value;
    } else if let SExpression::List(list) = program {
        let list: Vec<Type> = list.iter().map(|i| execute(i.clone())).collect();
        let function: Type = if let Some(i) = list.get(0) {
            i.clone()
        } else {
            return Type::Nil;
        };
        let params: Vec<Type> = list[1..list.len()].to_vec();

        match function.get_symbol().as_str() {
            "+" => Type::Number({
                let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();

                let mut result: f64 = *params.get(0).expect("The paramater is deficiency");
                for i in params[1..params.len()].to_vec().iter() {
                    result += i;
                }
                result
            }),
            "-" => Type::Number({
                let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();

                let mut result: f64 = *params.get(0).expect("The paramater is deficiency");
                for i in params[1..params.len()].to_vec().iter() {
                    result -= i;
                }
                result
            }),
            "*" => Type::Number({
                let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();

                let mut result: f64 = *params.get(0).expect("The paramater is deficiency");
                for i in params[1..params.len()].to_vec().iter() {
                    result *= i;
                }
                result
            }),
            "/" => Type::Number({
                let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();

                let mut result: f64 = *params.get(0).expect("The paramater is deficiency");
                for i in params[1..params.len()].to_vec().iter() {
                    result /= i;
                }
                result
            }),
            "%" => Type::Number({
                let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();

                let mut result: f64 = *params.get(0).expect("The paramater is deficiency");
                for i in params[1..params.len()].to_vec().iter() {
                    result %= i;
                }
                result
            }),
            "=" => Type::Bool({
                let params: Vec<String> = params.iter().map(|i| i.get_symbol()).collect();

                match params.first() {
                    Some(first) => params.iter().all(|x| x == first),
                    None => true, // ベクタが空の場合はtrueとする
                }
            }),
            ">" => Type::Bool({
                let params: Vec<String> = params.iter().map(|i| i.get_symbol()).collect();
                params.windows(2).all(|window| window[0] > window[1])
            }),
            ">=" => Type::Bool({
                let params: Vec<String> = params.iter().map(|i| i.get_symbol()).collect();
                params.windows(2).all(|window| window[0] >= window[1])
            }),
            "<" => Type::Bool({
                let params: Vec<String> = params.iter().map(|i| i.get_symbol()).collect();
                params.windows(2).all(|window| window[0] < window[1])
            }),
            "<=" => Type::Bool({
                let params: Vec<String> = params.iter().map(|i| i.get_symbol()).collect();
                params.windows(2).all(|window| window[0] <= window[1])
            }),
            "&" => Type::Bool({
                let params: Vec<bool> = params.iter().map(|i| i.get_bool()).collect();
                params.iter().all(|&x| x)
            }),
            "|" => Type::Bool({
                let params: Vec<bool> = params.iter().map(|i| i.get_bool()).collect();
                params.iter().any(|&x| x)
            }),
            "!" => Type::Bool({
                !params
                    .get(0)
                    .expect("The paramater is deficiency")
                    .get_bool()
            }),
            "concat" => Type::String(params.iter().map(|i| i.get_string()).collect()),
            "println" => {
                println!(
                    "{}",
                    params
                        .get(0)
                        .expect("The paramater is deficiency")
                        .get_string()
                );
                Type::Nil
            }
            "print" => {
                print!(
                    "{}",
                    params
                        .get(0)
                        .expect("The paramater is deficiency")
                        .get_string()
                );
                Type::Nil
            }
            "eval" => execute(parse(
                params
                    .get(0)
                    .expect("The paramater is deficiency")
                    .get_string(),
            )),
            "if" => {
                let condition = params
                    .get(0)
                    .expect("The paramater is deficiency")
                    .get_bool();
                if condition {
                    params.get(1).expect("The paramater is deficiency").clone()
                } else {
                    params.get(2).expect("The paramater is deficiency").clone()
                }
            }

            "exit" => exit(0),
            _ => panic!("This function is undefined"),
        }
    } else {
        return Type::Nil;
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
        SExpression::Atom(Type::String(inner_string))
    } else {
        SExpression::Atom(if let Ok(i) = source.parse::<f64>() {
            Type::Number(i)
        } else if let Ok(b) = source.parse::<bool>() {
            Type::Bool(b)
        } else if source == "nil" {
            Type::Nil
        } else {
            Type::Symbol(source)
        })
    }
}

fn tokenize(input: String) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current_token = String::new();
    let mut in_parentheses = false;
    let mut in_quote = false;

    for c in input.chars() {
        match c {
            '(' if !in_quote => {
                if in_parentheses {
                    current_token.push(c);
                } else {
                    if !current_token.is_empty() {
                        tokens.push(current_token.clone());
                        current_token.clear();
                    }
                    in_parentheses = true;
                    current_token.push(c);
                }
            }
            ')' if !in_quote => {
                if in_parentheses {
                    current_token.push(c);
                    in_parentheses = false;
                    tokens.push(current_token.clone());
                    current_token.clear();
                } else {
                    current_token.push(c);
                }
            }
            '"' if !in_parentheses => {
                if in_quote {
                    current_token.push(c);
                    in_quote = false;
                    tokens.push(current_token.clone());
                    current_token.clear();
                } else {
                    in_quote = true;
                    current_token.push(c);
                }
            }
            ' ' | '\n' | '\t' | '\r' | '　' => {
                if in_parentheses || in_quote {
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

    if !current_token.is_empty() {
        tokens.push(current_token);
    }

    tokens
}
