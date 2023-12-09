use pathfinding::prelude::bfs;

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::ffi::OsStr;

fn puzzle() -> i32 {
    return env::args()
        .next()
        .as_ref()
        .map(Path::new)
        .and_then(Path::file_name)
        .and_then(OsStr::to_str)
        .map(String::from)
        .expect("Unknown puzzle number")
        .parse::<i32>()
        .unwrap();
}

type Grid = Vec<Vec<u8>>;

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
struct Coord {
    x: i64,
    y: i64,
}

/*
trait AdjacentExt {
    fn adjacent(&self, pos: Coord) -> Box<dyn Iterator<Item = Coord> + '_>;
}

impl AdjacentExt for Grid {
    fn adjacent(&self, pos: Coord) -> Box<dyn Iterator<Item = Coord> + '_> {
        let v = self.get(pos.y as usize).and_then(|row| row.get(pos.x as usize)).unwrap();

        let next = vec![
            Coord{x: pos.x - 1, y: pos.y},
            Coord{x: pos.x + 1, y: pos.y},
            Coord{x: pos.x, y: pos.y - 1},
            Coord{x: pos.x, y: pos.y + 1},
        ];

        return Box::new(next.into_iter().filter(move |p| {
            //println!("n {:?}", p);
            if let Some(adj) = self.get(p.y as usize).and_then(|row| row.get(p.x as usize)) {
                return adj <= &(v + 1);
            }
            return false;
        }));
    }
}

fn bfs(grid: &Grid, start: Coord, end: Coord) -> Option<Vec<Coord>> {
    let mut visited = HashMap::new();
    let mut queue = VecDeque::new();

    visited.insert(start, start);
    queue.push_back(start);

    while let Some(c) = queue.pop_front() {
        if c == end {
            // Found so we can bail
            break;
        }

        for next in grid.adjacent(c) {
            if !visited.contains_key(&next) {
                visited.insert(next, c);
                queue.push_back(next);
            }
        }
    }

    // Retrace the path
    if visited.contains_key(&end) {
        let mut path = vec![end];

        let mut p = end;
        while p != start {
            p = visited[&p];
            path.push(p);
        }

        return Some(path)
    }

    return None;
}

*/

fn read_grid(filename: &str) -> (Grid, Coord, Coord) {
    let file = File::open(filename).expect("failed to open file");

    let lines = io::BufReader::new(file)
        .lines()
        .map(|line| line.expect("failed to read line"))
        .map(|line| line.trim().to_owned())
        .filter(|line| !line.is_empty());

    let mut start: Option<Coord> = None;
    let mut end: Option<Coord> = None;
    let mut grid = Vec::new();

    let mut y = 0;
    for line in lines {
        grid.push(line.clone().into_bytes());

        if let Some(x) = line.find('S') {
            start = Some(Coord{x: x.try_into().unwrap(), y});
            grid[y as usize][x] = 'a' as u8;
        }
        if let Some(x) = line.find('E') {
            end = Some(Coord{x: x.try_into().unwrap(), y});
            grid[y as usize][x] = 'z' as u8;
        }

        y += 1;
    }


    let start = start.unwrap();
    let end = end.unwrap();

    (grid, start, end)
}

fn solution1(filename: &str) -> i64 {
    let (grid, start, end) = read_grid(filename);

    let successors = |pos: &Coord| {
        let v = grid.get(pos.y as usize).and_then(|row| row.get(pos.x as usize)).unwrap();

        let mut next = vec![
            Coord{x: pos.x - 1, y: pos.y},
            Coord{x: pos.x + 1, y: pos.y},
            Coord{x: pos.x, y: pos.y - 1},
            Coord{x: pos.x, y: pos.y + 1},
        ];

        // Remove directions we can't move to.
        next.retain(|p| {
            if let Some(adj) = grid.get(p.y as usize).and_then(|row| row.get(p.x as usize)) {
                return adj <= &(v + 1);
            }
            return false;
        });

        return next;
    };

    let path = bfs(&start, successors, |p| *p == end).unwrap();
    
    return (path.len() as i64) - 1;
}


fn solution2(filename: &str) -> i64 {

    let (grid, _start, end) = read_grid(filename);

    let successors = |pos: &Coord| {
        let v = grid.get(pos.y as usize).and_then(|row| row.get(pos.x as usize)).unwrap();

        let mut next = vec![
            Coord{x: pos.x - 1, y: pos.y},
            Coord{x: pos.x + 1, y: pos.y},
            Coord{x: pos.x, y: pos.y - 1},
            Coord{x: pos.x, y: pos.y + 1},
        ];

        // Remove directions we can't move to.
        next.retain(|p| {
            if let Some(adj) = grid.get(p.y as usize).and_then(|row| row.get(p.x as usize)) {
                return adj + 1 >= *v;
            }
            return false;
        });

        return next;
    };

    let path = bfs(&end, successors, |p| {
        let v = grid[p.y as usize][p.x as usize];
        return v == ('a' as u8);
    }).unwrap();
    
    return (path.len() as i64) - 1;

}

fn main() {
    let num = puzzle();

    let answer1 = solution1(&format!("{}.txt", num));

    println!("Answer1: {:?}", answer1);
    assert_eq!(answer1, 31);

    let answer2 = solution1(&format!("{}b.txt", num));

    println!("Answer2: {:?}", answer2);
    assert_eq!(answer2, 352);

    let answer3 = solution2(&format!("{}.txt", num));

    println!("Answer3: {:?}", answer3);
    assert_eq!(answer3, 29);

    let answer4 = solution2(&format!("{}b.txt", num));
    println!("Answer4: {:?}", answer4);
    assert_eq!(answer4, 345);
}
