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
```rust
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

```rust
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


```rust
struct UnitStruct;
```

### 構造体データの所有権

* この段落の User 構造体の定義で、フィールドの型は &str のような文字列スライス型ではなく、所有権のある String 型を使用した
* この選択は意識的なもの。
* この構造体のインスタンスには全データを所有してもらう必要があり、インスタンスが有効な間はそのフィールドも有効でいてほしいため。

```rust
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

```rust
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

```rust
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

```rust
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

```rust
1: rect.area();
2: (&rect).area();
```

### 関連関数

* self を取らない関数。メソッドではない。インスタンスではなく構造体に関連付けられる。
* 構造体の新規インスタンスを返すコンストラクタとしてよく使われる。`String::from` など。

```rust
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}

// Rectangle::square(10) のように呼び出す
```

### 複数の impl ブロック

* 分けて書くことができる

```rust
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

## Enum を定義する

```rust
enum IpAddrKind {
    V4,
    V6,
}

let four = IpAddrKind::V4;
let six = IpAddrKind::V6;


fn route(ip_type: IpAddrKind) {}
```

enum 値に型を関連付けることができる。

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));

let loopback = IpAddr::V6(String::from("::1"));
```

enum 値はそれぞれ異なる型でもよい。

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));
```

それぞれ異なる型でもよい。実際 std::net::IpAddr はそのような定義になっている。

```rust
struct Ipv4Addr {
    // 省略
}

struct Ipv6Addr {
    // 省略
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}
```

異なる型を定義できるので、いずれの型も受け取る関数を簡単に定義することができる。

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 }, // 匿名構造体
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn process_messsage(message: Message) {}
```

enum にメソッド定義することができる。

```rust
impl Message {
    fn call(&self) {
        // method body would be defined here
        // メソッド本体はここに定義される
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

### 標準ライブラリ enum Option

標準ライブラリの `enum Option` で、値がなにかかそうでないか、を型安全にコード化できる

例えば Rust には null が存在しないが、`enum Option` を使うことで、値が存在するか不在化という概念をコード化することができる。

```rust
enum Option<T> {
    Some(T),
    None,
}
```

`Option` は有益なため Rust の Prelude（初期化）にも含まれ、`Option::` の prefix なしで直接 Some(T) と None を使うことができる。 
`Option<T>` には様々なメソッドがあり、それらのを使って Some(T) が持つ値を取り出したりすることができる。

`let x: Option<i8> = Some(5);` のようにすると、`i8` の値を持つ Some 値を表すことができるが、通常の `let y: i8 = 10;` の型とは互換性が無く、`let z = x + y;` はコンパイルエラーとなる。
通常の型であれば値が必ずあることがコンパイラレベルで保障される。Option であれば値を保持していない可能性があり、それを考慮したプログラムを組む必要があることを明示することができるのである。

## match フロー制御演算子

* `match` 演算子によりパターンマッチできる。
* 全てのありうるパターンを処理しているかをコンパイラがチェックしてくれる

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Luckey penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

### パターンマッチと束縛

* パターンにマッチしたらその値を束縛することができる。


```rust
#[derive(Debug)] // すぐに州を点検できるように
enum UsState {
    Alabama,
    Alaska,
    // ... などなど
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            // value_in_cents(Coin::Quarter(UsState::Alaska)) と呼び出したら、
            // state には UsState::Alaska が束縛される
            println!("State quarter from {:?}!", state);
            25
        },
    }
}
```

### Option<T> とのマッチ

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

### _ プレースホルダー

* Rust のパターンマッチは包括的であるため、全てのパターンを網羅していないとコンパイルエラーとなる。
* 全ての可能性を列挙したくない場合は、`_` で残り全てのパターンをマッチすることができる。

```rust
let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (), // () はただのユニット値なので何もしない
}
```

## if let で簡潔なフロー制御

* `if let` を使うと、ある値にマッチしたときだけに行いたい処理を簡潔に書くことができる。
* `match` を使用する場合と比較して簡潔に書くことができるが、`match` が持つ包括性チェックは失われるので、包括性よりも簡潔性を得るほうが適切な場合に使う。

```rust
let some_u8_value = Some(0u8);

match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
}

// if let で簡潔に書き直す
if let Some(3) = some_u8_value {
    println!("three");
}
```

* `else` を使うこともできる。

```rust
match coin {
    Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    _ => count += 1,
}

// if let else で書き直すと以下
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
```


# 肥大化していくプロジェクトをパッケージ、クレート、モジュールを利用して管理する

Rust のモジュールシステム

* パッケージ： クレートをビルドし、テストし、共有することができる Cargo の機能
* クレート: ライブラリか実行可能ファイルを生成する木構造をしたモジュール郡
* モジュールと use: これを使うことで、パスの構成、スコープ、公開するか否かを決定できる。
* パス: 要素（構造体や関数やモジュールなど）に名前をつける方法

## パッケージとクレート

* クレート
  * ライブラリかバイナリのどちらか。
  * crate root: クレートのルートモジュールを作るソース・ファイル。コンパイラの開始点。
    * src/main.rs: バイナリ用ルートクレートのデフォルト
    * src/lib.rs: ライブラリ用ルートクレートのデフォルト

* パッケージ
  * ある機能郡を提供する一つ以上のクレート
  * Cargo.toml でそれらのクレートをどのようにビルドするのか定義する

* パッケージに src/main.rs と src/lib.rs を置くと、同じパッケージ名を持つライブラリとバイナリのクレート2つを持つことになる
* ファイルを src/bin に置くと、パッケージに複数の別々のバイナリクレートを持つことができる。

## モジュールを定義して、スコープとプライバシーを制御する

* モジュールはクレート内のコードをグループ化する
* モジュールで、要素のプライバシー (public or private) をで制御できる
* `mod モジュール名` でモジュールを定義することができる

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```

上記モジュールは以下のツリー構造となる。

```
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
```

## モジュールツリーの要素を示すためのパス

* ファイルシステムをたどるように、モジュールツリー内の要素を「パス」を使って見つける
* パスは「絶対パス」と「相対パス」がある
* 絶対パス
  * クレート名、あるいは `crate` という名前をクレートルートとして、目的のモジュール要素までのパスを表す
* 相対パス
  * `self`、あるいは `super`、または現在のモジュール内の識別子を使うことで、現在の位置から目的のモジュール要素までのパスを表す
* 絶対パスを使うか相対パスを使うかは、要素を定義するコードを、その要素を使うコードと別々に動かすか一緒に動かすか、どちらが起こりそうかによって決めるのが良い。

```rust
// src/lib.rs

mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // 絶対パス
    crate::front_of_house::hosting::add_to_waitlist();

    // 相対パス （front_of_house は同じ階層なので、識別子を使ってパスをたどることができる）
    front_of_house::hosting::add_to_waitlist();
}
```

* 上記コードはこのままではコンパイルエラー
* モジュールの要素はデフォルトでは非公開(private)
* 親モジュールは、非公開な子モジュールの要素をパスでたどることはできない
  * `mod hosting` と `fn add_to_waitlist` を、それぞれ `pub mod`、`pub fn` にすればコンパイルできる
* 子モジュールは、非公開であってもその祖先モジュールの要素をパスでたどることができる
  * 子モジュールは自分の定義された文脈を見ることができる
* 兄弟モジュールもたどることができる

### 相対パスを super で始める

```rust
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order(); // 親の兄弟要素である serve_order を参照
    }

    fn cook_order() {}
}
```

### 構造体と enum を公開する

* 構造体を pub で公開しても、フィールドはそのままでは非公開

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String, // 公開フィールド
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast { // 公開メソッド
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}
```

* 一方で、enum は公開するとその variant は全て公開される。

```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

## `use` キーワードでパスをスコープに持ち込む

* `use` でパスをスコープに持ち込むことで、パスの要素をローカルに存在するかのように呼び出すことができる。
* 絶対パスのほか、相対パスでも use を使うことができる


```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

* モジュール内の関数までパス指定できるが、モジュールまでに収めるのが慣例的
* 一方で構造体や enum その他の要素を持ち込む場合は、その要素までパス指定するのが慣例的
* しかし名前が衝突する場合は親モジュールのパスを使わないといけない
* あるいは、`as` キーワードで別名を与える

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting::add_to_waitlist; // 慣例的でない

pub fn eat_at_restaurant() {
    add_to_waitlist();
    add_to_waitlist();
    add_to_waitlist();
}

use std::collections::HashMap; // HashMap 構造体。慣例的。

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}

use std::fmt;
use std::io;

fn function1() -> fmt::Result { // io::Result と衝突するので fmt までのパス指定
    // --snip--
    // （略）
}

fn function2() -> io::Result<()> {
    // --snip--
    // （略）
}

use std::fmt::Result;
use std::io::Result as IoResult; // 衝突回避のために as で別名を与える

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

### `pub use` を使って名前を再公開する

* `use` でスコープに持ち込んだ名前を `pub use` とすることで再公開できる

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting; // 外部のコードがhosting::add_to_waitlistを使ってadd_to_waitlist関数を呼び出せるようになる

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

### 外部のパッケージを使う

* Cargo.toml で crates.io からダウンロードするパッケージを指定する

```toml
[dependencies]
rand = "0.5.5"
```

* Cargo.toml でダウンロードしたパッケージを使う
* 標準ライブラリの `std` は予め Rust に含まれているため、Cargo.toml で指定する必要はない

```rust
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
}
```

### 巨大な `use` のリストをネストしたパスを使って整理する

```rust
use std::cmp::Ordering;
use std::io;
```

はネストしたパスを使って以下のように整理できる。

```rust
use std::{cmp::Ordering, io};
```

また、`self` をつかうと、

```rust
use std::io;
use std::io::Write;
```

```rust
use std::io::{self, Write};
```

### glob 演算子

* `*` でそのパスにおいて公開されている要素をすべて持ち込むことができる
* プログラムで使われている名前がどこで定義されたものかわかりづらくなるので注意して使う
* テストの際に `tests` モジュールの公開要素をすべて持ち込むのによく利用される

```rust
use std::collections::*;
```

## モジュールを複数のファイルに分割する

* 当然モジュールは複数ファイルに分割することができる
* `mod front_of_house;` のように、ブロックの代わりに `;` を使うと、モジュールの本体をその名前を持つファイルから探す
* ルートの子要素で `;` を使って、自身の要素名のディレクトリ配下にモジュールを分割することもできる

src/lib.rs

```rust
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

src/front_of_house.rs

```rust
pub mod hosting;
```

src/front_of_house/hosting.rs

```rust
pub fn add_to_waitlist() {}
```


# 一般的なコレクション

* ベクタ型: 可変長の値を並べて保存できる
* 文字列型: 文字のコレクション
* ハッシュマップ: キー、バリューで保存できるマップの特定の実装

## ベクタで一連の値を保存する

* `Vec<T>`

### ベクタの生成

```rust
// 空のベクタの生成
let v: Vec<i32> = Vec::new();

// vec! マクロで生成
let v = vec![1, 2, 3];
```

### ベクタの更新

```rust
// 状態を変更したかったら mut 必要
// また T は、この後のデータ挿入で i32 と推論できるので指定不要
let mut v = Vec::new(); 

v.push(5);
v.push(6);
v.push(7);
v.push(8);
```

### ベクタをドロップすれば、要素もドロップする

```rust
{
    let v = vec![1, 2, 3, 4];

    
} // <- v はここでドロップ。中身の要素もドロップ
```

* 一見単純だが、ベクタの要素への参照を導入した途端、 もうちょっと複雑になる可能性がある
* 次の段落でそれを見ていく

### ベクタの要素を読む

```rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2]; // 添字アクセス。添字が範囲外の場合は panic
let third: Option<&i32> = v.get(2); // get アクセス。Option を返却し、添字が範囲外の場合は None となる。

let third = v.get(2); // 型推論
```

所有権により、ベクタへの不変な参照が有効な間は、可変な参照を同時に行うことはできない。
以下のプログラムはコンパイルエラーとなる。

```rust
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];

v.push(6);

println!("The first element is: {}", first);
```

```
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
  |
4 |     let first = &v[0];
  |                  - immutable borrow occurs here
5 |
6 |     v.push(6);
  |     ^ mutable borrow occurs here
7 | }
  | - immutable borrow ends here
```

新規要素をベクタの終端に追加すると、現在ベクタが確保しているメモリに十分な容量がない場合、メモリを新規確保して古い要素を新しいスペースにコピーする。
その場合、最初の要素を指す参照は解放されたメモリを指すことになる。借用規則によりそのようにならないよう回避される。

### ベクタの値を走査する。

```rust
let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}

let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50; // 可変参照が挿している値を変更するには、* でデリファレンスして値にたどり着かなければならない
}
```

### Enum を使って複数の型を保持する

ベクタは同じ型の値しか保持できないが、Enum を使うことで、異なる型のコレクションを表現することができる。

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

## 文字列でUTF-8でエンコードされたテキストを保持する

文字列は、

* それがテキストとして解釈されたときに有用となるメソッドと、
* バイトのコレクション

として実装されている。

### 文字列とは？

Rust で文字列というと以下の `str` と、`String` ２つのことになる

* `str` (文字列型)
  * 言語のコアとして組み込まれている文字列型
  * 通常、借用された形態 `&str` で見かける。文字列スライス。
  * 文字列スライスは、ある場所に格納されている UTF-8 な文字列データへの参照。
  * 例えば、プログラム中の文字列リテラルは、プログラムバイナリに出力される文字列データを参照する文字列スライスである。
* `String` 型
  * 標準ライブラリで提供される伸張可能で可変な、所有権のある UTF-8 な文字列型。

Rust の標準ライブラリには、他の文字列型も含まれる。

* OsString
* OsStr
* CString
* CStr

など。〜String と 〜Str はそれぞれ所有権ありバージョンと、借用バージョンを表す。
これらの文字列型は、異なるエンコード方法でテキストを格納していたり、メモリ上の表現方法が異なっていたりする。

### 新規文字列を生成する

`String` の生成

```rust
let mut s = String::new(); 

// Display トレイトを実装する型は to_string メソッドを呼び出すことができる。
let s = "initial contents".to_string();

// 文字列リテラルから生成
let s = String::from("initial contents");
```

### 文字列を更新する

#### `push_str` と `push` で文字列に追加する

```rust
let mut s = String::from("foo");
s.push_str("bar");
```

push_str は、引数の所有権を得なくていいので、文字列スライスを引数に取る。
なので以下のコードは有効。

```rust
let mut s1 = String::from("foo");
let s2 = "bar";
s1.push_str(s2);
println!("s2 is {}", s2); // push_str で所有権を渡す必要はないので、s2 は有効
```

push は「文字」を引数に取る。

```rust
let mut s = String::from("lo");
s.push('l');
```

#### `+` 演算子、または `format!` マクロで連結

`+` 演算子

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // s1はムーブされ、もう使用できないことに注意
```

`+` 演算子は以下のようなシグネチャを持つ `add` メソッドを呼び出す。

```rust
fn add(self, s: &str) -> String {  // 実際はジェネリクスを使用する
```

* 上記 `s2` が `+ &s2` となっているのは、`add` の引数が `&str` となっているため
* `&s2` はそのままでは `&String` だが、コンパイラが `&String` を `&str` に **型矯正** してくれる（型矯正については別途説明）
* `add` の第一引数は `&self` ではなく `self` なので所有権をもらう。

複数の文字列を連結すると使いづらくなるので、`format!` マクロを使うと簡潔に書け、所有権も奪わない。

```rust
// 複数の文字列を連結
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = s1 + "-" + &s2 + "-" + &s3;

// format! マクロ
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{}-{}-{}", s1, s2, s3);
```

### 文字列に添え字アクセスする

* Rust では文字列に添え字アクセスすることはできない
* `String` は `Vec<u8>` のラッパーであるが、任意のバイト位置にアクセスしても Unicode として有効な文字を取得できない可能性があるので、Rust ではこれは禁止されている

###  バイトとスカラー値と書記素クラスタ

Rust から UTF-8 文字列を見るには３つの観点がある

* バイト列
* スカラー値
* 書記素クラスタ

ヒンディー語の単語、“नमस्ते”をデーヴァナーガリー(訳注: サンスクリット語とヒンディー語を書くときに使われる書記法)で表記したものを見たら、

バイト列では以下の18バイトになる。
```
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
224, 165, 135]
```

Unicode スカラー値でみたら以下になる。（Rust の char 型もこれ。）
```
['न', 'म', 'स', '्', 'त', 'े']
```

スカラー値では6つの char 値があるが、4番目と6番目は文字ではない。（単独では意味をなさないダイアクリティック）
これを、書記素クラスタ（人間が文字と呼ぶものに一番近い）として見たら、以下の4文字が得られる。
```
["न", "म", "स्", "ते"]
```

Rustには、データが表す自然言語に関わらず、各プログラムが必要な解釈方法を選択できるように、 コンピュータが保持する生の文字列データを解釈する方法がいろいろ用意されている。

### 文字列をスライスする

* 文字列境界として中途半端な位置でスライスするとパニックするので気をつけるべき。添字アクセスするのと同じくしばしば悪い結果になる。

```rust
let hello = "Здравствуйте";

let s = &hello[0..4];
```

### 文字列を走査するメソッド群

上述のヒンディー語の単語、“नमस्ते” を題材とすると、

`chars()` メソッドでは

```rust
for c in "नमस्ते".chars() {
    println!("{}", c);
}
```

以下の6つの char 値が出力される。

```
न
म
स
्
त
े
```

`bytes()` メソッドは、

```rust
for b in "नमस्ते".bytes() {
    println!("{}", b);
}
```

各バイト値をそのまま得る。
```
224
164
// --snip--
165
135
```

書記素クラスタを得ることは複雑であり、標準ライブラリとして提供されていない。
この機能が必要なら、crates.ioでクレートを入手可能。


## キーとそれに紐づいた値をハッシュマップに格納する

* `HashMap<K, V>`

```rust
use std::collections::HashMap; // use 必要

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

// タプルのベクタの collect メソッドによる生成
let teams  = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];

// collect はいろいろなコレクション型にデータをまとめ上げるため、ハッシュマップでほしいことを伝えるために
// HashMap<_, _> の型注釈が必要。K と V の型は推論される。
let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
```

### ハッシュマップと所有権

* `i32` のような `Copy` トレイトを実装する型ならば、その値はハッシュマップにコピーされる。
* String のような所有権のある型ならば、その所有権はハッシュマップにムーブされる。
* 値の参照ならば、その値はムーブされない。ただしハッシュマップが有効な間、その値も有効でなければならない（ライフタイム指定子が必要）

```rust
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value); // ここで field_name と field_value はムーブ
```

### ハッシュマップの値にアクセスする

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

// get(&key) によるアクセス
// 下記 score は Some(&10) になる。get は Option<&V> を返却し、値がない場合は None を返却する
let team_name = String::from("Blue");
let score = scores.get(&team_name);

// for ループによる列挙
for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```

### ハッシュマップを更新する

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25); // 値の上書き

// 値が存在しない場合にinsert。
// entry は Entry という enum を返却する。存在していたりいなかったりする可能性のある値を表す。
scores.entry(String::from("Yellow")).or_insert(50); 
scores.entry(String::from("Blue")).or_insert(50);   


// 古い値にもとづいて値を更新する。
// 以下は単語の出現回数をカウントしている。
let text = "hello world wonderful world";
let mut map = HashMap::new();

for word in text.split_whitespace() {
    // or_insert は Entry に値がある場合に、その値への可変参照を返却する
    let count = map.entry(word).or_insert(0);
    *count += 1;
}
```

### ハッシュ関数

* ハッシュ関数はデフォルトでは DoS 耐性のある暗号学的に安全なハッシュ関数が使われる
* 速度はまぁまぁ
* `BuildHasher` トレイトを実装する独自の hasher を指定することもできる
* crates.io に様々な hasher が公開されている


# エラー処理

* Rust には「回復可能エラー」と「回復不可能なエラー」がある
* 回復可能なエラーは、例えばファイルが見つからないなど、問題をユーザーに報告し再試行することが合理的なもの
* 回復不可能なエラーは、配列の境界を超えたアクセスなどバグの兆候となるもの
* Rust には例外の機構はない。代わりに回復可能なエラーには `Result<T, E>` 値があり、回復不可能なエラーには `panic!` マクロがある。

## `panic!` による回復不可能なエラー

### パニックに対してスタックを巻き戻すか異常終了するか

* 標準では、パニックするとスタックを巻き戻し、遭遇した各関数のデータの片付けを行う
* この遡りと片付けはすべきことが多い
* 巻き戻しの対立案としては、即座に異常終了し、使用していたメモリなどの片付けを OS に任せること
* Cargo.toml の適切な [profile] 欄に `panic = 'abort'` を追記することで、 パニック時に巻き戻しから異常終了するように切り替えることができる
* abort にすると、実行可能ファイルのサイズを小さく抑えることができる (?)

```toml
# リリースモード時に巻き戻しではなく異常終了する例
[profile.release]
panic = 'abort'
```

### `panic!` バックトレースを使用する

* `RUST_BACKTRACE` 環境変数に 0 以外をセットするとバックトレースが出力される
* バックトレースで情報を十分に得るにはデバッグシンボルを有効にする必要があり、`--release` オプションなしで `cargo build / run` していればデフォルト有効になる

## `Result` による回復可能なエラー

Result enum で結果、あるいは回復可能なエラーを表現することができる

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

```rust
use std::fs::File;

fn main() {
    // File::open は Result<std::fs::File, std::io::Error> を返却する
    // f のインスタンスは open が成功したときは Ok<std::fs::File> となり、失敗したときは Err<std::io::Error> となる
    let f = File::open("hello.txt");

    // match アーム内で成功と失敗をハンドリングする
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            // ファイルを開く際に問題がありました
            panic!("There was a problem opening the file: {:?}", error)
        },
    };
}
```

### 色々なエラーにマッチする

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        // `if error.kind() == ErrorKind::NotFound` はマッチガードと呼ばれる。この式に一致した場合にのみブロックが実行される。
        // `ref` は、 error の所有権がマッチガード式に移ってしまわないようにするために必要。
        // `&` だと 参照にマッチしその値を返すが、`ref` は値にマッチしその参照を返す 
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        //ファイルを作成しようとしましたが、問題がありました
                        "Tried to create file but there was a problem: {:?}",
                        e
                    )
                },
            }
        },
        Err(error) => {
            panic!(
                "There was a problem opening the file: {:?}",
                error
            )
        },
    };
}
```

### エラー時にパニックするショートカット: `unwrap` と `expect`

`Result<T, E>` は色々な作業をするヘルパーメソッドを備える

```rust
use std::fs::File;

fn main() {
    // unwrap は Result 値が Ok 列挙子 のときその中身を返却し、Err ならば panic! を呼ぶ
    let f = File::open("hello.txt").unwrap();

    // expect は unwrap と似ているが、panic! を呼ぶ際のエラーメッセージを指定することができる
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
```

### エラーを委譲する

`Result<T, E>` を返却する関数は、その処理中におきた回復可能なエラーを呼び出し元に委譲することもできる

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

Rust において、この種のエラー委譲は非常に一般的なので `?` 演算子を使ったショートカット構文が存在する

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    // ? は Ok ならばその値を返し、Err ならば return キーワードを使ったようにそれを関数から返却する
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```
 
* `?` 演算子と `match` 式には違いがある
* `?` の場合は、受け取ったエラー型が現在の関数の戻り値型で定義されているエラー型に変換される
* 標準ライブラリの `From` トレイトの `from` 関数を通ることにより実現される
* `?` は `Result` を返す関数でしか使用することができない

`?` 呼び出し後の処理を連結することで以下のようにさらに簡潔に書くこともできる

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
```

## `panic!` するべきかするまいか

### プロトタイプコード、テスト

プロトタイプコードでは `unwrap` や `expected` を積極的に使って良い

* => 頑健なエラー処理により、プロトタイプが何を実証しようとしているのかわかりづらくする恐れがある
* => プロトタイプ時点では `unwrap` などによりエラー簡易に panic! で扱い、後々 `unwrap` などを呼び出していく部分を `match` などで置き換えブラッシュアップしていくことができる

テストコードでも `unwrap` や `expected` を使っていくべき

* => `panic!` がテスト失敗と印付けられる手段なので

### コンパイラよりもプログラマがより情報を持っている場合

コンパイラよりもプログラマがより情報を持っている場合、つまり `Result` が `Ok` であると確信できる場合、`unwrap` や `expected` を積極的に使って良い。例えば以下。


```rust
use std::net::IpAddr;

let home: IpAddr = "127.0.0.1".parse().unwrap();
```

### エラー処理のガイドライン

panic! する際のガイドライン

* コードが悪い状態、つまり無効な値、矛盾する値、行方不明な値がコードに渡されるとき

かつ以下のいずれか一つ以上の状態であるとき、panic! すべし

* 悪い状態が起こると予想されていないとき
* この時点以降、この悪い状態にないことを頼りにコードが書かれているとき
* 使用している型に、この悪い状態の情報をコード化するいい手段がないとき

そうでない場合は、Result で呼び出し元にエラー処理をさせる余地を残したほうが適切


# ジェネリック型、トレイト、ライフタイム

## ジェネリックなデータ型

### 関数定義では

イメージ的には以下のような形でジェネリックな関数定義を行う

```rust
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```

ただしこちらはこのままではコンパイルできない。
すべての T となりうる型が比較可能でない可能性があるため。
これにはジェネリックな型を特定のトレイトを持つものに限定することで解消できるが、後ほど説明する

```
error[E0369]: binary operation `>` cannot be applied to type `T`
(エラー: 2項演算`>`は、型`T`に適用できません)
 --> src/main.rs:5:12
  |
5 |         if item > largest {
  |            ^^^^^^^^^^^^^^
  |
  = note: an implementation of `std::cmp::PartialOrd` might be missing for `T`
  (注釈: `std::cmp::PartialOrd`の実装が`T`に対して存在しない可能性があります)

```

### 構造体定義では

```rust
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
```

複数の型に対応するならば以下のようにかける

```rust
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let integer_and_float = Point { x: 5, y: 4.0 };
}
```

### enum 定義では

標準ライブラリの Option や Result の定義

```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

### メソッド定義では

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
```

ジェネリックな型を持つ `Point<T>` インスタンスではなく、`Point<f32>` にのみ、メソッドを実装したい場合、以下のようにできる。

```rust
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
```

### ジェネリクスを使用したコードのパフォーマンス

* Rust では、ジェネリックな型を使用したコードが、具体的な型を使用したコードよりも遅くならないように実装されている。
* コンパイル時に単相化(monomorphization)を行っている。単相化は、コンパイル時に使用されている具体的な型を入れ、ジェネリックなコードを特定の型のコードに変換する。


## トレイト: 共通の振る舞いを定義する

* 特定の型に存在し、他の型と共有できる機能
* 共通の振る舞いを抽象的に表現できる
* 違いはあるものの、他言語のインターフェース機能に類似している

### トレイトを定義する

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

### トレイトを方に実装する

```rust
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {  // impl の後に実装するトレイトの名前、for の後に実装対象の型を指定する
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

トレイトの制限

* トレイトか、あるいは実装対象の型がクレートローカルである場合にのみ、型に対してトレイトを実装できる
  * 上記例でいうと、標準ライブラリのトレイト `Display` を `NewsArticle` に実装することは、`NewsArticle` 型がクレートローカルであるため可能である
  * クレートローカルのトレイト `Summary` を、標準ライブラリの型 `Vec<T>` に実装することは、`Summary` トレイトがクレートローカルであるため可能である
* 外部のトレイトを外部の型に対して実装することはできない
  * 外部のトレイト `Display` を、外部の型 `Vec<T>` に対して実装することはできない
  * coherence, orphan rule と呼ばれ、この規則により、他の人のコードが自分のコードを壊すことがないことを保障する
  * この規則がないと、2つのクレートが同じ型に対して同じトレイトを実装できてしまい、 コンパイラはどちらの実装を使うべきかわからなくなってしまう

### デフォルト実装

```rust
pub trait Summary {
    fn summarize(&self) -> String { // ブロックを続けることでデフォルト定義になる
        // "（もっと読む）"
        String::from("(Read more...)")
    }
}

impl Summary for NewsArticle {} // デフォルト実装の利用

impl Summary for Tweet {
    fn summarize(&self) -> String {  // デフォルト実装のオーバーライドになる
        format!("{}: {}", self.username, self.content)
    }
}
```

トレイトのデフォルト実装から、デフォルト実装を持たない他のメソッドを呼び出すことができる

```rust
pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        // "（{}さんの文章をもっと読む）"
        format!("(Read more from {}...)", self.summarize_author())
    }
}
```

### 引数としてのトレイト

以下のようにすることで、引数として `Summary` トレイトを実装するものを受け取ることができる

```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

#### トレイト境界構文

実は上記は **トレイト境界 (trait bound)** と呼ばれる以下のような記述のシンタックスシュガーである

```rust
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

並べて比較すると以下のような感じ
```rust
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
}

pub fn notify<T: Summary>(item1: &T, item2: &T) {
}
```

簡単なケースに対し `impl Trait` 構文はコードを簡潔にすることができ便利。
より複雑なケースの場合、トレイト境界構文で複雑な状態を表現することができる。

#### 複数のトレイト境界を `+` 構文で指定する

`Summary` と標準ライブラリの `Display` 両方を実装する引数を指定するには以下のように `+` 構文を使う

```rust
pub fn notify(item: &(impl Summary + Display)) {
}

// トレイト境界構文
pub fn notify<T: Summary + Display>(item: &T) {
}
```

#### `where` 句を使ったより明確なトレイト境界

大量のトレイト境界に関する情報をメソッドシグネチャに指定すると読みづらくなる

```rust
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
}
```

代わりに `where` 句を使って、より簡潔に記載することができる

```rust
fn some_function<T, U>(t: &T, u: &U) -> i32
    where T: Display + Clone,
          U: Clone + Debug
{
}
```

### トレイトを実装している型を返す

```rust
// Summary を実装する何らかの型を返す
fn returns_summarizable() -> impl Summary {
}
```

`-> impl Trait` は一種類の型を返す場合にのみ使える。例えば以下のように `NewsArticle` か `Tweet` を返すようなコードは失敗する

```rust
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}
```

### トレイト境界を使用して、メソッド実装を条件分けする

以下は、Pair インスタンス作成時の `T` が、`Display` と `PartialOrd` トレイトを実装する場合にのみ有効となる `cmp_display` メソッドを定義している

```rust
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// T が Display + PartialOrd を実装する場合にのみ有効となるメソッド
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

あるトレイトを実装するあらゆる型、それに対するトレイト実装を行うこともできる。
以下は標準ライブラリの例だが、`Display` トレイトを実装するあらゆる型 `T` に、`ToString` トレイト実装を行っている。
このような実装は **ブランケット実装** と呼ばれている。

```rust
pub trait ToString {
    fn to_string(&self) -> String;
}

// ブランケット実装
impl<T: Display> ToString for T {
    // --snip--
}
```


## ライフタイムで参照を検証する

* Rust において、参照は全てライフタイムを保持する
* ライフタイムとは、その参照が有効になるスコープ
* 型と同じように、大体の場合ライフタイムは推論される
* 同様に、ライフタイムが推論できない場合、注釈しなければならない

### ライフタイムでダングリング参照を回避する

* ライフタイムの主な目的はダングリング参照を回避すること
* ダングリング参照があるプログラムは、参照するつもりだったデータ以外のデータを参照してしまう可能性がある
* Rust は「借用チェッカー」により、ライフタイムの有効性をチェックしている

### 借用チェッカー    

以下のコードは借用チェッカーによりコンパイルエラーになる

```rust
{
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}
```

```
error[E0597]: `x` does not live long enough
(エラー: `x`の生存期間が短すぎます)
  --> src/main.rs:7:5
   |
6  |         r = &x;
   |              - borrow occurs here
   |              (借用はここで起きています)
7  |     }
   |     ^ `x` dropped here while still borrowed
   |     (`x`は借用されている間にここでドロップされました)
...
10 | }
   | - borrowed value needs to live until here
   | (借用された値はここまで生きる必要があります)
```

借用チェッカーは以下のようにライフタイムの有効性をチェックしている

```rust
{
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}
```

* 変数 `r` が指す値のライフタイムは `'a`  
* 変数 `x` が指す値のライフタイムは `'b`
* コンパイラが、`x` のライフタイムは  `'a`  だが、 `'b` の間しかライフタイムがないメモリを参照していることを確認する
* `'b` は `'a` よりライフタイムが短いので、コンパイルエラーとなる

### 関数のジェネリックなライフタイム

以下のコードはコンパイルエラーとなる

```rust
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

```
error[E0106]: missing lifetime specifier
(エラー: ライフタイム指定子が不足しています)
 --> src/main.rs:1:33
  |
1 | fn longest(x: &str, y: &str) -> &str {
  |                                 ^ expected lifetime parameter
  |                                   (ライフタイム引数が予想されます)
  |
  = help: this function's return type contains a borrowed value, but the
signature does not say whether it is borrowed from `x` or `y`
  (助言: この関数の戻り値型は借用された値を含んでいますが、
シグニチャは、それが`x`か`y`由来のものなのか宣言していません)
```

返却している参照が `x` のものなのか `y` のものなのかわからないから。
`x` と `y` の具体的なライフタイムが借用チェッカーには判別できないのでエラーとなっている。
このエラーを解消するにはライフタイムを注釈する必要がある。

### ライフタイム注釈記法

* `&'a i32` で、`'a` というライフタイム注釈を持つ `i32` を表現することができる
* 1つのライフタイム注釈それだけでは、大して意味はない
* 複数のライフタイム注釈つきの引数が、 お互いにどう関係するかをコンパイラに指示することを意図している

前段のコードのエラーは以下のようにライフタイムを注釈することでコンパイルすることができる。

```rust
// x と y は、それぞれ 'a で注釈されるライフタイムを「少なくとも」持つことをコンパイラに教える。
// 実際には x と y のライフタイムは異なっていてもよく、よりライフタイムが短いほうが 'a のライフタイム値として採用され、それが戻り値のライフタイムとなる。
// 戻り値を 'a より長いライフタイムを持つ変数に代入しようとするとエラーになるので注意する
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

### 構造体定義のライフタイム注釈

```rust
// ImportantExcerpt のインスタンスが、partが参照する値のライフタイムよりも長生きしないことを注釈する
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    // 僕をイシュマエルとお呼び。何年か前・・・
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");  // '.'が見つかりませんでした
    let i = ImportantExcerpt { part: first_sentence };
}
```

### ライフタイム省略

以下のコードは、引数と戻り値が参照であるにも関わらず、ライフタイム注釈なしでコンパイルすることができる。

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

Rust 1.0 以前ではこのようなコードはコンパイルエラーとなり、以下のように全参照に明示的にライフタイムを注釈する必要があった

```rust
fn first_word<'a>(s: &'a str) -> &'a str {
```

しかし現在では、**3つのライフタイム省略規則** に当てはまるパターンであれば、その注釈を省略できるようになっている。

3つのライフタイム省略規則は以下のとおり

* 1つ目: 参照である各引数は、注釈しない限り独自のライフタイム引数を得る
  * 1引数の関数は、1つのライフタイム引数を得る `fn foo<'a>(x: &'a i32)`
  * 2引数の関数は、2つのライフタイム引数を得る `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`
* 2つ目: 1つだけ入力ライフタイム引数があるなら、そのライフタイムが全ての出力ライフタイム引数に代入される  
  * 入力ライフタイム引数は関数引数のこと。出力は戻り値。
  * `fn foo<'a>(x: &'a i32) -> &'a i32` のようになる。
* 3つ目: 複数の入力ライフタイム引数があるけれども、メソッドなのでそのうちの一つが `&self` や `&mut self` だったら、 `self` のライフタイムが全出力ライフタイム引数に代入される

### メソッド定義のライフタイム注釈

以下は3つ目の省略規則が適用されるため、メソッドの引数や戻り値について注釈が必要無い例。

```rust
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        // お知らせします
        println!("Attention please: {}", announcement);
        self.part
    }
}
```

### 静的ライフタイム

* `'static` という特殊なライフタイムがあり、静的ライフタイムと呼ばれる
* 静的ライフタイムはプログラム全体の期間を示す
* 文字列リテラルはすべて静的ライフタイムをもっている。したがって、リテラルはプログラム全体の期間に渡って有効となる。
  * 文字列のテキストは、プログラムのバイナリに直接格納され、常に利用可能。故に、全文字列リテラルのライフタイムは `'static`


# 自動テストを書く

## テストの記述法

* `#[test]` で注釈された関数がテスト関数となる
* `cargo test` でそのプロジェクトの全テスト実行
* Rust の Nightly ビルドでベンチマークテストも利用可能になっている
* テスト関数実行中のスレッドが `panic!` に遭遇するとそのテストは失敗とマークされる  

```rust
// シンプルなテスト
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4)
}
```

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// テストを内部モジュールにまとめている
#[cfg(test)]
mod tests {
    // Rectangle を import　するために use super::*; する
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }
}
```

### `assert_eq!` と `assert_ne!` で等価性をテストする

* `assert_eq!` と `assert_ne!` マクロで等価性のアサーションができる
* `==` と `!=` を利用してアサーションするので、比較対象の値は `PartialEq` と `Debug` トレイトを実装していなければならない
* どちらのトレイトも導出可能なトレイトなので、通常は単純に構造体や enum 定義に `#[derive(PartialEq, Debug)]` を注釈するだけで使える

### カスタムの失敗メッセージを追加する

* `assert!` マクロなどに追加引数でカスタムメッセージを追加することができる

```rust
pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}

#[test]
fn greeting_contains_name() {
    let result = greeting("Carol");
    assert!(
        result.contains("Carol"),
        //挨拶(greeting)は名前を含んでいません。その値は`{}`でした
        "Greeting did not contain name, value was `{}`",
        result
    );
}
```

### `should_panic` でパニックを確認する

* `#[should_panic]` で関数を注釈することでパニックすることを検証できる

```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                //予想値は1以上でなければなりませんが、{}でした。
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                //予想値は100以下でなければなりませんが、{}でした。
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    //予想値は100以下でなければなりません
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```

### `Result<T, E>` をテストで使う

* パニックや assert の代わりに、テスト関数で `Result<T, E>` を返してテストすることもできる

```rust
fn it_works() -> Result<(), String> {
    if 2 + 2 == 4 {
        Ok(())
    } else {
        Err(String::from("two plus two does not equal four"))
    }
}
```

## テストの実行のされ方を制御する

* `cargo test` はコードをテストモードでコンパイルし、出来上がったテストバイナリを実行する
* テストバイナリの既定動作は、テストを全て並行に実行し、出力をキャプチャしテスト結果に関係する出力を読みやすくする
* `cargo test --help` で `test` のヘルプを、`cargo test -- --help` で `cargo test --` の区分記号の後に使えるオプションが表示される
* `cargo test -- --test-threads=1`: テスト実行スレッド数（並列度）指定
* `cargo test -- --nocapture`: テスト対象関数の出力をキャプチャせずそのまま出力
* `cargo test <function name prefix>`: テスト対象関数名を指定
* `cargo test -- --ignored`: `#[ignore]` で ignore にしたテスト関数を実行

## テストの体系化

* Rust ではテストを「単体テスト」と「結合テスト」の2つのカテゴリで捉えている
* 単体テスト
  * 小規模で集中
  * 一回に一モジュールをテスト
  * 非公開のインターフェースをテストすることもある
* 結合テスト
  * ライブラリ外から、外部コードが自分のコードを使うのと同様にテストする
  * 公開インターフェースのみを使用
  * 一回に複数のモジュールをテストすることもある

### 単体テスト

* 慣習的に、src 配下の、テスト対象のコードが含まれる各ファイルに `tests` という名前のモジュールを作り、テスト関数を実装し、そのモジュールを `#[cfg(test)]` で注釈する
* `#[cfg(test)]` とすることで、`cargo build` ではなく `cargo test` を走らせたときだけテストコードをコンパイルするよう指示することができる
  * 結合テストは src とは別のディレクトリに作成するので `#[cfg(test)]` は必要ない

例: src/lib.rs

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

### 結合テスト

* src と同じ階層の tests ディレクトリに結合テスト用のファイルを配置する
* 結合テストファイルはいくつでも配置でき、それぞれ個別のクレートとしてコンパイルされる

例: tests/integration_test.rs

```rust
extern crate adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}
```

* それぞれの結合テストでコードを共有したい場合、`tests/<module_name>/mod.rs` のようなモジュールにそれを配置する

例: 
tests/common/mod.rs

```rust
pub fn setup() {
    // ...
}
```

tests/integration_test.rs

```rust
extern crate adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
```

### バイナリクレート用の結合テスト

* `src/main.rs` ファイルのみを含むバイナリクレートだったら、 testsディレクトリに結合テストを作成し、 extern crateを使用して `src/main.rs` ファイルに定義された関数をインポートすることはできない
* そのため、バイナリ提供する Rust プロジェクトでは、しばしば　`src/lib.rs` にロジックを配置し `src/main.rs` はそれを呼び出すだけになっているものがある。ロジックを lib.rs に配置することで、結合テスト対象にすることができる


# 入出力プロジェクト: コマンドラインプログラムを構築する

https://github.com/kei2100/playground-rust/tree/main/minigrep

# 関数型言語の機能: イテレータとクロージャ

## クロージャ: 環境をキャプチャできる匿名関数

```rust
let expensive_closure = |num| {  // クロージャ本体が式一つなら、波括弧は省略可能
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num // クロージャの戻り値
};
```

### クロージャの型推論と注釈

* クロージャのスコープはごく限定的となることが多いので、多くの場合型推論が効く
* 必要な場合は以下のように注釈することが可能

```rust
let expensive_closure = |num: u32| -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
};
```

### ジェネリック引数とFnトレイトを使用してクロージャを保存する

`Fn` トレイトを使って、クロージャやクロージャ呼び出し結果の値をキャッシュする構造体を作成できる

```rust
struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T, // calculation は Fn(u32) -> u32 のクロージャを保持する
    value: Option<u32>,
}

impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    // value 関数は self.value に値がなければ calculation クロージャを実行し、値をキャッシュし返却する。
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }
}

fn foo() {
    // Cacher の生成
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });
    println!(
        "Today, do {} pushups!",
        expensive_result.value(intensity)
    );}
```

### クロージャで環境をキャプチャする

他言語のクロージャと同じように、クロージャ定義時のスコープをキャプチャできる。
クロージャが環境から値をキャプチャすると、メモリを使用してクロージャ本体で使用できるようにその値を保存する。

```rust
fn main() {
    let x = 4;

    let equal_to_x = |z| z == x;

    // 以下のように、クロージャではなく関数にすると x をキャプチャすることはできないので
    // コンパイルエラーとなる
    // fn equal_to_x(z: i32) -> bool { z == x }

    let y = 4;

    assert!(equal_to_x(y));
}
```


クロージャは３つの方法で環境から値をキャプチャできる。これらはそれぞれ以下のように `Fn` トレイトでコード化されている。

* 所有権を奪う: `FnOnce` トレイト。環境から値をムーブする。
* 可変で借用する: `FnMut` トレイト。可変で値を借用するので、環境を変更することができる
* 不変で借用する: `Fn` トレイト。環境から不変で値を借用する。

クロージャを生成する際、コンパイラがどのトレイトを使うか推論する。

* 全てのクロージャは少なくとも1回は呼び出されるので、全てのクロージャは `FnOnce` を実装している
* キャプチャした変数をムーブしないクロージャは、 `FnMut` も実装する
* キャプチャした変数に可変でアクセスする必要のないクロージャは、`Fn` も実装している

環境でクロージャが使用している値の所有権を奪うことをクロージャに強制したいなら、引数リストの前にmoveキーワードを使用できる。
このテクニックは、新しいスレッドにデータが所有されるように、クロージャを新しいスレッドに渡して、 データをムーブする際に大概は有用。

```rust
fn main() {
    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    // ここでは、xを使用できません: {:?}
    println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}
```

## 一連の要素をイテレータで処理する

```rust
let v1 = vec![1, 2, 3];

// Rust においてイテレータは怠惰。つまりイテレータを使い込んで消費するメソッドを呼ぶまではなんの効果もない
let v1_iter = v1.iter();

for val in v1_iter {
    println!("Got: {}", val);
}
```

### Iterator トレイトと next メソッド

* 全てのイテレータは、標準ライブラリの `Iterator` を実装している
* `type Item` と `Self::Item` というのはトレイトの **関連型** と呼ばれるもの。詳しくは後述
* `next` メソッドは、イテレータに次の要素がある場合、`Some<Self::Item>` を返却し、なければ `None` を返却する

```rust
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // デフォルト実装のあるメソッドは省略
    // methods with default implementations elided
}
```

* next メソッドを直接呼び出した場合の挙動は以下のようになる
* 直接呼び出す場合は、イテレータに副作用があるので、`let mut v1_iter` のように可変にする必要がある。for ループでは `v1_iter` を可変にする必要がなかったが、これはループが `v1_iter` の所有権を奪い可変にするといった操作を裏で実行しているから
* next はベクタ値の不変な参照であることにも注目
* `v1` の所有権を奪い、所有された値をを返すイテレータがほしいなら `v1.into_iter()` を呼び出す
* また、ベクタ値の可変な参照がほしいなら、`v1.iter_mut` を呼び出す

```rust
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3];

    let mut v1_iter = v1.iter();

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
}
```

### イテレータを消費するメソッド (consuming adaptors)

* `Iterator` トレイトには、多くのデフォルト実装メソッドがある
* それらのメソッドは、内部でイテレータの `next` メソッドを呼び出しイテレータを消費するため、**消費アダプタ (consuming adaptors)** と呼ばれる
* 例えば `sum` メソッドはイテレータの所有権を奪い、`next` メソッドを呼び出し続けることでイテレータの要素数を数える。
* `sum` メソッドはイテレータの所有権を奪うため、呼び出したあとにそのイテレータを使うことはできない

```rust
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    assert_eq!(total, 6);
}
```

### 他のイテレータを生成するメソッド (iterator adaptors)

* イテレータを別のイテレータに変えるものを **イテレータアダプタ (iterator adaptors)** と呼ぶ

```rust
let v1: Vec<i32> = vec![1, 2, 3];

// map はクロージャを取り、イテレータの要素をそのクロージャにより変更するイテレータアダプタ
let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

assert_eq!(v2, vec![2, 3, 4]);
```

### Iteratorトレイトで独自のイテレータを作成する

```rust
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}

#[test]
fn calling_next_directly() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(), Some(1));
    assert_eq!(counter.next(), Some(2));
    assert_eq!(counter.next(), Some(3));
    assert_eq!(counter.next(), Some(4));
    assert_eq!(counter.next(), Some(5));
    assert_eq!(counter.next(), None);
}
```

## パフォーマンス比較: ループVSイテレータ

* Rust はゼロコスト抽象化という原則があり、抽象化を行うことが追加の実行時オーバーヘッドを産まないようにすることを良しとしている
* C++ のゼロオーバーヘッド原則に似ている。
  * 一般的に、C++の実装は、ゼロオーバーヘッド原則を遵守します: 使用しないものには、支払わなくてよい。 さらに: 実際に使っているものに対して、コードをそれ以上うまく渡すことはできない。
* 内部実装がゼロコスト抽象化原則に従っている限り、低レベルなループと高レベルなイテレータでパフォーマンスが異なることはない


# Cargo と Crates.io についてより詳しく

## リリースプロファイルでビルドをカスタマイズする

* Cargo には `dev` プロファイルと `release` プロファイルの2つのプロファイルが用意されている
* `cargo build` は `dev` プロファイル、`cargo build --release` で `release` プロファイルになる
* `dev`　は開発用、`release` はリリース用にそれぞれ最適化されたビルドを実行するが Cargo.toml でその動作をカスタマイズできる
* それぞれのプロファイルの設定一覧は https://doc.rust-lang.org/cargo/ を参照
　
```toml
# デフォルトは以下で、それぞれ変更可能
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

## Crates.io にクレートを公開する

### 役に立つドキュメンテーションコメントをする

```rust
/// Adds one to the number given.
/// 与えられた数値に1を足す。
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, my_crate::add_one(5));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

* `///` でコメントを始めると Markdown を記述できる
* `cargo doc --open` をすると現在のクレートのドキュメント HTML をブラウザで開くことができる
* `# Examples` セクション以外によく使われるセクション
  * `# Panics`: 対象の関数などが `panic!` する可能性のある筋書きを記載する
  * `# Errors`: 対象の関数が `Result` を返すなら、起きうるエラーの種類とその条件を記載する
  * `# Safety`: 対象の関数を呼び出すのが `unsafe` なら、その理由と呼び出し元に保持していると期待する不変条件を記載する

### テストとしてのドキュメンテーションコメント

```rust
/// Adds one to the number given.
/// 与えられた数値に1を足す。
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, minigrep::add_one(5));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```
* 上記のような `///` のコメント内に書かれたコード片は `cargo test` でテストすることができる。

### 含まれている要素にコメントする

```rust
//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

//! #自分のクレート
//!
//! `my_crate`は、ユーティリティの集まりであり、特定の計算をより便利に行うことができます。

/// Adds one to the number given.
/// 与えられた数値に1を足す。
///
/// # Examples
///
/// ```
/// let five = 5;
///
/// assert_eq!(6, minigrep::add_one(5));
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}
```

* `//!` は慣例的に `src/lib.rs` などで使用され、クレートやモジュール全体のコメントとなる
* markdown が書ける

### `pub use` で便利な公開 API をエクスポートする

* `pub use` で深い階層の公開 API をクレートのルートなどの API としてエクスポートすることができる

### Crates.io のアカウントをセットアップする

* `$ cargo login <access token>` で https://crates.io/me/ で発行したアクセストークンを設定する
* `~/.cargo/credentials` に保存される

### Crates.io に公開する

* Cargo.toml にパッケージに関する情報を記載し `cargo publish` すると公開される
* 公開は永久で、バージョンは上書きできず削除もできないので注意すること

```toml
[package]
name = "guessing_game"
version = "0.1.0"
authors = ["Your Name <you@example.com>"]
description = "A fun game where you guess what number the computer has chosen."
license = "MIT OR Apache-2.0"

[dependencies]
...
```

### cargo yank で Crates.io からバージョンを取り下げる

* 削除はできないものの、`cargo yank --vers <version>` すると他のパッケージが新たにこのパッケージに依存することを防ぐことができる
* `cargo yank --vers <version> --undo` で取り下げのキャンセルもできる


## Cargo のワークスペース

* ワークスペース機能で、一つのライブラリクレートを複数のライブラリクレートに分割できる
* ワークスペースは、一つの Cargo.lock と出力ディレクトリを共有する


すでにある 「adder」 バイナリクレートに 「add-one」 ライブラリクレートを追加するような場合、

```toml
[workspace]

members = [
    "adder",
    "add-one", # 追加
]
```

を追加して、`cargo new add-one --lib` すると以下のディレクトリ構造を得る

```
├── Cargo.lock
├── Cargo.toml
├── add-one
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── adder
│   ├── Cargo.toml
│   └── src
│       └── main.rs
└── target
```

adder から add-one への依存を作成するには、adder/Cargo.tml に以下を記載する

```toml
[dependencies]

add-one = { path = "../add-one" }
```

* `cargo test -p add-one` のようにすると特定のクレートのみテストすることができる

## cargo installでCrates.ioからバイナリをインストールする

* crates.io に登録されたバイナリクレートは `cargo install <crate name>` でインストールすることができる
* `rustup` でセットアップした際のデフォルトのバイナリインストールパスは `$HOME/.cargo/bin`

## 独自のコマンドで Cargo を拡張する

* $PATH にあるバイナリが `cargo-something` のような名前なら、`cargo something` を実行することで Cargo のサブコマンドであるかのように実行することができる
* `cargo --list` するとそのようなコマンドを列挙することができる


# スマートポインタ

* ポインタ
  * メモリのアドレスを含む変数の一般的な概念
  * Rust では `&` で表され、指している値を借用する
  * 単にデータを参照すること以外特別な能力はなく、オーバーヘッドもない
* スマートポインタ
  * C++ に端を発する、ポインタのように振る舞い追加のメタデータと能力を持つデータ構造
  * 例えば、参照カウント
  * Rust において、通常のポインタ（参照）はデータを借用するだけだが、スマートポインタはデータを所有する
  * `String` や `Vec<T>` も実はスマートポインタ
* スマートポインタの実装
  * 通常、構造体で実装される
  * `Deref` と `Drop` トレイトを実装する
* 標準ライブラリのスマートポインタの一例
  * ヒープに値を確保する `Box<T>`
  * 複数の所有権を可能にする参照カウント型の `Rc<T>`
  * `RefCell<T>` を通してアクセスされ、コンパイル時ではなく実行時に借用規則を強制する型の `Ref<T>` と `RefMut<T>`


## ヒープのデータを指す `Box<T>` を使用する

* `Box<T>` により、スタックではなくヒープにデータを格納することができる。スタックに残るのはヒープデータへのポインタ
* データをヒープに格納する以外はオーバーヘッドはないが、多くのおまけの能力もある
  * コンパイル時にはサイズを知ることができない型があり、正確なサイズを要求する文脈でその型の値を使用する時
  * 多くのデータがあり、所有権を転送したいが、その際にデータがコピーされないようにしたい時
  * 値を所有する必要があり、特定の型ではなく特定のトレイトを実装する型であることのみ気にかけている時

### `Box<T>` を使ってヒープにデータを格納する

```rust
fn main() {
    let b = Box::new(5);
    println!("b = {}", b);
}
```

* 上記コードにおいて、i32 の 5 はスタックではなくヒープに配置される
* `b` 自体はスタックに配置される
* main を抜けると、スタックに配置されている `b` も、ヒープに配置されている `5` もメモリ解放される

### ボックスで再帰的な型を可能にする

コンスリストのような再帰的な型の場合、値のネストが無限に続く可能性があるので、コンパイラはその値を格納するのに必要な領域を決定できない。


```rust
enum List {
    Cons(i32, List),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
```

コンスリストの構造

![image](https://user-images.githubusercontent.com/1415655/105613623-006f2c80-5e07-11eb-9882-0fb217977b0f.png)

上記 `List::Cons` の必要なサイズを計算するには、内包する `List` のサイズを知ることが必要で、定義上はそれが再帰的に無限に続く可能性がある。
そのため上記コードは `list` をスタックに配置するのに必要なサイズを決定できず、コンパイルエラーとなる。


```rust
error[E0072]: recursive type `List` has infinite size
(エラー: 再帰的な型`List`は無限のサイズです)
 --> src/main.rs:1:1
  |
1 | enum List {
  | ^^^^^^^^^ recursive type has infinite size
2 |     Cons(i32, List),
  |               ----- recursive without indirection
  |
  = help: insert indirection (e.g., a `Box`, `Rc`, or `&`) at some point to
  make `List` representable
  (助言: 間接参照(例: `Box`、`Rc`、あるいは`&`)をどこかに挿入して、`List`を表現可能にしてください)
```

これは Box を使って以下のようにすることでコンパイルエラーを解消することができる

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let list = Cons(1,
        Box::new(Cons(2,
            Box::new(Cons(3,
                Box::new(Nil))))));
}
```

上記 `List::Cons` は、先程と違い、`List` ではなく `Box<List>` を持つ。
`Box<T>` はポインタなので、コンパイラが必要な領域を決定できコンパイルエラーは解消される。

`Box<T>` 型は、Deref トレイトを実装しているので、スマートポインタであり、 このトレイトにより `Box<T>` の値を参照のように扱うことができる。
`Box<T>` 値がスコープを抜けると、 Drop トレイト実装によりボックスが参照しているヒープデータも片付けられる

## Deref トレイトでスマートポインタを普通の参照のように扱う

Deref トレイトで `*`（参照外し演算子）の振る舞いをカスタマイズすることができる

### 参照外し演算子で値までポインタを追いかける

```rust
fn main() {
    let x = 5;  // x は 5
    let y = &x; // y は i32 の 5 の値をもつ x の参照

    assert_eq!(5, x);
    assert_eq!(5, *y); // 参照外しで 5 の値を得る
    // assert_eq!(5, y);  // これはコンパイルエラー。参照と数値はことなる型なので比較できない
}
```

### `Box<T>`  を参照のように扱う

前段のコードは  Box を使って以下のように書くことができる

```rust
fn main() {
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // 参照外し演算子を使いボックスのポインタが指す値を得ることができる
}
```

### 独自のスマートポインタを定義し、Deref トレイトを実装して型を参照のように扱う

Box のように振る舞う独自のスマートポインタを定義する
Deref トレイトを実装することで、参照外し演算子が作用したときの動作を定義する

```rust
use std::ops::Deref;

// MyBox は T 型の要素を一つ持つタプル
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    // Deref トレイトが使用する関連型を定義する。関連型については後ほど説明
    type Target = T;

    fn deref(&self) -> &T {
        // self の先頭要素への参照を返す
        &self.0
    }
}
```

```rust
fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
```

`*y` のとき、コンパイラは水面下で以下のようなコードを走らせている

```rust
*(y.deref())
```

* `deref` メソッドが値への参照を返し、`*(y.deref())` のかっこの外の何の変哲もない参照外しがそれでも必要な理由は、 所有権システムのため
* `deref` メソッドが値への参照ではなく、値を直接返したら、値は `self` から外にムーブされてしまう
* 今回の場合や、参照外し演算子を使用する多くの場合には `MyBox<T>` の中の値の所有権を奪いたくはない


### 関数やメソッドで暗黙的な参照外し型強制

**参照外し型強制** というコンパイラの機能がある。
メソッド定義に一致しない **参照型の引数** でメソッドを呼び出した場合、コンパイラが自動で再帰的に `deref` 呼び出しを引数に対して行い、定義と一致する型に自動的に変換する。

前段の `MyBox` を使い、以下のようなコードを書くことができる

```rust
fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
```

* `hello(&m)` の `&m` の型は `&MyBox<String>`
* コンパイラは `deref` を呼び出し、`&MyBox<String>` から `&String` を得る
* `String` は、標準ライブラリにて文字列スライスを返す `Deref` を実装している。したがって、コンパイラは `&String` からさらに `&str` を得る
* このようにして `hello` の定義と一致させることができる

もし参照外し型強制がなかった場合、以下のようなコードになっていた

```rust
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
}
```

### 参照外し型強制が可変性と相互作用する方法

`Deref` トレイトを使用して不変参照に対して `*` をオーバーライドするように、 `DerefMut` トレイトを使用して可変参照の `*` 演算子をオーバーライドできる


以下の3つの場合に型やトレイト実装を見つけた時にコンパイラは、参照外し型強制を行う

* T: `Deref<Target=U>` の時、`&T` から `&U`
* T: `DerefMut<Target=U>` の時、`&mut T` から `&mut U`
* T: `Deref<Target=U>` の時、`&mut T` から `&U`

3つめのケースは巧妙。Rust は可変参照から不変参照への型強制を行う。が、その逆はない


## Drop トレイトで片付け時にコードを走らせる

* `Drop` トレイトを実装することで、値がスコープを抜けるときの動作をカスタマイズすることができる
* ファイルやネットワーク接続などのリソース解放を実装するのにも便利
* `Drop` トレイトは、スマートポインタを実装するときにほぼ常に使われる
* 例えば `Box` は、　`Drop` トレイトを実装して、スコープを外れる際にボックスが指しているヒープの領域を解放する

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created.");
}
```

上記プログラムの実行すると下記の出力を得る

```
CustomSmartPointers created.
Dropping CustomSmartPointer with data `other stuff`!
Dropping CustomSmartPointer with data `my stuff`!
```

### `std::mem::drop` で早期に値をドロップする

* Rust は drop を手動で呼び出すことはできない (error[E0040]: explicit use of destructor method)
* 値をスコープから抜ける前に早期に片付けたいときは `std::mem::drop` を利用することができる
* `std::mem::drop` は初期化処理に含まれているので `drop(val)` と呼び出すことができる

```rust
fn main() {
    let c = CustomSmartPointer { data: String::from("some data") };
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}
```

上記プログラムを実行すると下記の出力を得る

```
CustomSmartPointer created.
Dropping CustomSmartPointer with data `some data`!
CustomSmartPointer dropped before the end of main.
```

### `Rc<T>` は、参照カウント方式のスマートポインタ

TODO
