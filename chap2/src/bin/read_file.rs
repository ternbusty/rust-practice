// 標準入力で与えられたファイル名の中身を読み込み
// chap2 ディレクトリに入って cargo run --package chap2 --bin read_file test.txt
use std::env; // コマンドライン引数のため
use std::fs; // ファイル読み込み用

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    println!("入力ファイルを指定してください");
    return;
  }

  // TODO: これ args[1] にすると怒られるのなんで？
  let filename = &args[1];
  // ここからがファイルを読む部分
  let text = fs::read_to_string(filename).unwrap();
  println!("{}", text);
}
