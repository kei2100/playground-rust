use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    hello();

    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();
}

fn hello() {
    // ! 付きは関数呼び出しではなくマクロ呼び出し
    println!("hello world!");
}
