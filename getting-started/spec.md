# 一般的なプログラミングの概念

## 変数と可変性

```rust
let x = 5;
let mut y = 6;
```

* 変数はデフォルトでは `immutable`。
* `mut` キーワードをつけることで `mutable` にできる。
* 必要でない限りデフォルト `immutable` を尊重すべきだろうが、大きなデータ構造の場合、メモリ割当を節約するために `mut` にするといったチューニングをすることはあり得る。

### 変数と定数の違い

```rust
const MAX_POINTS: u32 = 100_000;
```

* 変数は常に不変で、グローバルスコープも含め色々なスコープで宣言できる。
* 型の注釈は必須。
* 関数呼び出しの結果や、実行時に評価される値をセットすることはできない。

### シャドーイング

```rust
let x = 5;
let x = x + 1;
let x = x * 2;

let spaces = "   ";
let spaces = spaces.len();
```

* n-1 番目の `x` は、n 番目の `x` に`覆い隠される` と表現される。
* 覆い隠された変数の型を変更することも可能。2 番目の `spaces` は数値型になる。


## データ型

```rust
let guess: u32 = "42".parse().expect("Not a number!"); 
```

* 型推論がある。
* 複数の型が推論可能な場合は、型を `:u32` のように注釈して指定する必要がある。（例えば上記プログラムで `:u32` を注釈しないと、どの型に parse するのかわからず、コンパイルエラーとなる）

### スカラー型

* 単独の値を表す。
* 整数、浮動小数点数、論理値、文字値がある

#### 整数型

* 符号付きの i8, i16, i32, i64、符号無しの u8, ... がある。
* isize, usize という型もある。これはコンピューターが 32bit ならば i32/u32、64bit ならば i64/u64 になる。
  * isize, usize を使う主な状況は何らかのコレクションにアクセスするとき。
* 2, 8, 16進数リテラルがある。10進数リテラルは `let x = 10_000;` のようにアンダースコア区切りできる
* u8 型だけバイトリテラルがある。`let x = b'A';` のように表現する。

一般的にどの整数型を使うべきか？

* 確信がもてないときは Rust の基準型（`i32`）に従っておけば良い。
* 64bit でもこれが普通最速になる。


#### 浮動小数点数型

* f32 と f64 型。
* Rust の基準型は f64。現代の CPU では、f32 とほぼ同スピードにも関わらず精度が高くなるため。

#### 文字型

* char 型
* ユニコードのスカラー値を表す。
* ダブルクォートする文字列に対し、char 型はシングルクォートで表されることに注意。

### 複合型

* 複数の値を一つの型にまとめた型。タブルと配列がある。

#### タブル型

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
```

* タブル内の値はすべて同じ型でなくて良い。

```rust
let tup = (500, 6.4, 1);
let (x, y, z) = tup;
```

* 上記の例は、タプルをパターンマッチして x, y, z の変数に「分配」している。

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);
let x = tup.0;
let y = tup.1;
let z = tup.2;
```

* 上記の例は、タプルのインデックスアクセス。

#### 配列型

```rust
let a = [1, 2, 3, 4, 5];
```

* 配列内の値はすべて同じ型。
* Rust の配列は固定長。一度宣言すると伸縮できない。
  * 伸縮するコレクションが必要なときは、標準ライブラリのベクタ型を使用できる。
  * どちらを使うべきか確信をもてないときは、おそらくベクタ型を使うべき。
  * "1月" 〜 "12月" のように、サイズが自明である場合は配列型を使うべき。
* 配列は、ヒープではなくスタックにメモリ確保したいときに有効。  

```rust
let a = [1, 2, 3, 4, 5];
let first = a[0];
let second = a[1];
```

* 添字で配列内の値にアクセスできる。
* 無効な添字にアクセスするコードは実行時エラーでパニックになる。


## 関数

```rust
fn main() {
    another_function(5);
}

// 引数の型は宣言しなければならない
fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
```

### 関数本体は、文と式を含む

* 文: なんらかの動作をして値を<b>返さない</b>命令
* 式: 結果が値に評価される。式は<b>文の一部</b>になり得る

```rust
fn main() {
    // 文
    let x = 5;

    // 文: let y = { ... };
    // 式: { ... }（ブロック。セミコロン覗いた部分。セミコロンをつけると文になる）
    let y = {
        let x = 3;
        // 式で値を返したいのでセミコロン無し
        x + 1
    };    

    // 式: マクロ呼び出し（セミコロン覗いた部分）
    println!("The value of y is: {}", y);

    // 式: 関数呼び出し（セミコロン覗いた部分）
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}
```

### 戻り値のある関数

```rust
fn main() {
    println!("{}", five());
}

// 戻り値有り
fn five() -> i32 {
    // 式
    5
}
```


## コメント

```rust
fn main() {
    // comment
    // 複数行
    println!("hello"); // comment
}
```

ドキュメントコメントというものもある。

```rust
//! ここに包括的なコメントを書く

/// # MY_CONST
/// マークダウンが記述できる。
/// * foo
/// * bar
///
/// ## コードもかける
/// ```rust
/// pub const MY_CONST:u32 = 100;
/// ```
pub const MY_CONST:u32 = 100;
```

これを `cargo doc` するとドキュメントが出力される。

![image](https://user-images.githubusercontent.com/1415655/97228018-a962b700-1819-11eb-89e6-a12055868c1b.png)


## フロー制御

### if 式

```rust
fn main() {
    let number = 6;

    if number % 4 == 0 {
        // 数値は4で割り切れます
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        // 数値は3で割り切れます
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        // 数値は2で割り切れます
        println!("number is divisible by 2");
    } else {
        // 数値は4、3、2で割り切れません
        println!("number is not divisible by 4, 3, or 2");
    }
}
```

### ループ
#### loop

```rust
fn main() {
    loop {
        println!("again!");   // また
    }
}
```

#### while

```rust
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    // 発射！
    println!("LIFTOFF!!!");
}
```

#### for

コレクションを繰り返すのに使う

```rust
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        // 値は{}です
        println!("the value is: {}", element);
    }
}
```

```rust
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
```
