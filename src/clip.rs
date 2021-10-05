use serde::{Deserialize, Serialize};
use std::cmp::{max, min, Ordering};

#[derive(Clone, Debug, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

#[derive(Clone, Debug)]
pub struct Line {
    pub start: Point,
    pub end: Point,
}

// The first Vec<Point> is outer ring
// The rest Vec<Point> is inner rings
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Polygon {
    pub rings: Vec<Vec<Point>>,
}

#[derive(Clone, Debug)]
enum PolyListOption {
    List(Vec<Vec<InterVertex>>),
    InsidePoly(Vec<Vec<Point>>),
    None,
}

#[derive(Clone, Debug, Copy)]
enum InterVertex {
    InsideVertex(Point),
    OutsideVertex(Point),
    InIntersection(Point),
    OutIntersection(Point),
}

impl InterVertex {
    fn get_point(&self) -> Point {
        match *self {
            InterVertex::InIntersection(ref p) => p.clone(),
            InterVertex::OutIntersection(ref p) => p.clone(),
            InterVertex::InsideVertex(ref p) => p.clone(),
            InterVertex::OutsideVertex(ref p) => p.clone(),
        }
    }

    fn get_first_in_intersection(list: &mut Vec<InterVertex>) -> Option<Point> {
        let mut found = 0;
        let mut result = None;
        if let Some(p) = list.iter().enumerate().find(|x| {
            let (i, x) = *x;
            if let InterVertex::InIntersection(_) = *x {
                found = i;
                return true;
            }
            false
        }) {
            result = Some(p.1.get_point());
        };
        if found > 0 {
            for _ in 0..found {
                list.remove(0);
            }
        }
        result
    }
}

impl Point {
    pub fn is_in_polygon(&self, poly: &Polygon) -> bool {
        let mut count = 0;
        for points in poly.rings.iter() {
            for (i, _) in points.iter().enumerate() {
                let p0 = points[i];
                let p1 = points[(i + 1) % points.len()];
                if p0.y == p1.y {
                    continue;
                };
                if self.y < min(p0.y, p1.y) || self.y >= max(p0.y, p1.y) {
                    continue;
                }
                let inter_x = (self.y - p0.y) * (p1.x - p0.x) / (p1.y - p0.y) + p0.x;
                if inter_x > self.x {
                    count += 1;
                }
            }
        }
        return count % 2 == 1;
    }

    fn distance_cmp(&self, first: &Point, second: &Point) -> Ordering {
        let dst_first = (self.x - first.x).abs() + (self.y - first.y).abs();
        let dst_second = (self.x - second.x).abs() + (self.y - second.y).abs();

        if dst_first < dst_second {
            Ordering::Less
        } else if dst_first > dst_second {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

fn get_first_outside_vertex_index(points: &Vec<Point>, poly: &Polygon) -> Option<usize> {
    points.iter().position(|ref x| !x.is_in_polygon(poly))
}

fn get_first_inside_vertex_index(points: &Vec<Point>, poly: &Polygon) -> Option<usize> {
    points.iter().position(|ref x| x.is_in_polygon(poly))
}

impl Polygon {
    pub fn clip(&self, other: &Polygon) -> Option<Vec<Vec<Point>>> {
        let option = self.get_inter_vertex_list(other);
        let other_option = other.get_inter_vertex_list(self);
        match option {
            PolyListOption::List(subject_list) => match other_option {
                PolyListOption::List(clip_list) => {
                    let list1 = subject_list.into_iter().flatten().collect();
                    let list2 = clip_list.into_iter().flatten().collect();
                    Polygon::get_clip_polygons(list1, list2)
                }
                PolyListOption::InsidePoly(list) => Some(list),
                PolyListOption::None => None,
            },
            PolyListOption::InsidePoly(list) => Some(list),
            PolyListOption::None => None,
        }
    }

    fn get_clip_polygons(
        mut subject: Vec<InterVertex>,
        mut clip: Vec<InterVertex>,
    ) -> Option<Vec<Vec<Point>>> {
        let mut result: Vec<Vec<Point>> = Vec::new();
        while let Some(start_point) = InterVertex::get_first_in_intersection(&mut subject) {
            if let Some(poly) =
                Polygon::get_clip_polygon(&mut subject, &mut clip, start_point.clone())
            {
                result.push(poly);
            } else {
                break;
            }
        }
        if result.len() > 0 {
            Some(result)
        } else {
            None
        }
    }

    fn get_clip_polygon(
        subject: &mut Vec<InterVertex>,
        clip: &mut Vec<InterVertex>,
        initial: Point,
    ) -> Option<Vec<Point>> {
        let mut result: Vec<Point> = Vec::new();

        let mut subject_as_list = true;
        let mut start_point = initial.clone();
        let mut end_point = subject[subject.len() - 1].clone().get_point();
        while initial != end_point {
            if let Some(values) = Polygon::collect_from_list(
                if subject_as_list { subject } else { clip },
                start_point,
            ) {
                let (mut edges, end) = values;
                end_point = end.clone();
                start_point = end.clone();
                if subject_as_list {
                    subject_as_list = false;
                } else {
                    subject_as_list = true;
                }
                result.append(&mut edges);
            } else {
                println!("something went wrong");
                println!("res {:?}", result);
                return None;
            }
        }
        if result.len() > 0 {
            result = result
                .iter()
                .enumerate()
                .filter_map(|x| {
                    let (i, x) = x;
                    let next = if i == result.len() - 1 { 0 } else { i + 1 };
                    let next_point = &result[next];
                    if *next_point == *x {
                        None
                    } else {
                        Some(*x)
                    }
                })
                .collect();
            Some(result)
        } else {
            None
        }
    }

    fn collect_from_list(
        list: &mut Vec<InterVertex>,
        start_point: Point,
    ) -> Option<(Vec<Point>, Point)> {
        let mut initial_vertex_not_found = true;
        let mut last_point: Option<Point> = None;
        let (mut start_i, mut end_i) = (0, 0);
        let dont_skip = list[0].get_point() == start_point;
        let points: Vec<Point> = list
            .iter()
            .enumerate()
            .skip_while(|x| {
                // need to skip until InIntersection occurs,
                // but include the InIntersection
                if dont_skip || !initial_vertex_not_found {
                    return false;
                };
                let (i, _) = *x;
                let next = if i == list.len() - 1 { 0 } else { i + 1 };

                let next_point = &list[next];
                match next_point {
                    &InterVertex::InIntersection(_) | &InterVertex::OutIntersection(_) => {
                        if next_point.get_point() == start_point {
                            start_i = next;
                            initial_vertex_not_found = false;
                            return true;
                        }
                        return true;
                    }
                    &InterVertex::InsideVertex(_) | &InterVertex::OutsideVertex(_) => {}
                }
                initial_vertex_not_found
            })
            .take_while(|x| {
                let (i, x) = *x;

                if let InterVertex::OutIntersection(ref p) = *x {
                    end_i = i;
                    last_point = Some(p.clone());
                    return false;
                }
                true
            })
            .map(|x| {
                let (_, x) = x;

                x.get_point()
            })
            .collect();
        let amount = end_i - start_i + 1;
        for _ in 0..amount {
            list.remove(start_i);
        }
        if points.len() > 0 {
            Some((points, last_point.unwrap()))
        } else {
            None
        }
    }

    fn get_inter_vertex_list(&self, poly: &Polygon) -> PolyListOption {
        let mut res = vec![];
        for (i, points) in self.rings.iter().enumerate() {
            let subject = points.clone();
            let mut cursor_inside = if i == 0 { false } else { false };
            if let Some(start_index) = get_first_outside_vertex_index(&subject, poly) {
                if let None = get_first_inside_vertex_index(&subject, poly) {
                    if poly.rings.iter().flatten().all(|x| x.is_in_polygon(self)) {
                        return PolyListOption::InsidePoly(poly.rings.clone());
                    }
                };
                let result = subject
                    .iter()
                    .enumerate()
                    .skip(start_index)
                    .chain(subject.iter().enumerate().take(start_index))
                    .fold(Vec::new(), |mut acc, x| {
                        let (i, start) = x;

                        // check vertex
                        if i != start_index && start.is_in_polygon(poly) {
                            acc.push(InterVertex::InsideVertex(start.clone()));
                        } else {
                            acc.push(InterVertex::OutsideVertex(start.clone()));
                        }

                        // check intersection
                        let next_i = if i == subject.len() - 1 { 0 } else { i + 1 };

                        let end = subject[next_i].clone();
                        let line = Line {
                            start: start.clone(),
                            end,
                        };
                        let mut intersections =
                            poly.get_intersections_with_line(&line, &mut cursor_inside);
                        acc.append(&mut intersections);
                        acc
                    });
                // Check if there are any intersection
                if result
                    .iter()
                    .find(|x| match **x {
                        InterVertex::InsideVertex(_) | InterVertex::OutsideVertex(_) => false,
                        InterVertex::InIntersection(_) | InterVertex::OutIntersection(_) => true,
                    })
                    .is_none()
                {
                    // res.push(PolyListOption::None)
                } else {
                    res.push(result);
                }
            } else {
                return PolyListOption::InsidePoly(self.rings.clone());
            }
        }
        PolyListOption::List(res)
    }

    fn get_intersections_with_line(
        &self,
        line: &Line,
        cursor_inside: &mut bool,
    ) -> Vec<InterVertex> {
        let mut lines: Vec<Point> = vec![];
        for points in self.rings.iter() {
            let mut line:Vec<Point> =
                points
                    .iter()
                    .enumerate()
                    .filter_map(|x| {
                        let (i, start) = x;
                        let next_i = if i == points.len() - 1 { 0 } else { i + 1 };
                        let end = points[next_i].clone();
                        let l = Line {
                            start: start.clone(),
                            end,
                        };
                        if let Some(p) = l.get_intersection(line) {
                            if p == line.start || p == line.end || p == *start || p == end {
                                None
                            } else {
                                Some(p)
                            }
                        } else {
                            None
                        }
                    })
                    .collect();
            lines.append(&mut line);
        }
        lines.sort_by(|a, b| line.start.distance_cmp(a, b));
        lines
            .iter()
            .map(|x| {
                if *cursor_inside {
                    *cursor_inside = !*cursor_inside;
                    InterVertex::OutIntersection(x.clone())
                } else {
                    *cursor_inside = !*cursor_inside;
                    InterVertex::InIntersection(x.clone())
                }
            })
            .collect()
    }

    pub fn new(rings: Vec<Vec<Point>>) -> Self {
        Polygon { rings }
    }
}
impl Line {
    pub fn get_intersection(self, line: &Line) -> Option<Point> {
        let (p0, p1) = (self.start, self.end);
        let (p2, p3) = (line.start, line.end);
        // ax + by = e
        // cx + dy = f
        let a = -(p0.y - p1.y);
        let b = p0.x - p1.x;

        let c = -(p2.y - p3.y);
        let d = p2.x - p3.x;

        let e = a * p0.x + b * p0.y;
        let f = c * p2.x + d * p2.y;

        // x = l1 / l
        // y = l2 / l

        let l1 = e * d - b * f;
        let l2 = a * f - e * c;
        let l = a * d - b * c;

        if l == 0 {
            return None;
        }

        let x = l1 / l;
        let y = l2 / l;

        let line1_min_x = min(p0.x, p1.x);
        let line1_max_x = max(p0.x, p1.x);
        let line1_min_y = min(p0.y, p1.y);
        let line1_max_y = max(p0.y, p1.y);

        let line2_min_x = min(p2.x, p3.x);
        let line2_max_x = max(p2.x, p3.x);
        let line2_min_y = min(p2.y, p3.y);
        let line2_max_y = max(p2.y, p3.y);

        let x_in_bound =
            x >= line1_min_x && x <= line1_max_x && x >= line2_min_x && x <= line2_max_x;
        let y_in_bound =
            y >= line1_min_y && y <= line1_max_y && y >= line2_min_y && y <= line2_max_y;

        if x_in_bound && y_in_bound {
            return Some(Point::new(x, y));
        } else {
            return None;
        }
    }
}
