extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // thread_rng() 実行スレッドに固有で、OSによりシード値を与えられる
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess. (1 to 100)");

        // * let mut: `mutable` variable (NOTE: default is `immutable`)
        // * String はサイズ可変で UTF-8 なテキスト片を表す
        // * ::new は String の関連関数 new を呼び出すことを表す (NOTE: 関連関数は他の言語におけるいわゆる静的メソッド)
        let mut guess = String::new();

        // * & で guess の参照を渡す。Rust では参照も default immutable であるため、mut を付与して可変にする必要がある。
        // * read_line は io::Result を返却する。io::Result は列挙型（enum）であり、Ok、Err のどちらとなる。
        //   expect() メソッドは、Result が Err のとき、プログラムをクラッシュし引数の文字列を表示して終了する。Ok の場合は、Ok が保持する返り値を取り出して返却する。
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line!");

        // guess はシャドーイング
        // :u32 を付与することで、parse() が成功した場合の戻り値は u32 型として推論される?
        // secret_number 変数も u32 型として推論される。
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        // match 式
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
