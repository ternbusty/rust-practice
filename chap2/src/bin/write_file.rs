use std::fs::File;
use std::io::{BufWriter, Write};

fn main() {
  let filename = "output.txt";

  let fp = File::create(filename).unwrap();
  let mut writer = BufWriter::new(fp);
  for i in 1..=100 {
    // TODO: 数値を書き込もうとするとほんとうにこうするしかないのか
    // TODO: それか format!("{}", i); ってするくらいか
    // TODO: バイト形式以外では書き込めないの？
    writer.write(i.to_string().as_bytes()).unwrap();
    writer.write("\n".as_bytes()).unwrap();
  }

  // 一度にまとめて書き込めば事足りるのであれば、write_all が使える
  // fp.write_all(bytes).unwrap();
}
