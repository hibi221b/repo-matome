use std::io;
use console::style;

pub fn select_lower_bound() -> String {
    let lower = loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("cannot read line");
    
        let num: u64 = match s.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("{} input number...", style("info:").red());
                continue;
            }
        };

        if num < 1 {
            println!("{} you need to input the number more than 1", style("info:").red());
            continue;
        }

        break num.to_string();
    };

    lower
}

pub fn select_uppder_bound() -> String {

    let upper = loop {
        let mut s = String::new();
        io::stdin().read_line(&mut s).expect("cannot read line");
    
        if s == "*\n".to_string() || s == "*\r\n".to_string() {
            break "*".to_string();
        } else {
            let num: u64 = match s.trim().parse() {
                Ok(n) => n,
                Err(_) => {
                    println!("{} input number or *", style("info:").red());
                    continue;
                }
            };
    
            if num < 1 {
                println!("{} you need to input the number more than 1 or *", style("info:").red());
                continue;
            }
    
            break num.to_string();
        }
    };

    upper
}