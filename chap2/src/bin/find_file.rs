// cargo run --package chap2 --bin find_file . .rs
use std::{env, path};

fn find_file(target: &path::PathBuf, keyword: &str) {
  // read_dir の返り値は Result<ReadDir>
  let files = target.read_dir().expect("存在しないパス");
  // dir_entry は Result<DirEntry, Error>
  for dir_entry in files {
    // PathBuf から path を引っ張り出している
    let path = dir_entry.unwrap().path();
    // ディレクトリであれば再帰
    if path.is_dir() {
      find_file(&path, keyword);
      continue;
    }
    // ファイルであれば、PathBuf からファイル名を引っ張り出している
    let fname = path.file_name().unwrap().to_string_lossy();
    if fname.find(keyword) == None {
      continue;
    }
    // to_string_lossy は、Unicode で表現できない文字を削除して、
    // U+FFFD に変換するメソッド
    println!("{}", path.to_string_lossy());
  }
}

fn main() {
  let args: Vec<String> = env::args().collect();

  if args.len() < 3 {
    println!("findfile (path) (keyword)");
    return;
  }

  // TODO: これ args[1] だと怒られるのに &args だと良いのよくわかってない
  let target_dir = &args[1];
  let keyword = &args[2];
  // 検索パスを PathBuf に変換
  let target = path::PathBuf::from(target_dir);
  find_file(&target, keyword);
}
