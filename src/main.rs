use ansi_term;
use std::io::Write;

pub mod lexer;
pub mod shared;

fn main() -> std::io::Result<()> {
    println!("Welcome!");
    let mut buffer: String;
    loop {
        print!("{} ", ansi_term::Color::Green.bold().paint(">>>"));
        std::io::stdout().flush()?;
        buffer = format!("");
        std::io::stdin().read_line(&mut buffer)?;

        if buffer.trim() == "exit" {
            break;
        };

        let tokenized = match lexer::Lexer::from(buffer.trim().to_string()).tokenize() {
            Ok(t) => t,
            Err(e) => panic!("{:?}", e),
        };

        println!("{:?}", tokenized);
    }
    Ok(())
}
