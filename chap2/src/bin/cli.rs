// 標準入力ではなくコマンドライン引数を使ってみる
// chap2 ディレクトリに入って cargo run --package chap2 --bin cli 5 7 みたいに実行できる
fn main() {
  // コマンドライン引数はこのように引っ張り出せる
  let args = std::env::args();
  let mut total = 0.0;
  // iterate
  for (i, s) in args.enumerate() {
    if i == 0 {
      // 自身なので skip
      continue;
    }
    let num: f64 = match s.parse() {
      Ok(v) => v,
      Err(_) => 0.0,
    };
    total += num;
  }

  println!("{}", total);
}
