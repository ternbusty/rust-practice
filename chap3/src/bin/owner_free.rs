fn main() {
  {
    let s1 = String::from("文字列1");
    let s2 = String::from("文字列2");
    {
      let s3 = s1;
      println!("{}", s3);
    }
    println!("{}", s2);
  }
}
