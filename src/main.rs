use std::{fs::File, io::Read};

#[derive(Debug)]
enum Token {
    Keyword(Keywords),
    NumLit(i32),
    Simbulo(String),
    Igual,
    SepLinha,
    ParenAb,
    ParenFe,
}

#[derive(Debug)]
enum Keywords {
    Var,
    TipoInt,
    // Print,
}

fn tokenizar(input: &str) -> Vec<Token> {
    let mut vetFinal: Vec<Token> = vec![];
    let mut stringdim = input;
    while stringdim.len() > 0 {
        if stringdim.starts_with("var") {
            vetFinal.push(Token::Keyword(Keywords::Var));
            if let Some(x) = stringdim.strip_prefix("var") {
                stringdim = x
            }
            continue;
        }
        if stringdim.starts_with("int") {
            vetFinal.push(Token::Keyword(Keywords::TipoInt));
            if let Some(x) = stringdim.strip_prefix("int") {
                stringdim = x
            }
            continue;
        }
        // if stringdim.starts_with("print") {
        //     vetFinal.push(Token::Keyword(Keywords::Print));
        //     if let Some(x) = stringdim.strip_prefix("print") {
        //         stringdim = x
        //     }
        //     continue;
        // }
        if stringdim.starts_with("(") {
            vetFinal.push(Token::ParenAb);
            if let Some(x) = stringdim.strip_prefix("(") {
                stringdim = x
            }
            continue;
        }
        if stringdim.starts_with(")") {
            vetFinal.push(Token::ParenFe);
            if let Some(x) = stringdim.strip_prefix(")") {
                stringdim = x
            }
            continue;
        }
        if stringdim.starts_with(";") {
            vetFinal.push(Token::SepLinha);
            if let Some(x) = stringdim.strip_prefix(";") {
                stringdim = x
            }
            continue;
        }
        if stringdim.starts_with("=") {
            vetFinal.push(Token::Igual);
            if let Some(x) = stringdim.strip_prefix("=") {
                stringdim = x
            }
            continue;
        }
        if stringdim.starts_with(" ") || stringdim.starts_with("\n") || stringdim.starts_with("\r") {
            stringdim = stringdim.trim_start();
            continue;
        }
        if stringdim.trim_start_matches(char::is_numeric) != stringdim {
            let mut num: i32 = 0;
            while stringdim.trim_start_matches(char::is_numeric) != stringdim {
                num *= 10;
                num += stringdim.chars().nth(0).expect("erro pegandodp primeiro pro nuimero").to_string().parse::<i32>().unwrap();
                stringdim = &stringdim[1..];
            }
            vetFinal.push(Token::NumLit(num));
            continue;
        }
        if stringdim.trim_start_matches(char::is_alphabetic) != stringdim {
            let mut s = String::from("");
            while stringdim.trim_start_matches(char::is_alphabetic) != stringdim {
                s.push(stringdim.chars().nth(0).expect("erro pegando primeira letra"));
                stringdim = &stringdim[1..];
            }
            vetFinal.push(Token::Simbulo(s));
            continue;
        };
        println!("nao reconheceu resto do input: {:?}\ntimado: {:?}", &stringdim[0..], stringdim.trim_start_matches(char::is_numeric));
        stringdim = &stringdim[1..];
    }
    return vetFinal;
}

fn main() {
    let mut arq = File::open("ex.txt").expect("erro abrindo arquivo");
    let mut input = String::new();
    arq.read_to_string(&mut input).expect("erro lendo arquivo");

    println!("input:\n{}", input);

    let tokens = tokenizar(&input);

    println!("tokens:");
    for t in tokens {
        println!("| {:?}", t);
    }
}
