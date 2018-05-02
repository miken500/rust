#[derive(PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
pub fn spiral_matrix(n: usize) -> Vec<Vec<u32>> {
    if n == 0 { return vec![] }
    let mut v = vec![vec![0u32;n];n];
    let mut dir = Dir::Right;
    let mut loc = (0,0);
    for i in 1..n*n+1 {
        v[loc.1][loc.0] = i as u32;
        if dir == Dir::Right && (loc.0 == n-1 || v[loc.1][loc.0+1] > 0) {
            dir = Dir::Down;
        }
        else if dir == Dir::Down && (loc.1 == n-1 || v[loc.1+1][loc.0] > 0) {
            dir = Dir::Left;
        }
        else if dir == Dir::Left && (loc.0 == 0 || v[loc.1][loc.0-1] > 0) {
            dir = Dir::Up;
        }
        else if dir == Dir::Up && (loc.1 == 0 || v[loc.1-1][loc.0] > 0) {
            dir = Dir::Right;
        }
        match dir {
            Dir::Right => loc.0 += 1,
            Dir::Left => loc.0 -= 1,
            Dir::Down => loc.1 += 1,
            Dir::Up => loc.1 -= 1,
        }
    }
    v
}
