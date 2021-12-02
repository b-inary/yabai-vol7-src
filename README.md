# yabai-vol7-src

[yabaitech.tokyo vol.7](https://yabaitech.tokyo/c99/2021/12/19/C99.html) に投稿した記事「不完全情報ゲームのナッシュ均衡を CFR (Counterfactual Regret Minimization) アルゴリズムで求めよう」で紹介したプログラムです。

```sh
$ # 【5.3節】Kuhn poker の解析を実行
$ cargo run --release

$ # 【6.2節】プリフロップにおける勝率テーブルを生成
$ # (staticフォルダにある計算済みのものと同一のファイルを生成できます)
$ cargo run --release --bin gen-table

$ # 【6.3節】プッシュ/フォールドの解析を実行
$ cargo run --release --bin push-fold
```

## ファイル構成

【srcディレクトリ】
- [`interface.rs`](src/interface.rs): 【4.1節】ゲームのインターフェース定義です。
- [`cfr.rs`](src/cfr.rs): 【4.2節】CFRアルゴリズム本体の実装です。
- [`util.rs`](src/util.rs): 【4.3節】ユーティリティ関数の実装です。
- [`game_kuhn.rs`](src/game_kuhn.rs): 【5.2節】Kuhn poker のゲーム定義の実装です。
- [`main_kuhn.rs`](src/main_kuhn.rs): 【5.3節】Kuhn poker の解析を行う `main()` 関数の実装です。
- [`game_push_fold.rs`](src/game_push_fold.rs): 【6.2節】プッシュ/フォールドのゲーム定義の実装です。
- [`main_gen_table.rs`](src/main_gen_table.rs): 【6.2節】プリフロップにおける勝率テーブルを生成するプログラムです。
- [`main_push_fold.rs`](src/main_push_fold.rs): 【6.3節】プッシュ/フォールドの解析を行う `main()` 関数の実装です。

【staticディレクトリ】
- [`headsup_preflop_equity.bin`](static/headsup_preflop_equity.bin): [`game_push_fold.rs`](src/game_push_fold.rs) の `evaluate()` で参照される勝率テーブルです。
  [`main_gen_table.rs`](src/main_gen_table.rs) によって生成することができます（筆者の16スレッドマシンで15分程度を要します）。
