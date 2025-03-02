use std::io::{self, Write};
use std::process::Command;

fn print_logo() {
    println!(r#"
    ██╗  ██╗███████╗██╗     ██╗      ██████╗ ██████╗ ███╗   ███╗
    ██║  ██║██╔════╝██║     ██║     ██╔════╝██╔═══██╗████╗ ████║
    ███████║█████╗  ██║     ██║     ██║     ██║   ██║██╔████╔██║
    ██╔══██║██╔══╝  ██║     ██║     ██║     ██║   ██║██║╚██╔╝██║
    ██║  ██║███████╗███████╗███████╗╚██████╗╚██████╔╝██║ ╚═╝ ██║
    ╚═╝  ╚═╝╚══════╝╚══════╝╚══════╝ ╚═════╝ ╚═════╝ ╚═╝     ╚═╝
    "#);
}

    
fn main() {
    print_logo();
    loop {
        print!("ZorpSh> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() {
            continue;
        } else if input == "exit" {
            println!("Goodbye, Zorp!");
            break;
        }

        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap();
        let args: Vec<&str> = parts.collect();

        let status = Command::new(command)
            .args(&args)
            .spawn()
            .and_then(|mut child| child.wait());

        match status {
            Ok(status) => println!("Process exited with: {}", status),
            Err(e) => println!("Zorp error: {}", e),
        }
    }
}
