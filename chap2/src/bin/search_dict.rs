// ファイルをまとめて読み込むのではなく、一行ずつ読み込むのをやってみる
// cargo run --package chap2 --bin search_dict coworker
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() < 2 {
    println!("単語を入力してください");
    return;
  }
  let filename = "ejdict-hand-utf8.txt";
  let word = &args[1];

  // ファイルポインタを取得
  let fp = File::open(filename).unwrap();
  // BufReader で一行ずつ読む
  let reader = BufReader::new(fp);
  for line in reader.lines() {
    // こうすれば Result から値を引っ張り出せる
    let line = line.unwrap();
    // 文字列の検索こんな感じでできるのね。find は Option 型を返すっぽい
    if line.find(word) != None {
      println!("{}", line);
      return;
    }
  }

  println!("Not Found");
}
