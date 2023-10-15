fn is_prime(num: i32) -> bool {
  for i in 2..num {
    if num % i == 0 {
      return false;
    }
  }
  true
}

// [i32; 100] というのは、長さ 100 の i32 型の配列という意味の型 annotation
// & で参照渡しであるということ、mut は mutable であることを表す
// 長さを指定せずに渡すことはできないのかな？
fn get_prime(primes: &mut [i32; 100]) {
  let mut cnt = 0;
  let mut current: i32 = 2;
  while cnt < 100 {
    if is_prime(current) {
      primes[cnt] = current;
      cnt += 1;
    }
    current += 1;
  }
}

fn main() {
  // [0; 100] というのは、0 が初期値、要素数 100 の配列ということ。これで配列を初期化できる
  let mut primes: [i32; 100] = [0; 100];
  // 渡すときも &mut を付ける必要がある
  get_prime(&mut primes);
  // println! で {:?} を指定するとデータ型をわかりやすく表示できて便利らしい
  println!("{:?}", primes);
}
