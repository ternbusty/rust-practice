// 標準入力から文字列を受け取る
fn input_str() -> String {
  // 入力文字列で上書きするので mut をつけている
  let mut s = String::new();
  // TODO: この引数が &mut s になるのよくわかってない
  // read_line 関数は Result 型の文字列を返す
  // Result 型は Ok か Err のいずれかを返す。ここでは特に match とか書いてないので、
  // Err の場合は expect の内容を表示して強制終了
  std::io::stdin().read_line(&mut s).expect("入力エラー");
  s.trim_end().to_string()
}

// 標準入力からf64 を受け取る (一旦文字列で受け取ってから f64 に変換している)
fn input_f(default: f64) -> f64 {
  let s = input_str();
  // parse メソッドは Result 型を返すので、
  // match 文で処理。Err の場合は 0 を返すようにしている
  match s.trim().parse() {
    Ok(v) => v,
    Err(_) => default,
  }
}

fn main() {
  let mut height;
  // loop は while true と同義だが、非推奨なので loop で書いている
  loop {
    println!("身長は");
    height = input_f(0.0);
    if height > 0.0 {
      break;
    };
    println!("入力が不正です");
  }
}
