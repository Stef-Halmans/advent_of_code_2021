use crate::Challange;
use std::cmp;
use std::collections::HashMap;

#[derive(Debug)]
struct LineSegment {
    point1: Point,
    point2: Point,
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn create_point(input: &str) -> Point {
        let point_array: Vec<i32> = input
            .split(",")
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.parse::<i32>().expect("error"))
            .collect();

        Point {
            x: point_array[0],
            y: point_array[1],
        }
    }
}

trait IntoLineSegment {
    fn into(self) -> LineSegment;
}

impl LineSegment {
    fn new<A>(args: A) -> LineSegment
    where
        A: IntoLineSegment,
    {
        args.into()
    }

    fn same_x(&self) -> bool {
        self.point1.x == self.point2.x
    }

    fn same_y(&self) -> bool {
        self.point1.y == self.point2.y
    }

    fn smallest_x(&self) -> i32 {
        cmp::min(self.point1.x, self.point2.x)
    }

    fn smallest_y(&self) -> i32 {
        cmp::min(self.point1.y, self.point2.y)
    }

    fn biggest_x(&self) -> i32 {
        cmp::max(self.point1.x, self.point2.x)
    }

    fn biggest_y(&self) -> i32 {
        cmp::max(self.point1.y, self.point2.y)
    }
}

impl IntoLineSegment for (Point, Point) {
    fn into(self) -> LineSegment {
        LineSegment {
            point1: self.0,
            point2: self.1,
        }
    }
}

impl IntoLineSegment for &str {
    fn into(self) -> LineSegment {
        let points: Vec<&str> = self.split(" -> ").collect();
        LineSegment::new((
            Point::create_point(points[0]),
            Point::create_point(points[1]),
        ))
    }
}

pub fn run_challange(input_data: &str, challange: Challange) {
    let line_segments = create_line_segments(input_data);

    let amount_dangerous_points = calculate_points_on_lines(
        &line_segments,
        match challange {
            Challange::One => false,
            Challange::Two => true,
        },
    );

    println!("dangerous points: {}", amount_dangerous_points);
}

fn create_line_segments(input: &str) -> Vec<LineSegment> {
    let lines: Vec<&str> = input.lines().collect();
    let mut line_segments: Vec<LineSegment> = vec![];

    for line in lines {
        line_segments.push(LineSegment::new(line));
    }

    println!("line segment 1: {:?}", line_segments[0]);
    println!("line segment 2: {:?}", line_segments[1]);
    println!("last line segment: {:?}", line_segments.last().unwrap());

    line_segments
}

fn calculate_points_on_lines(line_segments: &[LineSegment], include_diagonals: bool) -> u32 {
    let mut points: HashMap<String, i32> = HashMap::new();

    for line_segment in line_segments {
        //horizontal line
        if line_segment.same_y() {
            for i in line_segment.smallest_x()..line_segment.biggest_x() + 1 {
                let mut key: String = i.to_string();
                key.push(',');
                key.push_str(&line_segment.point1.y.to_string());

                if points.contains_key(&key) {
                    *points.get_mut(&key).unwrap() += 1;
                } else {
                    points.insert(key, 1);
                }
            }
        }
        //vertical line
        if line_segment.same_x() {
            for i in line_segment.smallest_y()..line_segment.biggest_y() + 1 {
                let mut key: String = line_segment.point1.x.to_string();
                key.push(',');
                key.push_str(&i.to_string());

                if points.contains_key(&key) {
                    *points.get_mut(&key).unwrap() += 1;
                } else {
                    points.insert(key, 1);
                }
            }
        }
        //diagonal line
        if !line_segment.same_x() && !line_segment.same_y() && include_diagonals {}
    }

    let mut points_with_overlap = 0;

    for point in points.values() {
        if *point >= 2 {
            points_with_overlap += 1;
        }
    }

    points_with_overlap
}
