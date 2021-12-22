use core::panic;
use std::fmt::Debug;
use std::io;
use std::io::prelude::*;
use std::ops::Add;
use std::ops::Sub;
use std::str::FromStr;

fn parse_input<Input, Type>(input: Input) -> Type
where
    Type: FromStr,
    <Type as FromStr>::Err: Debug,
    Input: AsRef<str>,
{
    input.as_ref().trim().parse().unwrap()
}

#[derive(Debug, Clone, PartialEq, Eq, Copy, Hash, Default)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn new(x: i64, y: i64, z: i64) -> Point {
        Point { x, y, z }
    }

    fn constrain_min(&self, min_bound: &Point) -> Point {
        Point::new(
            std::cmp::max(min_bound.x, self.x),
            std::cmp::max(min_bound.y, self.y),
            std::cmp::max(min_bound.z, self.z),
        )
    }

    fn constrain_max(&self, max_bound: &Point) -> Point {
        Point::new(
            std::cmp::min(max_bound.x, self.x),
            std::cmp::min(max_bound.y, self.y),
            std::cmp::min(max_bound.z, self.z),
        )
    }
}
impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
fn input_coord_to_values(s: &str) -> (i64, i64) {
    let (_, b) = s.split_once('=').unwrap();
    let (p1, p2) = b.split_once("..").unwrap();
    (parse_input(p1), parse_input(p2))
}

fn parse_input_line(s: &str) -> (bool, (Point, Point)) {
    let (st, coord) = s.split_once(" ").unwrap();
    let status = st == "on";
    let coords = coord
        .split(",")
        .map(|x| input_coord_to_values(x))
        .collect::<Vec<_>>();
    let start_point = Point::new(coords[0].0, coords[1].0, coords[2].0);
    let end_point = Point::new(coords[0].1, coords[1].1, coords[2].1);

    (status, (start_point, end_point))
}
fn volume(p1: &Point, p2: &Point) -> i64 {
    let dx = (p2.x - p1.x).abs() + 1;
    let dy = (p2.y - p1.y).abs() + 1;
    let dz = (p2.z - p1.z).abs() + 1;
    return dx * dy * dz;
}

fn overlaps(pa_origin: &Point, pa_opp: &Point, pb_origin: &Point, pb_opposite: &Point) -> bool {
    if pb_opposite.x < pa_origin.x || pb_opposite.y < pa_origin.y || pb_opposite.z < pa_origin.z {
        return false;
    }
    if pb_origin.x > pa_opp.x || pb_origin.y > pa_opp.y || pb_origin.z > pa_opp.z {
        return false;
    }
    true
}
#[derive(Debug)]
enum CubeTreeNodeType {
    All(bool),
    Split(Vec<std::rc::Rc<std::cell::RefCell<CubeTree>>>),
    Single((bool, bool, Point, Point)),
}

#[derive(Debug)]
struct CubeTree {
    origin: Point,
    opposite: Point,
    children: CubeTreeNodeType,
}

impl CubeTree {
    fn new(origin: Point, opposite: Point, status: bool) -> CubeTree {
        if origin.x > opposite.x || origin.y > opposite.y || origin.z > opposite.z {
            panic!("Invalid cube {:?} {:?}", origin, opposite);
        }
        CubeTree {
            origin,
            opposite,
            children: CubeTreeNodeType::All(status),
        }
    }

    //split into (up to) 27 cubes with the target being the central one
    fn calc_children(
        &mut self,
        start_point: &Point,
        end_point: &Point,
        my_status: bool,
        new_status: bool,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<CubeTree>>> {
        let mut v = Vec::new();

        for x_pos in &[
            (self.origin.x, start_point.x - 1),
            (start_point.x, end_point.x),
            (end_point.x + 1, self.opposite.x),
        ] {
            if x_pos.1 < x_pos.0 {
                continue;
            }
            for y_pos in &[
                (self.origin.y, start_point.y - 1),
                (start_point.y, end_point.y),
                (end_point.y + 1, self.opposite.y),
            ] {
                if y_pos.1 < y_pos.0 {
                    continue;
                }
                for z_pos in &[
                    (self.origin.z, start_point.z - 1),
                    (start_point.z, end_point.z),
                    (end_point.z + 1, self.opposite.z),
                ] {
                    if z_pos.1 < z_pos.0 {
                        continue;
                    }
                    let new_origin = Point::new(x_pos.0, y_pos.0, z_pos.0);
                    let new_oppos = Point::new(x_pos.1, y_pos.1, z_pos.1);
                    let status = if new_origin == *start_point && new_oppos == *end_point {
                        new_status
                    } else {
                        my_status
                    };
                    v.push(std::rc::Rc::new(std::cell::RefCell::new(CubeTree::new(
                        new_origin, new_oppos, status,
                    ))));
                }
            }
        }
        v
    }

    //classic octree: split into 8 evenly sized cubes and then update them with the target details
    fn calc_children_octree(
        &mut self,
        start_point: &Point,
        end_point: &Point,
        my_status: bool,
        new_status: bool,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<CubeTree>>> {
        let mut v = Vec::new();
        let mut midp = self.origin + self.opposite;
        midp.x /= 2;
        midp.y /= 2;

        midp.z /= 2;
        for i in 0..8 {
            let mut origin = self.origin;
            let mut opposite = self.opposite;
            if (i & 1) > 0 {
                origin.x = midp.x + 1;
            } else {
                opposite.x = midp.x;
            }
            if (i & 2) > 0 {
                origin.y = midp.y + 1;
            } else {
                opposite.y = midp.y;
            }
            if (i & 4) > 0 {
                origin.z = midp.z + 1
            } else {
                opposite.z = midp.z;
            }
            if origin.z > opposite.z || origin.y > opposite.y || origin.x > opposite.x {
                continue;
            }
            v.push(std::rc::Rc::new(std::cell::RefCell::new(CubeTree::new(
                origin, opposite, my_status,
            ))));
        }
        for x in v.iter() {
            let mut ln = x.as_ref().borrow_mut();
            ln.update(new_status, &start_point, &end_point, 0);
        }
        v
    }

    //octree with cube split - split into up to 8 cubes with either the first cube being the target or the 8th cube fully containing the target and starting at it's origin
    fn calc_children_octree_split_on_cube(
        &mut self,
        start_point: &Point,
        end_point: &Point,
        my_status: bool,
        new_status: bool,
    ) -> Vec<std::rc::Rc<std::cell::RefCell<CubeTree>>> {
        let mut v = Vec::new();
        let midp;
        if self.origin == *start_point {
            midp = *end_point;
        } else {
            midp = *start_point - Point::new(1, 1, 1); //bounds are inclusive and we want the entire cube to be included in the 8th cube
        }
        for i in 0..8 {
            let mut origin = self.origin;
            let mut opposite = self.opposite;
            if (i & 1) > 0 {
                origin.x = midp.x + 1;
            } else {
                opposite.x = midp.x;
            }
            if (i & 2) > 0 {
                origin.y = midp.y + 1;
            } else {
                opposite.y = midp.y;
            }
            if (i & 4) > 0 {
                origin.z = midp.z + 1
            } else {
                opposite.z = midp.z;
            }
            if origin.z > opposite.z || origin.y > opposite.y || origin.x > opposite.x {
                continue;
            }
            v.push(std::rc::Rc::new(std::cell::RefCell::new(CubeTree::new(
                origin, opposite, my_status,
            ))));
        }
        for x in v.iter() {
            let mut ln = x.as_ref().borrow_mut();
            ln.update(new_status, &start_point, &end_point, 0);
        }
        v
    }

    fn update(&mut self, new_status: bool, start_point: &Point, end_point: &Point, depth: i64) {
        if overlaps(&self.origin, &self.opposite, start_point, end_point) == false {
            return;
        }
        let start_point = start_point.constrain_min(&self.origin);
        let end_point = end_point.constrain_max(&self.opposite);

        if start_point == self.origin && end_point == self.opposite {
            self.children = CubeTreeNodeType::All(new_status);
            return;
        }

        match self.children {
            CubeTreeNodeType::All(stat) => {
                if stat == new_status {
                    return;
                }
                self.children =
                    CubeTreeNodeType::Single((stat, new_status, start_point, end_point));
            }
            CubeTreeNodeType::Single((
                outer_status,
                inner_status,
                inner_origin,
                inner_opposite,
            )) => {
                let tmp: Vec<_> = self.calc_children_octree_split_on_cube(
                    &inner_origin,
                    &inner_opposite,
                    outer_status,
                    inner_status,
                );

                for x in tmp.iter() {
                    let mut ln = x.as_ref().borrow_mut();
                    ln.update(new_status, &start_point, &end_point, depth + 1);
                }
                self.children = CubeTreeNodeType::Split(tmp);
            }

            CubeTreeNodeType::Split(ref children) => {
                for x in children.iter() {
                    let mut ln = x.as_ref().borrow_mut();
                    ln.update(new_status, &start_point, &end_point, depth + 1);
                }
            }
        }
    }

    fn count(&self) -> i64 {
        match self.children {
            CubeTreeNodeType::All(stat) => {
                if stat {
                    return volume(&self.opposite, &self.origin);
                }
                return 0;
            }
            CubeTreeNodeType::Single((
                outer_status,
                inner_status,
                inner_origin,
                inner_opposite,
            )) => {
                let inner_vol = volume(&inner_opposite, &inner_origin);
                let outer_vol = volume(&self.opposite, &self.origin);

                if inner_status && outer_status {
                    return outer_vol;
                } else if inner_status {
                    return inner_vol;
                } else if outer_status {
                    return outer_vol - inner_vol;
                } else {
                    return 0;
                }
            }

            CubeTreeNodeType::Split(ref children) => {
                let mut count = 0;
                for x in children.iter() {
                    let ln = x.as_ref().borrow();
                    count += ln.count();
                }

                return count;
            }
        }
    }
}

fn calc_on_tree(
    values: &Vec<(bool, (Point, Point))>,
    bound_min: Point,
    bound_max: Point,
    min_point: Point,
) -> i64 {
    let mut this_root = CubeTree::new(bound_min, bound_max, false);
    for (status, (start, end)) in values.iter() {
        let start_point_positive = (*start - min_point).constrain_min(&bound_min);
        let end_point_positive = (*end - min_point).constrain_max(&bound_max);

        this_root.update(*status, &start_point_positive, &end_point_positive, 0);
    }
    this_root.count()
}

fn main() {
    let stdin = io::stdin();
    let values = stdin
        .lock()
        .lines()
        .map(|input| {
            let s = input.unwrap();
            parse_input_line(&s)
        })
        .collect::<Vec<_>>();

    let minx = values.iter().map(|(_, (sp, _))| sp.x).min().unwrap() - 1;
    let miny = values.iter().map(|(_, (sp, _))| sp.y).min().unwrap() - 1;
    let minz = values.iter().map(|(_, (sp, _))| sp.z).min().unwrap() - 1;

    let maxx = values.iter().map(|(_, (_, ep))| ep.x).max().unwrap() + 1;
    let maxy = values.iter().map(|(_, (_, ep))| ep.y).max().unwrap() + 1;
    let maxz = values.iter().map(|(_, (_, ep))| ep.z).max().unwrap() + 1;
    let mp = Point::new(minx, miny, minz);
    let max_point = Point::new(maxx, maxy, maxz) - mp;

    let init_min_bound = Point::new(-50, -50, -50);
    let init_max_bound = Point::new(50, 50, 50);
    println!("{:?} {:?}", mp, max_point);
    println!(
        "{:?}",
        calc_on_tree(&values, init_min_bound - mp, init_max_bound - mp, mp)
    );
    println!("{:?}", calc_on_tree(&values, mp, max_point, mp));
}
