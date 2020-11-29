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


# 所有権を理解する

* Rust のもっともユニークな機能。
* 所有権のおかげでガベージコレクタなしに安全性担保を行うこことができる。

## 所有権とは

* すべてのプログラムは、実行中のコンピューターのメモリの使用方法を管理する必要がある。
* 一般的なメモリの管理方法は以下の二通り。
  * プログラマが、プログラム中に明示的にメモリの確保/解放命令を記述する。
  * ガーベジコレクタが、ランタイム中に自動的にメモリ解放をおこなう。
* Rust は、第三の選択肢として、<b>所有権</b> を通じてメモリ管理を行う。
  * コンパイラがコンパイル時にチェックする一定の規則とともに、所有権システムを通じて管理される。
  * 実行中に所有権機能により動作が遅くなることはない。

### スタックとヒープ

* Rust のような言語において、値がスタックに積まれるのかヒープに置かれるのかを理解することは重要。

#### スタック

* LIFO のデータ構造。
* 追加/取り出しの場所を探索する必要がないため、スタックのデータアクセスは高速。
* スタック上のデータは、全て既知の固定サイズでなければならない。

#### ヒープ

* コンパイル時にサイズがわからなかったり可変であるデータは、スタックの代わりにヒープに配置される。
* ヒープにデータを置くとき、OS はヒープ上の十分な大きさを持つメモリ領域を確保し、ポインタを返す。（ポインタはヒープのその領域へのアドレス。）
* スタックに値を積むことは、メモリ確保とは考えられない。
* ポインタ値は固定サイズなのでスタックに保管することができるが、実データが必要になったらポインタの指す領域を追いかける必要がある。

#### スタックとヒープの理解

* ヒープへのデータアクセスは、スタックと比較して、ポインタを追って目的の場所を探索しなければならない分低速。
  * 現代のプロセッサはメモリのあちこちを行き来すると低速になる。データが隔離されているヒープよりもスタックのほうが仕事を早くこなせる。
* 関数呼び出しを行うと、関数に渡された値と、関数のローカル変数がスタックにのる。関数の実行が終了すると、それらの値はスタックから取り除かれる。
  * 関数に渡された値は、ヒープのデータへのポインタ値である可能性がある。
* 所有権は、ヒープ管理における以下のような問題を解決する。
  * どの部分のコードがヒープを使用しているのか把握すること。
  * ヒープ上の重複データを最小化すること。
  * ヒープ上の未使用データを掃除すること。

### 所有権規則

* Rust の各値は、<b>所有者</b> と呼ばれる変数と対応している。
* いかなる時も所有者は一つ。
* 所有者がスコープから外れると、値は破棄される。

### 文字列リテラルと String 型

```rust
// 文字列リテラル
let s = "hello";

// String 型
let mut ss = String::from("hello");
ss.push_str(" world!");
```

* 文字列リテラル
  * 文字列リテラルは、値がプログラムにハードコードされる
  * 文字列リテラルは不変
  * 文字列リテラルは、中身はコンパイル時に判明しているのでプログラムバイナリにハードコードされる。このため文字列リテラルは高速で効率的になる。
* String 型
  * String 型は<b>可変にすることができる</b>
  * 可変で伸長可能なテキストをサポートするために、String 型の値は、コンパイル時に不明な量のメモリをヒープに確保して内容を保持する。
  * String 型では、メモリは実行時に OS に要求される。値を使用し終わったら、OS にメモリを返還する。

### メモリと確保

```rust
{
    let s = String::from("hello"); // s はここから有効になる
} // s のスコープの終了、メモリの返還
```

* Rust では、メモリを所有している変数がスコープを抜けたら、メモリは自動的に返還される。
* 変数がスコープを抜けるとき、Rust はその変数に対し `drop` と呼ばれる特別な関数を呼ぶ。ここで 変数型の書き手はリソースを解放するコードを配置することができる。

#### 変数とデータの相互作用: 所有権のムーブ

```rust
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y); // ともに 5 を出力する。
```

* 上記の例は、「値 5 を変数 x にバインドする。それから x の値をコピーして y にバインドする」となる。
* これら2つの「5」の値は、それぞれスタックに積まれる。

```rust 
let s1 = String::from("hello");
let s2 = s1;

    println!("s1 = {}, s2 = {}", s1, s2); // s1 は「ムーブされている」ため参照エラーとなる。
```

* 上記の例では、「s1 の値をコピーして s2 にバインドする」とはならない。
* 以下のようなイメージになる。

![image](https://user-images.githubusercontent.com/1415655/97935752-d9cbc780-1dbc-11eb-8b86-3f1cff7fa88e.png)

* String 型は以下の3つの部品でできている
  * ptr: 文字列の中身を保持するメモリへのポインタ
  * len: 文字列の長さ
  * capacity: 許容量
* s1 を s2 に代入すると、String 型の部品がコピーされる。ポインタが指すヒープ上のデータはコピーしない。
* s1 と s2 に対し、`drop` を呼び出してしまうとメモリ二重解放エラーになるため、Rust は s1 が最早必要でないことが分かると、それを無効化し、以後参照することはできなくなる。
* これを「s1 の所有権は s2 に<b>ムーブ</b>された」と表現する。
* ヒープに確保された値への変数を他の変数に代入するときにはこのような挙動となるため、Rust では自動的にデータの deep copy が行われることはない。

#### 変数とデータの相互作用: クローン

* 本当にデータの deep copy が必要な場合には `clone` を呼び出す。

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);  // s1 はムーブされていないので、問題なし
```

#### スタックのみのデータ: コピー

```rust
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
```

* 整数型のようなコンパイル時に既知となるサイズを持つ型は、上記のような `let y = x;` の代入を行ってもムーブは発生しない。
* このような型には `Copy` トレイトと呼ばれる特殊な注釈を配置することができる。型が `Copy` トレイトに適合している場合、代入後も古い変数が使用可能となる。
* 一方、型やその一部分でも `Drop` トレイトを実装している場合、`Copy` トレイトの注釈を行うことはできない。（型の値がスコープを外れた時に `drop` 関数で何か特別なことを起こす必要がある場合に `Drop` トレイトを実装する）
* 単純なスカラー値の集合は `Copy` 。メモリ確保が必要だったり何らかの形態のリソースだったりするものは `Copy` ではない。
  * 整数型 / 論理値型 / 浮動小数点数型 / 文字型 (char)
  * すべて `Copy` な型からなるタプル。e.g. (i32, i32) は `Copy`、(i32, String) は違う。

### 所有権と関数

* 関数などで値を返すことでも所有権はムーブする。
* 別の変数に値を代入したり、関数で値を返すとムーブされる。
* ヒープにデータを含む変数がスコープを抜けると、それがムーブされていない限り、`drop` が呼び出される。

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownershipは、戻り値をs1に
                                        // ムーブする

    let s2 = String::from("hello");     // s2がスコープに入る

    let s3 = takes_and_gives_back(s2);  // s2はtakes_and_gives_backにムーブされ
                                        // 戻り値もs3にムーブされる
} // ここで、s3はスコープを抜け、ドロップされる。s2もスコープを抜けるが、ムーブされているので、
  // 何も起きない。s1もスコープを抜け、ドロップされる。

fn gives_ownership() -> String {             // gives_ownershipは、戻り値を
                                             // 呼び出した関数にムーブする

    let some_string = String::from("hello"); // some_stringがスコープに入る

    some_string                              // some_stringが返され、呼び出し元関数に
                                             // ムーブされる
}

// takes_and_gives_backは、Stringを一つ受け取り、返す。
fn takes_and_gives_back(a_string: String) -> String { // a_stringがスコープに入る。

    a_string  // a_stringが返され、呼び出し元関数にムーブされる
}
```

* しかし上記のように、所有権を取り、またその所有権を戻す、ということを全ての関数でしていたら、ちょっとめんどくさい。
* 関数に値を使わせるものの、所有権を取らせないようにするにはどうするか。Rust では「<b>参照</b>」機能でこの概念を実現する。

## 参照と借用

### 参照

```rust
fn main() {
    let s0 = String::from("hello");
    // calculate_length では、関数に s0 を渡した時点で所有権がムーブするため、s0 に代入された値を参照するには以下のようにしなければならない。
    let (s00, len) = calculate_length(s0);
    println!("The length of '{}' is {}.", s00, len);


    let s1 = String::from("world");
    // calculate_length_ref では、s3 の参照を渡している。
    let len = calculate_length_ref(&s1);
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_ref(s: &String) -> usize {
    s.len()
}
```

上記例における `calculate_length_ref` の `s` と `s1` の関係を図示すると以下のようになる。

![image](https://user-images.githubusercontent.com/1415655/98428317-49ef8b80-20e4-11eb-818c-f7967871eff7.png)

したがって、以下のコードは、ヒープに配置された `"hello"` という文字列値を参照する `s1` を生成し、`&s1` により、その `s1` を参照する変数を生成しているといえる。
（文字列値の<b>参照の参照</b>）

`&s1` の記法は `s1` 参照を生成するが、`s1` を所有することはない。所有していないということは、`&1` が指している値は、`&s1` がスコープを抜けてもドロップされることはない。

```rust
let s1 = String::from("hello");
let len = calculate_length(&s1);
```

関数のシグニチャも同様。

```rust
fn calculate_length_ref(s: &String) -> usize { // s は Stringへの参照
    s.len()
} // ここで s はスコープ外になるが、参照しているものの所有権を持っているわけではないので
  // s が指しているものをドロップすることはない。
  // 所有権をもらわないので、所有権を返す目的で値を返す必要もない。
```

### 借用

* 変数代入や関数の引数に、ある変数の参照を取ることを<b>借用</b>と呼ぶ。
* 借用したものの参照先が不変であある場合、それを変更することはできない。

```rust
// これは動かない！
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}

// error[E0596]: cannot borrow immutable borrowed content `*some_string` as mutable
// (エラー: 不変な借用をした中身`*some_string`を可変で借用できません)
//  --> error.rs:8:5
//   |
// 7 | fn change(some_string: &String) {
//   |                        ------- use `&mut String` here to make mutable
// 8 |     some_string.push_str(", world");
//   | 
```

### 可変な借用

* 可変な変数 `let mut s` に対し、`&mut s` で可変な借用をすることができる。
* 同時に複数の可変な借用を行うことはできない。
* 同時に複数の不変な借用を行うことはできる。
* 同時に不変な借用と可変な借用を行うことはできない。

```rust 
fn main() {
    let mut s = String::from("hello");
    change(&mut s); // 可変な参照
    println!("{}", s); // hello, world

    // {
    //     // 以下はコンパイルエラー
    //     // 同時に複数の可変な借用を行うことはできない。
    //     let s1 = &mut s;
    //     let s2 = &mut s;
    //     println!("s1:{} s2:{}", s1, s2);
    // }
    {
        // 以下は問題なし
        // 同時に複数の不変な借用を行うことはできる。
        let s1 = &s;
        let s2 = &s;
        println!("s1:{} s2:{}", s1, s2);
    }
    {
        // 以下はコンパイルエラー
        // 同時に不変な借用と可変な借用を行うことはできない。
        let s1 = &s;
        let s2 = &mut s;
        println!("s1:{} s2:{}", s1, s2);
    }
}

fn change(ss: &mut String) { // 可変な借用
    ss.push_str(", world");
}
```

### 宙に浮いた参照

* Rust ではコンパイラによりダングリングポインタの発生を防止している。
  * ダングリングポインタ: その箇所へのポインタがあるにも関わらず、メモリを解放してしまうことで発生するもの

以下のコードはコンパイルエラーとなる

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");  // s は新しい String
    &s // String s への参照を返す
} // ここで、s はスコープを抜けドロップされる。そのメモリは消される。危険だ！
```

```
error[E0106]: missing lifetime specifier
(エラー: ライフタイム指定子がありません)
 --> main.rs:5:16
  |
5 | fn dangle() -> &String {
  |                ^ expected lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but there is no
    value for it to be borrowed from
    (助言: この関数の戻り値型は、借用した値を含んでいますが、借用される値がどこにもありません)
  = help: consider giving it a 'static lifetime
  ('staticライフタイムを与えることを考慮してみてください)
```

`dangle` 関数の戻り値を `&s` から `s` にすれば、所有権が呼び出し元にムーブされるため、コンパイルエラーではなくなる。

```rust
fn dangle() -> String {
    let s = String::from("hello");  // s は新しい String
    s
} // ここで、s はスコープを抜けるが、所有権はムーブされるためドロップされない
```

## スライス型

スライス型とは。

* 所有権の無いデータ型。
* コレクションの中の一連の要素を「参照」することができる。

### 文字列スライス

文字列スライスの例。

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

* [開始..終点]: 開始の位置から始まり、**終点未満** の位置までの範囲を表す。
* 上記では、 `let world` は、文字列の 7 バイト目へのポインタと 5 の長さを持つスライスになる。
* 範囲の添字は、有効な UTF-8 文字境界に置かなければならない。マルチバイト文字の中途半端な位置を指すと、エラーでプログラムが落ちる。

![image](https://user-images.githubusercontent.com/1415655/99161349-ca416c80-2734-11eb-9c6d-fed48093b848.png)

範囲添字は以下のような記法がある。

```rust
let s = String::from("hello world");

let world = &s[0..5];
let world = &s[..5]; // 先頭から。上と等価。

let world = &s[6..11];
let world = &s[6..];  // 末尾まで。上と等価。

let hw = &s[..]; // 文字列全体
```

文字列スライスを活用することで、安全なプログラムを書くことができるようになる例を以下にしめす。

まず、危険な例。first_word という文字列における最初の単語（英単語）を抽出する関数。

```rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s); // word の中身は、値5になる

    s.clear(); // String を空にする。

    // word はまだ値5を保持しているが、もうこの値を有効に使用できる文字列は存在しない！
    println!("the first word is: {}", s[]);
}

// first_word は、文字列 s から最初の単語のインデックスを返却する
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
```

スライスを使うことで、このプログラムを安全にすることができる。
　
* *不変な借用をしている間は、可変な借用をすることはできない*

のが、ポイント。

```rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // エラー

    println!("the first word is: {}", word);
}

// first_word は、文字列 s から最初の単語までを参照する不変なスライスを返却する。
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

このプログラムはコンパイル時に問題が検出される。
不変な借用が有効な間に、`s.clear();` により可変な借用をしようとしているからである。

```
$ cargo run --color=always --package sandbox --bin sandbox
error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:5
  |
4 |     let word = first_word(&s);
  |                           -- immutable borrow occurs here
5 | 
6 |     s.clear(); // エラー
  |     ^^^^^^^^^ mutable borrow occurs here
7 | 
8 |     println!("the first word is: {}", word);
  |                                       ---- immutable borrow later used here

error: aborting due to previous error
```

### 文字列リテラルはスライス

スライスを理解すると、文字列リテラルは何者なのかが分かる。

```rust
let s = "Hello, world!";
```

この s の型は、&str である。つまり不変。

### 引数としての文字列スライス

先の `first_word` 関数は、

```rust
fn first_word(s: &String) -> &str {
```

から

```rust
fn first_word(s: &str) -> &str {
```

とすることで、String でも str でも渡すことができるようになる。

### 他のスライス

```
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];
```

`slice` は `&[i32]` という型になる。これも文字列スライスと同様に動作する。つまり最初の要素への参照と長さを保持する。


# 構造体を使用して関係のあるデータを構造化する

## 構造体を定義し、インスタンス化する

### 基本
```rs
// 構造体とフィールドの定義
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}


// 構造体のインスタンス生成
let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

// フィールドへの値の設定
// NOTE: 値を設定するときはインスタンス全体が可変でないといけない！一部のフィールドのみを可変にすることはできない。
user1.email = String::from("anotheremail@example.com");


// 関数の戻り値に構造体を使う
fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

// 上記の省略記法
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// 構造体更新記法
// * 更新記法を使わないバージョン
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    active: user1.active,
    sign_in_count: user1.sign_in_count,
};

// * 更新記法を使うバージョン
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};
```

### タプル構造体

名前がついたタプルのような、フィールド名を持たない構造体

```rs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);

// black と origin は異なる型であることに注意

// アクセス方法は通常のタプルと同様
let x = origin.0;
let y = origin.1;
let z = origin.2;

let (x, y, z) = origin;
```

### フィールドの無いユニット様（よう）構造体

* ユニット型（要素がゼロのタプル `()`）のように振る舞う構造体
* フィールドを持たない
* ある型にトレイトを実装するが、 型自体に保持させるデータは一切ない場面に有効


```rs
struct UnitStruct;
```

### 構造体データの所有権

* この段落の User 構造体の定義で、フィールドの型は &str のような文字列スライス型ではなく、所有権のある String 型を使用した
* この選択は意識的なもの。
* この構造体のインスタンスには全データを所有してもらう必要があり、インスタンスが有効な間はそのフィールドも有効でいてほしいため。

```rs
struct User {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: "someone@example.com",
        username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
}
```

上記のようなコードは、コンパイルエラーとなる。

```
error[E0106]: missing lifetime specifier
(エラー: ライフタイム指定子がありません)
 -->
  | 
2 |     username: &str,
  |               ^ expected lifetime parameter
                   (ライフタイム引数を予期しました)

error[E0106]: missing lifetime specifier
 -->
  | 
3 |     email: &str,
  |            ^ expected lifetime parameter
```

ライフタイムという機能を使えばこのようなフィールド型をもたせることも可能であるが、それは後述。

## 構造体を使ったプログラム例

```rs
fn main() {
    println!("{}", area_simple(2, 10));
    println!("{}", area_use_tuple((3, 10)));
    let rect = Rectangle { width: 4, height: 10 };
    println!("{}", area_use_struct(&rect))
}

// 単純な面積計算
fn area_simple(width: u32, height: u32) -> u32 {
    width * height
}

// タプルを使ってリファクタリング
fn area_use_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// 構造体を使ってさらにリファクタリング
fn area_use_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

struct Rectangle {
    width: u32,
    height: u32,
}
```

### トレイトの導出で有用な機能を追加する

```rs
#[derive(Debug)] // Debug トレイトを導出する注釈を追加する
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect = Rectangle { width: 4, height: 10 };

    // {:?} フォーマットは Debug トレイトを実装? する値の出力を行うことができる
    println!("{:?}", &rect)
    // 出力 -=> Rectangle { width: 4, height: 10 }
}
```

## メソッド記法

### メソッドを定義する

```rs
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // メソッド
    // * 最初の引数は必ず自身のインスタンスを表す self となる
    // * インスタンスの状態を変更したかったら &mut self となる
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle { width: 4, height: 10 };
    println!("{}", rect.area())
}
```

### メソッド呼び出し

* Rust のメソッド呼び出しは、そのメソッドの self の定義に応じて、& や &mut、* を自動的に付与する。
* たとえば、上段のメソッドを呼び出すとき、2行目の参照の付与を自動的に実行している。

```rs
1: rect.area();
2: (&rect).area();
```

### 関連関数

* self を取らない関数。メソッドではない。インスタンスではなく構造体に関連付けられる。
* 構造体の新規インスタンスを返すコンストラクタとしてよく使われる。`String::from` など。

```rs
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

// Rectangle::square(10) のように呼び出す
```

### 複数の impl ブロック

* 分けて書くことができる

```rs
mpl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

# Enum とパターンマッチング

## enum を定義する

TODO
