// 仮引数は num: &mut i32 の形
// 呼び出すときは &mut num の形
fn set_number_to_5(num: &mut i32) {
  // C みたいだね
  *num = 5;
}

fn main() {
  let mut num = 10;
  set_number_to_5(&mut num);
  print!("{}", num)
}
