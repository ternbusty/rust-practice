fn main() {
  let s = "365";
  // unwrap を使うと match 文を省略させられる
  // expect とかでも一緒
  // だが、失敗したときに強制終了することになるので注意
  let i: i32 = s.parse().unwrap();
  println!("{}", i);
}
