use rand::Rng;

fn main() {
  // ここ usize にしないと怒られる (i32 とかだとダメ)
  // あと let ではなく const にしないとダメ
  const MAP_N: usize = 25;
  // 固定長の二次元配列は以下のように宣言する
  let mut maze: [[i32; 25]; 25] = [[0; MAP_N]; MAP_N];

  // 外周を壁にする
  for i in 0..MAP_N {
    maze[0][i] = 1;
    maze[i][0] = 1;
    maze[MAP_N - 1][i] = 1;
    maze[i][MAP_N - 1] = 1;
  }

  // 2 マスに 1 つ壁を配置する
  for y in 2..(MAP_N - 2) {
    for x in 2..(MAP_N - 2) {
      if x % 2 == 1 || y % 2 == 1 {
        continue;
      }
      maze[y][x] = 1;
      // 上下左右のいずれかを壁にする
      let mut rng = rand::thread_rng();
      let r: i32 = rng.gen_range(0..=3);
      // switch 的なのはこうやって書く
      match r {
        0 => maze[y - 1][x] = 1,
        1 => maze[y + 1][x] = 1,
        2 => maze[y][x - 1] = 1,
        3 => maze[y][x + 1] = 1,
        _ => unreachable!(), // これ書いとかないと non-exaustive pattern とかいって怒られるので
      }
    }
  }

  for y in 0..MAP_N {
    for x in 0..MAP_N {
      print!("{:2}", if maze[x][y] == 1 { 'Z' } else { ' ' });
    }
    println!();
  }
}
