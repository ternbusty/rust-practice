fn fibonacci(idx: i64) -> i64 {
  if idx == 0 {
    return 1;
  }
  if idx == 1 {
    return 1;
  }
  return fibonacci(idx - 1) + fibonacci(idx - 1);
}

fn main() {
  let n: i64 = 10;
  println!("{}", fibonacci(n));
}
