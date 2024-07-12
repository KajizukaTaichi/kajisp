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
    Nil,
}

impl Type {
    fn get_number(&self) -> f64 {
        match self {
            Type::Number(i) => *i,
            Type::Symbol(s) => s.parse().unwrap_or(0f64),
            Type::Nil => 0f64,
        }
    }

    fn get_symbol(&self) -> String {
        match self {
            Type::Number(i) => i.to_string(),
            Type::Symbol(s) => s.to_string(),
            Type::Nil => "nil".to_string(),
        }
    }
}

fn execute(program: SExpression) -> Type {
    if let SExpression::Atom(value) = program {
        return value;
    } else if let SExpression::List(list) = program {
        let list: Vec<Type> = list.iter().map(|i| execute(i.clone())).collect();
        let command: Type = list
            .get(0)
            .expect("チノちゃん「うるさいですね...」")
            .clone();
        let params: Vec<Type> = list[1..list.len()].to_vec();

        match command.get_symbol().as_str() {
            "+" => Type::Number({
                let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();

                let mut sum: f64 = params[0];
                for i in params[1..params.len()].to_vec().iter() {
                    sum += i;
                }
                sum
            }),
            "-" => Type::Number({
                let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();

                let mut sum: f64 = params[0];
                for i in params[1..params.len()].to_vec().iter() {
                    sum -= i;
                }
                sum
            }),
            "*" => Type::Number({
                let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();

                let mut sum: f64 = params[0];
                for i in params[1..params.len()].to_vec().iter() {
                    sum *= i;
                }
                sum
            }),
            "/" => Type::Number({
                let params: Vec<f64> = params.iter().map(|i| i.get_number()).collect();

                let mut sum: f64 = params[0];
                for i in params[1..params.len()].to_vec().iter() {
                    sum /= i;
                }
                sum
            }),
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

    for c in input.chars() {
        match c {
            '(' => {
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
            ')' => {
                if in_parentheses {
                    current_token.push(c);
                    in_parentheses = false;
                    tokens.push(current_token.clone());
                    current_token.clear();
                } else {
                    current_token.push(c);
                }
            }
            ' ' | '\n' | '\t' | '\r' | '　' => {
                if in_parentheses {
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
