fn main() {
  let nums = vec![1, 2, 3];
  println!("{:?}", nums);

  // 要素数を変更する場合は mut をつけるのかな
  let mut nums2 = Vec::new();
  nums2.push(1);
  nums2.push(3);
  nums2.push(5);
  println!("{:?}", nums2);

  // 文字列を格納する配列
  let s_vec: Vec<&str> = vec!["犬", "猫", "鳥"];
  // ループはこんな感じでできる
  for s in s_vec {
    print!("{} ", s);
  }

  // 配列の要素変更はできるのかな
  // nums[0] = 100; // これは immutable といって怒られる。要素の中身を変更したければ宣言時に mut をつけておくべき
  nums2[2] = 100; // これは通る

  // TODO: s_vec[1] = "ほげ"; これは仮に mut であっても通らない。なんでだろう？
  // TODO: 要素数を指定して初期化はできる？
}
