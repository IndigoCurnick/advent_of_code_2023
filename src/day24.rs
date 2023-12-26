use peroxide::fuga::matrix;
use peroxide::fuga::Shape;
use peroxide::prelude::SimplerLinearAlgebra;

use crate::read_lines;

pub fn day24() {
    let path = "data/day24.txt";
    let count = part1(path, 200_000_000_000_000.0, 400_000_000_000_000.0);
    println!("Day 24 Part 1 {}", count);
    let count = part2(path);
    println!("Day 24 Part 2 {}", count);
    // NOTE: On my machine, this outputs 769281292688187.3 for my input
    // The correct solution is 769281292688187
    // Due to the massive size of the input numbers, there is significant floating point error
}

fn part1(path: &str, min: f64, max: f64) -> usize {
    let lines = read_lines(path);
    let hail: Vec<Point> = lines.iter().map(|l| l.into()).collect();

    let mut count = 0;

    for i in 0..hail.len() {
        for j in i + 1..hail.len() {
            if let Some(point) = hail[i].intersect_xy(&hail[j]) {
                match point {
                    Intersection::Parallel => count += 1,
                    Intersection::Point(point) => {
                        if point.x > min
                            && point.x < max
                            && point.y > min
                            && point.y < max
                            && !hail[i].in_past_xy(&point)
                            && !hail[j].in_past_xy(&point)
                        {
                            count += 1;
                        }
                    }
                }
            }
        }
    }

    return count;
}

pub fn part2(path: &str) -> f64 {
    let lines = read_lines(path);
    let hail: Vec<Point> = lines.iter().map(|l| l.into()).collect();

    let mut p0 = hail[0].clone();
    let mut p1 = hail[1].clone();
    let mut p2 = hail[2].clone();

    let centoid = Point {
        x: (p0.x + p1.x + p2.x) / 3.0,
        y: (p0.y + p1.y + p2.y) / 3.0,
        z: (p0.z + p1.z + p2.z) / 3.0,
        vx: 0.0,
        vy: 0.0,
        vz: 0.0,
    };

    p0.recenter_around_origin(&centoid);
    p1.recenter_around_origin(&centoid);
    p2.recenter_around_origin(&centoid);

    let b: Vec<f64> = vec![
        (p0.y * p0.vx - p1.y * p1.vx) - (p0.x * p0.vy - p1.x * p1.vy),
        (p0.y * p0.vx - p2.y * p2.vx) - (p0.x * p0.vy - p2.x * p2.vy),
        (p0.z * p0.vx - p1.z * p1.vx) - (p0.x * p0.vz - p1.x * p1.vz),
        (p0.z * p0.vx - p2.z * p2.vx) - (p0.x * p0.vz - p2.x * p2.vz),
        (p0.z * p0.vy - p1.z * p1.vy) - (p0.y * p0.vz - p1.y * p1.vz),
        (p0.z * p0.vy - p2.z * p2.vy) - (p0.y * p0.vz - p2.y * p2.vz),
    ];

    // let b = matrix(b, 6, 1, Shape::Row);

    let a = vec![
        vec![
            p1.vy - p0.vy,
            p0.vx - p1.vx,
            0.0,
            p0.y - p1.y,
            p1.x - p0.x,
            0.0,
        ],
        vec![
            p2.vy - p0.vy,
            p0.vx - p2.vx,
            0.0,
            p0.y - p2.y,
            p2.x - p0.x,
            0.0,
        ],
        vec![
            p1.vz - p0.vz,
            0.0,
            p0.vx - p1.vx,
            p0.z - p1.z,
            0.0,
            p1.x - p0.x,
        ],
        vec![
            p2.vz - p0.vz,
            0.0,
            p0.vx - p2.vx,
            p0.z - p2.z,
            0.0,
            p2.x - p0.x,
        ],
        vec![
            0.0,
            p1.vz - p0.vz,
            p0.vy - p1.vy,
            0.0,
            p0.z - p1.z,
            p1.y - p0.y,
        ],
        vec![
            0.0,
            p2.vz - p0.vz,
            p0.vy - p2.vy,
            0.0,
            p0.z - p2.z,
            p2.y - p0.y,
        ],
    ];

    let a = matrix(a.concat(), 6, 6, Shape::Row);
    let r = a.solve(&b);

    let sum = r[0] + centoid.x + r[1] + centoid.y + r[2] + centoid.z;

    return sum;
}

#[derive(Clone)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}

impl From<&String> for Point {
    fn from(value: &String) -> Self {
        let split: Vec<&str> = value.split(" @ ").collect();

        let mut position_split = split[0].split(", ");
        let x = position_split.next().unwrap().trim().parse().unwrap();
        let y = position_split.next().unwrap().trim().parse().unwrap();
        let z = position_split.next().unwrap().trim().parse().unwrap();

        let mut velocity_split = split[1].split(", ");
        let vx = velocity_split.next().unwrap().trim().parse().unwrap();
        let vy = velocity_split.next().unwrap().trim().parse().unwrap();
        let vz = velocity_split.next().unwrap().trim().parse().unwrap();

        return Self {
            x,
            y,
            z,
            vx,
            vy,
            vz,
        };
    }
}

impl Point {
    fn slope_intercept(&self) -> (f64, f64) {
        let slope_self = self.vy / self.vx;
        let intercept_self = self.y - slope_self * self.x;

        return (slope_self, intercept_self);
    }

    fn intersect_xy(&self, other: &Point) -> Option<Intersection> {
        let (slope_self, intercept_self) = self.slope_intercept();
        let (slope_other, intercept_other) = other.slope_intercept();

        if slope_self == slope_other && intercept_self == intercept_other {
            return Some(Intersection::Parallel);
        } else if slope_self == slope_other {
            return None;
        }

        // Solve for x and y.
        let x = (intercept_other - intercept_self) / (slope_self - slope_other);
        let y = slope_self * x + intercept_self;
        return Some(Intersection::Point(Point {
            x,
            y,
            z: 0.0,
            vx: 0.0,
            vy: 0.0,
            vz: 0.0,
        }));
    }

    fn in_past_xy(&self, point: &Point) -> bool {
        let x = point.x - self.x;
        let y = point.y - self.y;

        let x = x / self.vx;
        let y = y / self.vy;

        return x < 0.0 && y < 0.0;
    }

    fn recenter_around_origin(&mut self, centroid: &Point) {
        self.x -= centroid.x;
        self.y -= centroid.y;
        self.z -= centroid.z;
    }
}

enum Intersection {
    Point(Point),
    Parallel,
}

#[test]
fn test_part1() {
    let path = "data_demo/day24_demo.txt";
    let count = part1(path, 7.0, 27.0);
    assert_eq!(count, 2);
}

#[test]
fn test_part2() {
    let path = "data_demo/day24_demo.txt";
    let count = part2(path);
    assert_eq!(count, 47.0);
}
