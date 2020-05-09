use std::io;
use console::style;

pub fn select_languages() -> String {
    let search_lang = loop {
        let mut selected_lang = String::new();
        io::stdin().read_line(&mut selected_lang).expect("cannot read line");
    
        let num: usize = match selected_lang.trim().parse() {
            Ok(n) => {
                if n <= 0 || n > LANG.len()  {
                    println!("{} select the number 1 ~ {}", style("info:").red(), LANG.len());
                    continue;
                }

                n
            },
            Err(_) => {
                println!("{} select the correct number...", style("info:").red());
                continue;
            }
        };

        break LANG[num - 1].to_string();
    };

    search_lang
}

//if you add lang, check code --> n @ 0 ..= X => LANG[n as usize].to_string(),
//0 ..= X   X is LANG.len()-1
pub const LANG: [&'static str; 28] = [
    "C",
    "C++",
    "Rust",
    "Go",
    "Swift",
    "Kotlin",
    "Java",
    "C#",
    "Python",
    "R",
    "JavaScript",
    "TypeScript",
    "Ruby",
    "PHP",
    "Dart",
    "Clojure",
    "Elixir",
    "F#",
    "Nim",
    "Shell",
    "Common Lisp",
    "Perl",
    "SQL",
    "D",
    "COBOL",
    "Object-c",
    "HTML",
    "CSS"
];