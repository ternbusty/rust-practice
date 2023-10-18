// コマンドライン引数を Vec で受け取ってみよう (collect でね)
fn main() {
  let args: Vec<String> = std::env::args().collect();
  for s in args {
    println!("{}", s);
  }
}
