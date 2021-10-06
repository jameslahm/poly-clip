use serde::{Deserialize, Serialize};
use std::cmp::{max, min, Ordering};

#[derive(Clone, Debug, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Debug, Copy)]
enum Vertex {
    In(Point),
    Out(Point),
    InInter(Point),
    OutInter(Point),
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

impl From<Vertex> for Point {
    fn from(v: Vertex) -> Self {
        match v {
            Vertex::In(p) => p,
            Vertex::Out(p) => p,
            Vertex::InInter(p) => p,
            Vertex::OutInter(p) => p,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Line {
    pub begin: Point,
    pub end: Point,
}

impl Line {
    pub fn generate_inter_vertex_in_lines(self, line: &Line) -> Option<Point> {
        let (p0, p1) = (self.begin, self.end);
        let (p2, p3) = (line.begin, line.end);
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

// The first Vec<Point> is outer ring
// The rest Vec<Point> is inner rings
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Polygon {
    pub rings: Vec<Vec<Point>>,
}

#[derive(Clone, Debug)]
enum ClipRes {
    In(Vec<Vec<Point>>),
    Inter(Vec<Vec<Vertex>>),
}

impl Vertex {
    fn get_first_in_inter(list: &mut Vec<Vertex>) -> Option<Point> {
        let mut result = None;
        let mut index: i32 = -1;
        for (i, v) in list.iter().enumerate() {
            if matches!(*v, Vertex::InInter(_)) {
                index = i as i32;
                result = Some((*v).into());
                break;
            }
        }
        if index > 0 {
            list.drain(0..(index as usize));
        }
        result
    }
}

fn generate_clip_polygons(mut primary: Vec<Vertex>, mut clip: Vec<Vertex>) -> Vec<Vec<Point>> {
    let mut result: Vec<Vec<Point>> = Vec::new();
    while let Some(initial) = Vertex::get_first_in_inter(&mut primary) {
        result.push(generate_clip_polygon(
            &mut primary,
            &mut clip,
            initial,
        ));
    }
    result
}

fn generate_clip_polygon(
    primary: &mut Vec<Vertex>,
    clip: &mut Vec<Vertex>,
    initial: Point,
) -> Vec<Point> {
    let mut result: Vec<Point> = Vec::new();

    let mut in_primary_list = true;
    let mut begin = initial.clone();
    let mut end = (*primary.last().unwrap()).into();
    while initial != end {
        if let Some((mut points, last_point)) =
            walk_in_list(if in_primary_list { primary } else { clip }, begin)
        {
            end = last_point;
            begin = last_point;
            in_primary_list = !in_primary_list;
            result.append(&mut points);
        }
    }
    result
}

fn walk_in_list(list: &mut Vec<Vertex>, initial: Point) -> Option<(Vec<Point>, Point)> {
    let mut initial_found = (Point::from(*(list.first().unwrap()))) == initial;
    let mut begin_index = 0;
    let mut end_index = 0;
    let mut last_point: Option<Point> = None;
    let points: Vec<Point> = list
        .iter()
        .enumerate()
        .skip_while(|(index, _) | {
            if initial_found {
                return false;
            };
            let next_index = if *index == list.len() - 1 { 0 } else { index + 1 };

            let next_point = list[next_index];
            match next_point {
                Vertex::In(_) | Vertex::Out(_) => {}
                Vertex::InInter(_) | Vertex::OutInter(_) => {
                    if Point::from(next_point) == initial {
                        begin_index = next_index;
                        initial_found = true;
                    }
                    return true;
                }
            }
            !initial_found
        })
        .take_while(|(index, &vertex)| {
            if matches!(vertex, Vertex::OutInter(_)) {
                end_index = *index;
                last_point = Some(vertex.into());
                return false;
            }
            true
        })
        .map(|(_, &vertex)| {
            vertex.into()
        })
        .collect();
    list.drain(begin_index..end_index+1);
    if !points.is_empty() {
        Some((points, last_point.unwrap()))
    } else {
        None
    }
}

impl Polygon {
    pub fn new(rings: Vec<Vec<Point>>) -> Self {
        Polygon { rings }
    }

    pub fn contains(self: &Polygon, point: &Point) -> bool {
        let mut count = 0;
        for points in self.rings.iter() {
            for (i, _) in points.iter().enumerate() {
                let p0 = points[i];
                let p1 = points[(i + 1) % points.len()];
                if p0.y == p1.y {
                    continue;
                };
                if point.y < min(p0.y, p1.y) || point.y >= max(p0.y, p1.y) {
                    continue;
                }
                let inter_x = (point.y - p0.y) * (p1.x - p0.x) / (p1.y - p0.y) + p0.x;
                if inter_x > point.x {
                    count += 1;
                }
            }
        }
        return count % 2 == 1;
    }

    fn generate_inter_vertexs_in_line(&self, line: &Line, is_enter: &mut bool) -> Vec<Vertex> {
        let mut lines: Vec<Point> = vec![];
        for points in self.rings.iter() {
            let mut intersects: Vec<Point> = points
                .iter()
                .enumerate()
                .filter_map(|x| {
                    let (index, begin) = x;
                    let next_index = if index == points.len() - 1 { 0 } else { index + 1 };
                    let end = points[next_index];
                    let l = Line {
                        begin: *begin,
                        end,
                    };
                    if let Some(point) = l.generate_inter_vertex_in_lines(line) {
                        // Dont consider begin and end
                        if point == *begin || point == end || point == line.begin || point == line.end  {
                            None
                        } else {
                            Some(point)
                        }
                    } else {
                        None
                    }
                })
                .collect();
            lines.append(&mut intersects);
        }
        // Sort
        lines.sort_by(|p1, p2| {
            let p0 = line.begin;
            let dst1 = (p0.x - p1.x).pow(2) + (p0.y - p1.y).pow(2);
            let dst2 = (p0.x - p2.x).pow(2) + (p0.y - p2.y).pow(2);

            if dst1 == dst2 {
                Ordering::Equal
            } else if dst1 > dst2 {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        });
        lines
            .iter()
            .map(|x| {
                if *is_enter {
                    *is_enter = false;
                    Vertex::OutInter(*x)
                } else {
                    *is_enter = true;
                    Vertex::InInter(*x)
                }
            })
            .collect()
    }

    fn generate_inter_vertexs(&self, poly: &Polygon) -> ClipRes {
        let mut result = vec![];
        // Special Check #1
        if self.rings.first().unwrap().iter().position(|ref x| poly.contains(x)).is_none() {
            if poly.rings.iter().flatten().all(|x| self.contains(x)) {
                return ClipRes::In(poly.rings.clone());
            }
        };
        // Special Check #2
        // for primary in self.rings.iter() {
        //     if primary.iter().position(|ref x| !poly.contains(x)).is_none() {
        //         return ClipRes::In(self.rings.clone());
        //     }
        // }
        // generate vertexs
        self.rings.iter().for_each(|primary| {
            let mut is_enter = false;
            if let Some(first_out_index) = primary.iter().position(|ref x| !poly.contains(x)) {
                let points = primary
                    .iter()
                    .enumerate()
                    .skip(first_out_index)
                    .chain(primary.iter().enumerate().take(first_out_index))
                    .fold(vec![], |mut res, (index, begin)| {
                        if poly.contains(begin) {
                            res.push(Vertex::In(*begin));
                        } else {
                            res.push(Vertex::Out(*begin));
                        }

                        let next_index = if index == primary.len() - 1 {
                            0
                        } else {
                            index + 1
                        };

                        let end = primary[next_index].clone();
                        let line = Line {
                            begin: *begin,
                            end: end
                        };
                        res.append(&mut poly.generate_inter_vertexs_in_line(&line, &mut is_enter));
                        res
                    });
                if points
                    .iter()
                    .find(|x| match **x {
                        Vertex::InInter(_) | Vertex::OutInter(_) => true,
                        Vertex::In(_) | Vertex::Out(_) => false,
                    })
                    .is_some()
                {
                    result.push(points);
                }
            }
        });
        ClipRes::Inter(result)
    }

    pub fn clip(&self, other: &Polygon) -> Vec<Vec<Point>> {
        let res1 = self.generate_inter_vertexs(other);
        let res2 = other.generate_inter_vertexs(self);
        match res1 {
            ClipRes::In(list) => list,
            ClipRes::Inter(primary_list) => match res2 {
                ClipRes::Inter(clip_list) => {
                    let list1 = primary_list.into_iter().flatten().collect();
                    let list2 = clip_list.into_iter().flatten().collect();
                    generate_clip_polygons(list1, list2)
                }
                ClipRes::In(list) => list,
            },
        }
    }
}
