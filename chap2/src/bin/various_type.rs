fn main() {
  let a = 100u8;
  let b = 100i128; // i128 型の 100
  let c = 10_000; // 10000 と同じ意味

  let c1 = 'a'; // char
  let c2 = b'a'; // u8 (文字コード)
  let c3 = '\x61'; // 16 進数の char

  let d1 = c1 as u32;
  let d2 = '\u{611b}'; // 16 進数の「愛」

  // 文字と 16 進数の対応はここで確認できる、面白い
  // https://glyphwiki.org/wiki/u5ca9
  let e = '\u{5ca9}';
  print!("{}", e);

  // 16 進数など
  let v1 = 0xFF;
  let v2 = 0x655;
  let v3 = 0b1101_0101;
  println!("{}, {}, {}", v1, v2, v3); // 255, 1621, 213

  // 浮動小数展型の精度指定
  let f3 = 10.5f32;
  let f4 = 10.5e+8; // 10.5 x 10^8 の意。競プロでよくあるやつ
}
