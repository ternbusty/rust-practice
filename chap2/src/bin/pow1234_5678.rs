use num_bigint::BigInt;

fn main() {
  // base と結果格納用の変数が BigInt であればよい (exponent は i32 でよい)
  let base = BigInt::from(1234);
  let exponent = 5678;
  let result = base.pow(exponent);
  println!("{}^{} = {}", base, exponent, result);
}
