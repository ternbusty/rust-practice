// カレントディレクトリのファイル名を表示する
use std::path;

fn main() {
  let target = path::PathBuf::from(".");
  let files = target.read_dir().expect("存在しないパス");
  for dir_entry in files {
    let path = dir_entry.unwrap().path();
    // to_str の出力は Option 型 (None or Some)
    // 正しいファイル名であれば &str を返し、そうでなければ "不正なファイル名" という値が返る
    // expect と違ってパニックにならず値が返るのがポイントっぽい
    let fname = path.to_str().unwrap_or("不正なファイル名");
    println!("{}", fname);
  }
}
