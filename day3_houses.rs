use std::collections::HashMap;
use std::env;

fn new_pos(pos: (i32, i32), c: char) -> (i32,i32) {
  match c {
    '>' => (pos.0 + 1, pos.1),
    '<' => (pos.0 - 1, pos.1),
    '^' => (pos.0, pos.1 + 1),
    'v' => (pos.0, pos.1 - 1),
    _ => (0,0)
  }
}

fn visit_house(position: (i32, i32), direction: char, visited: &mut HashMap<(i32, i32),i32>) -> (i32, i32) {
  let new_position = new_pos(position, direction);
  let new_el = match visited.get(&new_position) {
      Some(a) => a + 1,
      None => 1
  };
  visited.insert(new_position, new_el);
  new_position
}

fn main() {
  let mut visited = HashMap::new();
  let mut turn = 1;
  let mut santa_pos = (0,0);
  let mut robo_pos  = (0,0);

  visited.insert(santa_pos, 1);

  let args: Vec<String> = env::args().collect();

  for c in args[1].chars() {
    if turn % 2 == 0 {
      santa_pos = visit_house(santa_pos, c, &mut visited);
    } else {
      robo_pos = visit_house(robo_pos, c, &mut visited);
    }
    turn = turn + 1;
    ()
  }



  println!("{:?}", visited.len());

}
