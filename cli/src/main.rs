//! CLI testing harness for hip-key
//!
//! Direct Telex conversion testing

use std::io::{self, Write};
use hip_key_lang_vi::{Vietnamese, InputMethod};

fn print_help() {
    println!("hip-key CLI Testing Harness");
    println!("============================");
    println!();
    println!("Type Telex sequences, press Enter to convert:");
    println!("  aw  → ă    aa  → â    ow  → ơ    oo  → ô");
    println!("  uw  → ư    dd  → đ    ee  → ê");
    println!();
    println!("Commands:");
    println!("  q  → quit");
    println!();
}

fn main() {
    print_help();

    let vi = Vietnamese::with_method(InputMethod::Telex);

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("> ");
        stdout.flush().expect("Failed to flush stdout");

        let mut input = String::new();
        stdin.read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        if input == "q" {
            println!("Bye!");
            break;
        }

        if input.is_empty() {
            continue;
        }

        // Convert Telex sequence
        let result = vi.convert_telex(input);
        println!("   → {}\n", result);
    }
}
