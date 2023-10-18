// cargo run --package chap2 --bin sum_textfile nums.txt
// use std::env; // コマンドライン引数のため
// use std::fs; // ファイル読み込み用
use std::{env, fs}; // 同じモジュールのサブモジュールであれば、まとめて読み込める

fn parse(s: &str) -> f64 {
  match s.trim().parse() {
    Ok(v) => v,
    Err(_) => 0.0,
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    println!("入力ファイルを指定してください");
    return;
  }

  let filename = &args[1];
  let text = fs::read_to_string(filename).unwrap();

  let mut total: f64 = 0.0;
  // 改行で区切って読む
  for line in text.split('\n') {
    total += parse(line);
  }

  println!("{}", total);
}
