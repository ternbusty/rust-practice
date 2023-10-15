fn main() {
  for i in 1..100 {
    if i % 15 == 0 {
      // if の条件部分では、カッコを書くと怒られる
      println!("fizzbuzz") // 逆に if のブロックではたとえ 1 行であろうと {} を書かないと怒られる
    } else if i % 5 == 0 {
      println!("fizz")
    } else if i % 3 == 0 {
      println!("buzz")
    } else {
      println!("{}", i)
    };
  }
}
