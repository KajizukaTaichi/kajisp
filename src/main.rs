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
    Symbol(String),
    String(String),
    Nil,
}

impl Type {
    fn get_number(&self) -> f64 {
        match self {
            Type::Number(i) => *i,
            Type::Symbol(s) => s.parse().unwrap_or(0f64),
            Type::String(s) => s.parse().unwrap_or(0f64),
            Type::Nil => 0f64,
        }
    }

    fn get_symbol(&self) -> String {
        match self {
            Type::Number(i) => i.to_string(),
            Type::Symbol(s) => s.to_string(),
            Type::String(s) => format!("\"{s}\""),
            Type::Nil => "nil".to_string(),
        }
    }

    fn get_string(&self) -> String {
        match self {
            Type::Number(i) => i.to_string(),
            Type::Symbol(s) => s.to_string(),
            Type::String(s) => s.to_string(),
            Type::Nil => String::new(),
        }
    }
}

fn execute(program: SExpression) -> Type {
    if let SExpression::Atom(value) = program {
        return value;
    } else if let SExpression::List(list) = program {
        let list: Vec<Type> = list.iter().map(|i| execute(i.clone())).collect();
        let command: Type = if let Some(i) = list.get(0) {
            i.clone()
        } else {
            return Type::Nil;
        };
        let params: Vec<Type> = list[1..list.len()].to_vec();

        match command.get_symbol().as_str() {
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

                let mut result: f64 = *params.get(1).expect("The paramater is deficiency");
                for i in params[1..params.len()].to_vec().iter() {
                    result -= i;
                }
                result
            }),
            "*" => Type::Number({
                let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();

                let mut result: f64 = *params.get(1).expect("The paramater is deficiency");
                for i in params[1..params.len()].to_vec().iter() {
                    result *= i;
                }
                result
            }),
            "/" => Type::Number({
                let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();

                let mut result: f64 = *params.get(1).expect("The paramater is deficiency");
                for i in params[1..params.len()].to_vec().iter() {
                    result /= i;
                }
                result
            }),
            "%" => Type::Number({
                let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();

                let mut result: f64 = *params.get(1).expect("The paramater is deficiency");
                for i in params[1..params.len()].to_vec().iter() {
                    result %= i;
                }
                result
            }),
            "concat" => Type::String(params.iter().map(|i| i.get_string()).collect()),
            "println" => {
                println!(
                    "{}",
                    params
                        .get(1)
                        .expect("The paramater is deficiency")
                        .get_string()
                );
                Type::Nil
            }
            "print" => {
                print!(
                    "{}",
                    params
                        .get(1)
                        .expect("The paramater is deficiency")
                        .get_string()
                );
                Type::Nil
            }
            "eval" => execute(parse(
                params
                    .get(1)
                    .expect("The paramater is deficiency")
                    .get_string(),
            )),
            "exit" => exit(0),
            _ => panic!("美大落ちチョビ髭「チクショーメー」"),
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
        let value = source.parse::<f64>();
        SExpression::Atom(if let Ok(i) = value {
            Type::Number(i)
        } else {
            if source == "nil" {
                Type::Nil
            } else {
                Type::Symbol(source)
            }
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
