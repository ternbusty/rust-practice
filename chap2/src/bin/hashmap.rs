// Hashmap を使う練習
use std::collections::HashMap;

fn main() {
  const V_DATA: &str = "C,C,C,A,B,A,C";
  // HashMap を使ってみる
  let mut c_map: HashMap<&str, i32> = HashMap::new();
  for w in V_DATA.split(',') {
    // 存在確認する時は get を使うが、これは Option 型を返す
    // Option 型は、値がないときは None, あるときは Some を返す
    match c_map.get(w) {
      None => c_map.insert(w, 1),
      Some(_) => c_map.insert(w, c_map[w] + 1), // 存在が確認されていたら [] でアクセス可能
    };
  }

  // このようにすれば key を iterate できる
  // TODO: 引数の型はよくわからない
  for (key, value) in &c_map {
    println!("{}: {}", key, value);
  }
}
