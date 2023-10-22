// tuple を受け取る関数はこのように書く
fn print_tuple(item: &(&str, i64)) {
  // アクセスはこのようにやる (添え字とかじゃなくて、こうなんだ)
  println!("{}, {}", item.0, item.1);
}

// 構造体でやってみよう
// TODO: なぜ &str ではなく String なのかわからん
struct Item(String, i64);

fn main() {
  let banana = ("バナナ", 300);
  let apple = ("リンゴ", 200);

  print_tuple(&banana);
  print_tuple(&apple);

  // 構造体でやってみよう
  let banana2 = Item("バナナ".to_string(), 300);
  let apple2 = Item("リンゴ".to_string(), 200);
  // Item をベクターに放り込んでみる
  let items = vec![banana2, apple2];
  for item in items {
    println!("{}, {}", item.0, item.1);
  }
}
