use crate::UnimplementedPartTwo;

#[derive(Clone, Copy)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Direction {
    fn rotate(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
}

pub fn solve(s: &str) -> (u32, crate::UnimplementedPartTwo) {
    let (mut maze, mut guard) = parse(s);
    let (ymax, xmax) = (
        (maze.len() - 1) as i32,
        (maze.first().unwrap().len() - 1) as i32,
    );
    loop {
        let (y, x, dir) = guard;
        let (dy, dx) = match dir {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        };
        if (y == 0 && dy == -1)
            || (y == ymax && dy == 1)
            || (x == 0 && dx == -1)
            || (x == xmax && dx == 1)
        {
            break;
        }
        let (newy, newx) = (y + dy, x + dx);
        let newcell = maze
            .get_mut(newy as usize)
            .unwrap()
            .get_mut(newx as usize)
            .unwrap();
        if matches!(newcell, Cell::Obstacle) {
            guard = (y, x, dir.rotate())
        } else {
            *newcell = Cell::Visited;
            guard = (y + dy, x + dx, dir);
        }
    }
    let p1 = maze
        .iter()
        .map(|v| v.iter().filter(|c| matches!(c, Cell::Visited)).count() as u32)
        .sum();
    (p1, UnimplementedPartTwo {})
}

// Maybe we can optimise by, when the guard moves straight ahread, do this in a separate loop?
// For p2, we only need to look at the visited cells in p1.
//

#[derive(Clone, Copy)]
enum Cell {
    Free,
    Obstacle,
    Visited,
}

fn parse(s: &str) -> (Vec<Vec<Cell>>, (i32, i32, Direction)) {
    let mut guard: Option<(i32, i32, Direction)> = None;
    let v: Vec<Vec<Cell>> = s
        .lines()
        .enumerate()
        .map(|(row, line)| {
            let mut v = vec![Cell::Free; line.len()];
            for (col, (&b, e)) in line.as_bytes().iter().zip(v.iter_mut()).enumerate() {
                match b {
                    b'.' => (),
                    b'#' => *e = Cell::Obstacle,
                    b'^' | b'>' | b'v' | b'<' => {
                        let dir = match b {
                            b'^' => Direction::Up,
                            b'>' => Direction::Right,
                            b'v' => Direction::Down,
                            b'<' => Direction::Left,
                            _ => unreachable!(),
                        };
                        *e = Cell::Visited;
                        guard = Some((row as i32, col as i32, dir))
                    }
                    _ => panic!(),
                }
            }
            v
        })
        .collect();
    let fst = v.first().unwrap().len();
    assert!(v.iter().all(|i| i.len() == fst));
    (v, guard.unwrap())
}

#[cfg(test)]
mod tests {
    static TEST_STR: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test() {
        assert_eq!(super::solve(TEST_STR), (41, crate::UnimplementedPartTwo {}));
    }
}
