// クロージャ (ラムダ式) を使って書き直してみる
fn encrypt(text: &str, shift: i16) -> String {
  let a: i16 = 'a' as i16;
  // 判定する関数
  // || 内に引数を入れて、そのあとに返り値を書く
  let is_valid = |c: char| 'a' <= c && c <= 'z';
  // 変換する関数
  let conv = |c: char| (((c as i16 - a + shift + 26) % 26 + a) as u8) as char;
  // 判定して変換する関数
  let enc = |c: char| if is_valid(c) { conv(c) } else { c };
  return text.chars().map(|c: char| enc(c)).collect();
}

fn main() {
  let original: &str = "hello";
  let encrypted: String = encrypt(original, 5); // 暗号化
  let decoded: String = encrypt(&encrypted, -5); // 復号
  println!("{}", encrypted);
  println!("{}", decoded);
}
