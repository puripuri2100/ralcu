# ralcu

高機能電卓です。

## 実装されている機能

- 値
  - 整数値
  - 浮動小数点数
  - 真偽値
  - 関数
- 2項演算子
  - 四則演算
    - 整数用：`+`・`-`・`*`・`/`・`>`・`<`
    - 小数用：`+.`・`-.`・`*.`・`/.`・`>.`・`<.`
    - 真偽値用：`&&`・`||`
- 条件分岐
  - `if true then 5 else 6`のように使う
- 関数
  - `sin: float -> float`
  - `cos: float -> float`
  - `float: int -> float`
  - `int: float -> int`
- 変数・関数定義
  - OCaml likeな構文である
  - `let x = e1 in e2`という形で定義する
  - 関数の場合は`let f x y = x + y in e2`のように引数を取ればよい
  - `fun x1 x2 .. xlast = x1 + x2 in e2`のように無名関数を定義することができる
- 関数適用
  - 部分適用ができる
  - `let f x y = x + y in let g y = f 2 in g 3`


詳しくはtestフォルダ以下に様々な構文の例が書かれているため、それを参照してほしい。

## 実行方法

```sh
cargo run -- --type text "let x = 5 in x"
```

のように、文字列を与えることで結果を得ることができる。

```sh
cargo run -- --type file "path/to/file"
```

のようにすることで、ファイルに書かれたコードを評価して結果を得ることができる。


## build・test方法

Rustとそのパッケージマネージャを要求する。Rustの最低バージョンは`1.64.0`である。

```sh
cargo build
```

でビルドができる。

```sh
cargo test
```

でテストができる。


## 構成

### 字句解析器

[`src/lexer.rs`](src/lexer.rs)において実装されている。

先頭から一文字ずつ読み、用意されているパターンの先頭にマッチした場合に追加で読み進め、値を生成するようになっている。

### 構文解析器

[`src/parser.rs`](src/parser.rs)において実装されている。

LL(1)の構文解析器である。

このコードは[`llmaker`](https://github.com/puripuri2100/llmaker)という自作のパーサージェネレータを用いて、[`src/parser.mkr`](src/parser.mkr)から自動で生成されたのちに、[`parser.patch`](src/parser.patch)ファイルを適用して若干の修正が施されている。

parser.mkrファイルは、BNF likeな構文定義文と、抽象構文木を作成するためのRustコードからなっている。


### 評価器

[`src/eval.rs`](src/eval.rs)において実装されている。

`eval_exp`関数で、再帰的に評価を行っている。

事前に用意されている関数や二項演算子は`def_primitive`で定義されている。




---

(c) Naoki Kaneko
[MIT Licnese](https://github.com/puripuri2100/ralcu/blob/master/LICENSE)
