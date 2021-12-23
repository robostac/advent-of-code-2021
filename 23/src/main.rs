use std::collections::*;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::io;
use std::io::prelude::*;
use std::ops::Add;
use std::ops::Sub;

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, Default, PartialOrd, Ord)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    const fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }
}
impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialOrd, Ord)]
struct Layout {
    pods: [Point; 16],
    energy: i64,
    finished: i64,
    valid_y: [i64; 4],
}

impl Hash for Layout {
    fn hash<H: Hasher>(&self, state: &mut H) {
        //don't worry about energy usage when checking if states are equal
        self.pods.hash(state);
    }
}

impl PartialEq for Layout {
    fn eq(&self, other: &Self) -> bool {
        //don't worry about energy usage when checking if states are equal
        self.pods == other.pods
    }
}
impl Eq for Layout {}

impl Layout {
    fn new() -> Layout {
        Default::default()
    }

    fn make_move(&mut self, pod: usize, dest: &Point, cost: i64) -> bool {
        self.pods[pod] = *dest;
        self.energy += cost;
        self.update_finished();
        true
    }

    fn is_finished(&self) -> bool {
        self.finished == 16
    }

    fn update_finished(&mut self) {
        self.finished = 0;
        for ptype in 0..4 {
            self.valid_y[ptype] = 5;
            for y in (2..=5).rev() {
                let mut valid = false;
                for i in 0..4 {
                    let p = self.pods[ptype * 4 + i];
                    if p.x == COLUMNS[ptype] && p.y == y {
                        self.finished += 1;
                        self.valid_y[ptype] = y - 1;
                        valid = true;
                        break;
                    }
                }
                if valid == false {
                    break;
                }
            }
        }
    }
}

const DIRECTIONS: [Point; 4] = [
    Point::new(0, 1),
    Point::new(1, 0),
    Point::new(-1, 0),
    Point::new(0, -1),
];
const COLUMNS: [i64; 4] = [3, 5, 7, 9];
const ENERGY: [i64; 4] = [1, 10, 100, 1000];

fn find_possible(grid: &HashSet<Point>, layout: &Layout, idx: usize) -> Vec<(Point, i64)> {
    let mut v = Vec::new();
    let podtype = idx / 4;
    let dest_col = COLUMNS[podtype];
    if layout.pods[idx].x == dest_col && layout.pods[idx].y > layout.valid_y[podtype] {
        return v;
    }

    let start = layout.pods[idx];
    let mut current = VecDeque::new();
    current.push_back((start, 0));
    let mut visited: HashSet<Point> = layout.pods.iter().cloned().collect(); //mark all pods as visited as we can't move onto those squares
    while let Some((p, e)) = current.pop_front() {
        for d in DIRECTIONS.iter() {
            if d.y == 1 && dest_col != p.x {
                //can't go down unless we're going into our room
                continue;
            }
            let np = p + *d;
            if grid.contains(&np) == false || visited.contains(&np) {
                //check this move is valid and not somewhere we've been already
                continue;
            }
            let en = e + ENERGY[podtype];
            current.push_back((np, en));
            if np.x == dest_col && np.y == layout.valid_y[podtype] {
                //if we can move to a valid room then thats all we want to do
                return vec![(np, en)];
            }
            if start.y == 1 && np.y == 1 {
                //can't stop in corridor if started in corridor
            } else if np.y > 1 && (np.y != layout.valid_y[podtype]) {
                //can't stop in room until we reach bottom
            } else if np.y == 1 && COLUMNS.contains(&np.x) {
                //can't stop on junctions
            } else {
                //valid place to stop so add to list
                v.push((np, en));
            }
            visited.insert(np);
        }
    }
    v
}

fn find_best(pods: &Layout, grid: &HashSet<Point>) -> i64 {
    let mut states = VecDeque::new();
    states.push_back(pods.clone());
    let mut best = std::i64::MAX;
    let mut visited_states = HashMap::new();
    while let Some(st) = states.pop_front() {
        if st.energy >= *visited_states.get(&st).unwrap_or(&std::i64::MAX) {
            continue;
        }
        if st.energy >= best {
            continue;
        }
        visited_states.insert(st, st.energy);
        for pod in 0..st.pods.len() {
            for z in find_possible(&grid, &st, pod) {
                let mut p = st.clone();
                if p.make_move(pod, &z.0, z.1) {
                    if p.is_finished() {
                        best = std::cmp::min(best, p.energy);
                    } else {
                        states.push_back(p);
                    }
                }
            }
        }
    }
    return best;
}

fn main() {
    let stdin = io::stdin();
    let values = stdin
        .lock()
        .lines()
        .map(|input| input.unwrap())
        .collect::<Vec<_>>();

    let mut grid = HashSet::new();

    let mut amphipods = Layout::new();
    for (y, s) in values.iter().enumerate() {
        for (x, v) in s.chars().enumerate() {
            let p = Point::new(x as i64, y as i64);
            match v {
                '.' => {
                    grid.insert(p);
                }
                'A' => {
                    grid.insert(p);
                    if amphipods.pods[0] == Point::new(0, 0) {
                        amphipods.pods[0] = p;
                    } else {
                        amphipods.pods[1] = p;
                    }
                }
                'B' => {
                    grid.insert(p);
                    if amphipods.pods[4] == Point::new(0, 0) {
                        amphipods.pods[4] = p;
                    } else {
                        amphipods.pods[5] = p;
                    }
                }
                'C' => {
                    grid.insert(p);
                    if amphipods.pods[8] == Point::new(0, 0) {
                        amphipods.pods[8] = p;
                    } else {
                        amphipods.pods[9] = p;
                    }
                }
                'D' => {
                    grid.insert(p);
                    if amphipods.pods[12] == Point::new(0, 0) {
                        amphipods.pods[12] = p;
                    } else {
                        amphipods.pods[13] = p;
                    }
                }
                _ => {}
            }
        }
    }

    let amphipods = amphipods;
    let mut amphipods_p1 = amphipods.clone();
    for y in 0..2 {
        for (i, x) in COLUMNS.iter().enumerate() {
            grid.insert(Point::new(*x, y + 4));
            amphipods_p1.pods[(i * 4) + 2 + y as usize] = Point::new(*x, y + 4);
        }
    }
    amphipods_p1.update_finished();

    println!("{:?}", find_best(&amphipods_p1, &grid));

    let mut amphipods_p2 = amphipods.clone();
    for x in amphipods_p2.pods.iter_mut() {
        if x.y == 3 {
            x.y = 5;
        }
    }

    let line_idx = [9, 7, 5, 3];
    let line_idx2 = [7, 5, 9, 3];
    for i in 0..4 {
        amphipods_p2.pods[i * 4 + 2] = Point::new(line_idx[i], 3);
        amphipods_p2.pods[i * 4 + 3] = Point::new(line_idx2[i], 4);
    }
    amphipods_p2.update_finished();

    println!("{:?}", find_best(&amphipods_p2, &grid));
}
