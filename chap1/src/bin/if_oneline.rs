fn main() {
  let n = 5;
  // Python の三項演算子的なやつとは微妙に順序が違うので注意
  let judge = if n % 2 == 0 { "偶数" } else { "奇数" };
  println!("{}", judge)
}
