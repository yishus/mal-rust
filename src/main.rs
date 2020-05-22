use std::io::{self, Write};

fn main() {
    loop {
        rep();
    }
}

fn rep() {
    let str = read();
    let ast = eval(str);
    print(&ast);
}

fn read() -> String {
    print!("mal> ");

    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");

    input
}

fn eval(s: String) -> String {
    s
}

fn print(s: &String) {
    print!("{}", s);
}
