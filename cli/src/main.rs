//! CLI testing harness for hip-key
//!
//! Supports Telex and VNI input methods

use std::env;
use std::io::{self, Write};
use hip_key_lang_vi::{Vietnamese, InputMethod};

fn print_help(method: InputMethod) {
    println!("hip-key CLI Testing Harness");
    println!("============================");
    println!();
    if method == InputMethod::Telex {
        println!("Input method: Telex");
        println!("  aw  → ă    aa  → â    ow  → ơ    oo  → ô");
        println!("  uw  → ư    dd  → đ    ee  → ê");
        println!("  as  → á    af  → à    aj  → ả    ar  → ạ");
        println!("  ax  → a (remove tone)");
    } else {
        println!("Input method: VNI");
        println!("  a8  → ă    a6  → â    o7  → ơ    o6  → ô");
        println!("  u7  → ư    d9  → đ    e6  → ê");
        println!("  a1  → á    a2  → à    a3  → ả    a4  → ã    a5  → ạ");
    }
    println!();
    println!("Commands:");
    println!("  q  → quit");
    println!("  m  → switch input method (Telex/VNI)");
    println!();
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // Check for method in command line args
    let mut method = if args.len() > 1 && args[1] == "vni" {
        InputMethod::VNI
    } else {
        InputMethod::Telex
    };

    print_help(method);

    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        let method_name = if method == InputMethod::Telex { "Telex" } else { "VNI" };
        print!("[{}] > ", method_name);
        stdout.flush().expect("Failed to flush stdout");

        let mut input = String::new();
        stdin.read_line(&mut input).expect("Failed to read line");
        let input = input.trim();

        if input == "q" {
            println!("Bye!");
            break;
        }

        if input == "m" {
            method = if method == InputMethod::Telex {
                InputMethod::VNI
            } else {
                InputMethod::Telex
            };
            println!();
            print_help(method);
            continue;
        }

        if input.is_empty() {
            continue;
        }

        // Convert based on current method
        let vi = Vietnamese::with_method(method);
        let result = if method == InputMethod::Telex {
            vi.convert_telex(input)
        } else {
            vi.convert_vni(input)
        };
        println!("   → {}\n", result);
    }
}
