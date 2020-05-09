use std::io;

pub fn select_topic() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("cannot read line");
    
    s.trim().to_string()
}