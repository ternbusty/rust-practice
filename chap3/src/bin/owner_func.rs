fn show_message_err(message: String) {
  println!("{}", message);
}

// 返り値を利用して所有権を呼び出し元に戻す
fn show_message_corrected(message: String) -> String {
  println!("{}", message);
  return message;
}

// 借用 (そもそも所有権を関数に渡さない)
fn show_message_borrow(message: &String) {
  println!("{}", message);
}

fn main() {
  let s1 = String::from("hoge1");
  // ここで所有権が移動
  show_message_err(s1);
  // なので下の println はエラーになる
  // println!("{}", s);

  // このように、返り値を用いて所有権を呼び出し元へ返却するようにすれば OK
  let mut s2 = String::from("hoge2");
  s2 = show_message_corrected(s2);
  println!("{}", s2);

  // 借用を用いてみる
  let s3 = String::from("hoge3");
  show_message_borrow(&s3);
  println!("{}", s3);
}
