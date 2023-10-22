// 戻り値を参照にしたいとき、以下ではライフタイムの問題でエラーになる
// fn gen_message() -> &str {
//   let msg = String::from("hoge");
//   return &msg;
// }

// 参照ではなく実体で返す
// 新規作成して返したいなら実体を返り値として返すべきだし、
// 引数で渡したものを変更して欲しいなら、ミュータブル参照を渡して void にすべき
fn gen_message() -> String {
  let msg = String::from("hoge");
  return msg;
}

fn change_message(msg: &mut String) {
  // TODO: これにデリファレンスが不要な理由がわからん
  // 先頭に " を追加
  msg.insert(0, '"');
  // 末尾に " を追加
  msg.push('"');
}

// 引数を 2 倍にする関数
fn x2(num: &mut i32) {
  // デリファレンスが必要
  *num *= 2;
}

fn main() {
  // 関数側で新規作成してもらう
  let m = gen_message();
  println!("{}", m);

  // 作ってあるものを関数側で変えてもらう
  let mut mutable_m = String::from("hoge");
  // 参照を渡しており所有権は移動しない
  change_message(&mut mutable_m);
  // ので、怒られない
  println!("{}", mutable_m);

  // 練習: 引数を 2 倍にする
  let mut num1 = 10;
  x2(&mut num1);
  println!("{}", num1);
}
