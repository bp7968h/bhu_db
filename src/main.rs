use std::{io::{self, Write}, process};


fn main() {
  println!(r"
  .______****__****__***__****__******_______**.______***
  |***_**\**|**|**|**|*|**|**|**|****|*******\*|***_**\**
  |**|_)**|*|**|__|**|*|**|**|**|****|**.--.**||**|_)**|*
  |***_**<**|***__***|*|**|**|**|****|**|**|**||***_**<**
  |**|_)**|*|**|**|**|*|**`--'**|****|**'--'**||**|_)**|*
  |______/**|__|**|__|**\______/*****|_______/*|______/**
  *******************************************************

  		Welcome to bhu db!
		Type 'exit' to quit.
  ");
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
        println!("Exiting bhu_db....");
        process::exit(0);
      },
      _ => {
         continue
       }
    }
  }
}
