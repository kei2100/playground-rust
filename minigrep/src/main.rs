extern crate minigrep;

use minigrep::Config;
use std::{env, process};

// Rust コミュニティの main 関数ガイドライン
//
// * プログラムをmain.rsとlib.rsに分け、ロジックをlib.rsに移動する。
// * コマンドライン引数の解析ロジックが小規模な限り、main.rsに置いても良い。
// * コマンドライン引数の解析ロジックが複雑化の様相を呈し始めたら、main.rsから抽出してlib.rsに移動する。
//
// この工程の後にmain関数に残る責任は以下に限定される:
// * 引数の値でコマンドライン引数の解析ロジックを呼び出す
// * 他のあらゆる設定を行う
// * lib.rsのrun関数を呼び出す
// * runがエラーを返した時に処理する
fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
