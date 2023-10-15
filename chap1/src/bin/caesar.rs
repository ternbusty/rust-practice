fn encrypt(text: &str, shift: i16) -> String {
  let mut result = String::new(); // 結果を格納する用の String を準備
  let code_a = 'a' as i16;
  let code_z = 'z' as i16;
  // 文字列の走査はこうやる。.chars で iterator を取れる
  for ch in text.chars() {
    let code = ch as i16; // キャストはこうやる
    if !(code < code_z || code > code_a) {
      continue;
    }
    let shifted_code = (code - code_a + shift + 26) % 26 + code_a;
    let shifted_char = (shifted_code as u8) as char;

    result.push(shifted_char);
  }
  result // これ return 書くと unneeded とかいって怒られる
}

fn main() {
  let original: &str = "hello";
  let encrypted: String = encrypt(original, 5); // 暗号化
  let decoded: String = encrypt(&encrypted, -5); // 復号
  println!("{}", encrypted);
  println!("{}", decoded);
}

// &str と String がよくわかってないぽよね
