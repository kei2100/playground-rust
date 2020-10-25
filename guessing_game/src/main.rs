use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    // * let mut: `mutable` variable (NOTE: default is `immutable`)
    // * String はサイズ可変で UTF-8 なテキスト片を表す
    // * ::new は String の関連関数 new を呼び出すことを表す (NOTE: 関連関数は他の言語におけるいわゆる静的メソッド)
    let mut guess = String::new();

    // * & で guess の参照を渡す。Rust では参照も default immutable であるため、mut を付与して可変にする必要がある。
    // * read_line は io::Result を返却する。io::Result は列挙型（enum）であり、Ok、Err のどちらとなる。
    //   expect() メソッドは、Result が Err のとき、プログラムをクラッシュし引数の文字列を表示して終了する。Ok の場合は、Ok が保持する返り値を取り出して返却する。
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line!");

    println!("You guessed: {}", guess);
}
