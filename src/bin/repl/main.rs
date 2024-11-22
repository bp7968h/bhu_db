use std::{io::{self, Write}, process};
use bhu_db::lexer::Lexer;

const WELCOME_MSG: &str = r#"
.______****__****__***__****__******_______**.______***
|***_**\**|**|**|**|*|**|**|**|****|*******\*|***_**\**
|**|_)**|*|**|__|**|*|**|**|**|****|**.--.**||**|_)**|*
|***_**<**|***__***|*|**|**|**|****|**|**|**||***_**<**
|**|_)**|*|**|**|**|*|**`--'**|****|**'--'**||**|_)**|*
|______/**|__|**|__|**\______/*****|_______/*|______/**
*******************************************************

        Welcome to bhu_db!
        Type 'exit' to quit.
"#;

fn main() {
    println!("{}", WELCOME_MSG);
    
    let mut stdout = io::stdout();
    let stdin = io::stdin();

    loop {
        print!("bhu_db> ");
        stdout.flush().unwrap();

        let mut input = String::new();
        let _ = stdin.read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "exit" => {
                println!("Exiting bhu_db...");
                process::exit(0);
            },
            query => {
                let mut lexer = Lexer::new(query);
                lexer.tokenize();
                let tokens = lexer.tokens();
                println!("{:?}", tokens);
                continue;
            }
        }
    }
}
