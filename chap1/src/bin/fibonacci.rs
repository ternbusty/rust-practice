fn main() {
  let mut a: i64 = 1; // あとで再代入するので mut をつける
  let mut b: i64 = 1;
  print!("{} ", a);
  print!("{} ", b);
  for _ in 1..50 {
    let next: i64 = a + b;
    print!("{} ", next);
    a = b;
    b = next;
  }
}
