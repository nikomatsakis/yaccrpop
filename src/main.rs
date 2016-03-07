use bison::*;
use std::env;
use std::fs::File;
use std::io;
use std::io::Read;
use std::process;

mod bison;
mod yacc;

fn main() {
    if env::args().len() == 0 {
        println!("Usage: yaccrpop [file.y]");
        process::exit(1);
    }

    let input = env::args().skip(1).next().unwrap();
    let yacctext = read_file(&input).unwrap();
    let yacc = yacc::parse_Yacc(&yacctext).unwrap();

    println!("grammar;");

    let start_symbol = yacc.declarations.iter()
                                        .filter_map(|decl| match *decl {
                                            BisonDecl::Start(ident) => Some(ident),
                                            _ => None,
                                        })
                                        .next();

    let mut all_chars = vec![];

    for rule in &yacc.rules {
        let is_pub = start_symbol == Some(rule.nonterminal);
        println!("");
        println!("{}{}: () = {{", if is_pub {"pub "} else {""}, rule.nonterminal.text);

        for alternative in &rule.alternatives {
            let mut string = String::from("    ");
            for (index, symbol) in alternative.symbols.iter().enumerate() {
                if index > 0 {
                    string.push_str(" ");
                }
                match *symbol {
                    Symbol::Ident(ref i) => string.push_str(i.text),
                    Symbol::Character(i) => {
                        let ch = i.chars().skip(1).next().unwrap();
                        string.push('"');
                        string.push(ch);
                        string.push('"');
                        all_chars.push(ch);
                    }
                }
            }
            if alternative.symbols.is_empty() {
                string.push_str("() ");
            }
            string.push_str(",");
            println!("{}", string);
        }

        println!("}};");
    }

    all_chars.sort();
    all_chars.dedup();

    println!("extern {{");
    println!("  enum &'input str {{");
    let mut c = 0;
    for &ch in &all_chars {
        println!(r#"    "{}" => Token{},"#, ch, c);
        c += 1;
    }
    for declaration in &yacc.declarations {
        match *declaration {
            BisonDecl::Token(i) => {
                println!(r#"    {} => Token{},"#, i.text, c);
                c += 1;
            }
            BisonDecl::Start(_) => { }
        }
    }
    println!("  }}");
    println!("}}");
}

fn read_file(name: &str) -> io::Result<String> {
    let mut string = String::new();
    let mut f = try!(File::open(name));
    try!(f.read_to_string(&mut string));
    Ok(string)
}
