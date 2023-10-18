// 標準入力から f64 を受け取る関数
fn input(prompt: &str) -> f64 {
  // メッセージを表示
  println!("{}", prompt);
  // 入力を得る
  let mut s = String::new();
  // TODO: expect の使い方復習したい
  std::io::stdin().read_line(&mut s).expect("入力エラー");
  // 空白を除去して数値に変換
  return s.trim().parse().expect("数値変換エラー");
}

// 標準入力の練習
fn main() {
  let height = input("your height (cm): ");
  let weight = input("your weight (kg): ");

  // let bmi = weight / (height / 100 as f64)/ (height / 100 as f64);
  // 浮動小数点の二乗には powf が使える
  let bmi = weight / (height / 100 as f64).powf(2.0);
  // 小数第一位まで表示
  println!("{:.1}", bmi);
}
