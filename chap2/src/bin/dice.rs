use rand::Rng;

fn main() {
  let mut rng = rand::thread_rng();
  let n: u32 = rng.gen_range(1..=6);
  println!("dice: {}", n);
}
