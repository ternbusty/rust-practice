fn main() {
  let g1 = String::from("テスト文字列");
  let g2 = g1;
  println!("{}", g2);
  // 以下は怒られが発生する、所有権がすでに移動した後なので
  // println!("{}", g1);
  // .clone すれば所有権の移動なしで使える
  let g3 = g2.clone();
  println!("{}", g2);
  println!("{}", g3);
}
