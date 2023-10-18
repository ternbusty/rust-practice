use rand::seq::SliceRandom;

fn print_bingo(nums: Vec<i32>) {
  for y in 0..5 {
    for x in 0..5 {
      print!("{:3} ", nums[y * 5 + x]);
    }
    println!();
  }
}

fn main() {
  let mut nums = Vec::new();
  for i in 0..75 {
    nums.push(i);
  }

  let mut rng = rand::thread_rng();
  nums.shuffle(&mut rng);
  print_bingo(nums);
}
