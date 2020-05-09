use std::io;
use console::style;

pub fn select_headless_mode() -> bool {
    let headless = loop {
        let mut selected_lang = String::new();
        io::stdin().read_line(&mut selected_lang).expect("cannot read line");
        
        let result = match selected_lang.as_str() {
            "\n" | "\r\n" => true, //hide chrome   headless = true
            "n\n" | "n\r\n" => false, //visuzlize chrome  headless = false
            _ => {
                println!("{} press Enter or n", style("info:").red());
                continue;
            }
        };

        break result;
    };

    headless
}