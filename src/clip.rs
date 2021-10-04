use  std::{cmp, vec};


use crate::clip;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Vertex {
    x:i32,
    y:i32
}

impl Vertex {
    pub fn new(x:i32, y: i32) -> Self{
        Vertex {
            x,
            y
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum DegreeType {
    In,
    Out
}

#[derive(Clone, Copy, PartialEq)]
pub struct Intersect {
    degree: DegreeType,
    x: i32,
    y: i32,
    l1: usize,
    l2: usize,
    dis: i32
}

impl Intersect {
    pub fn new(v: Vertex, l1:usize, l2:usize) -> Self {
        Intersect {
            x: v.x,
            y: v.y,
            degree:DegreeType::In,
            l1: l1,
            l2: l2,
            dis: 0
        }
    }
}


#[derive(Debug, Clone)]
pub struct Polygon {
    pub points: Vec<Vertex>
}

impl Polygon {
    pub fn new(points: Vec<Vertex>) -> Self {
        Polygon {
            points: points
        }
    }
}

// This is point, with point and in or out, and intersect info
#[derive(PartialEq, Clone, Copy)]
pub enum Point {
    Intersect(Intersect),
    Vertex(Vertex)
}

impl Vertex {
    pub fn from_point(point:Point) -> Vertex {
        match point {
            Point::Vertex(v) => {v}
            Point::Intersect(inter) => {
                Vertex::new(inter.x, inter.y)
            }
        }
    }
}

pub fn intersect_line(p0: Vertex, p1:Vertex, p2:Vertex, p3:Vertex)-> Option<Vertex> {
    // ax+by = e
    // cx + dy = f
    let a = -(p0.y - p1.y);
    let b = p0.x - p1.y;

    let c = -(p2.y - p3.y);
    let d = p2.x - p3.x;

    let e = a*p0.x + b*p0.y;
    let f = c*p2.x + d*p2.y;

    // x = l1 / l
    // y = l2 / l

    let l1 = e * d - b*f;
    let l2 = a*f - e*c;
    let l = a*d - b*c;

    if l==0 {
        return None;
    }

    let x = l1 / l;
    let y = l2 / l;

    let line1_min_x = cmp::min(p0.x, p1.x);
    let line1_max_x = cmp::max(p0.x, p1.x);
    let line1_min_y = cmp::min(p0.y,p1.y);
    let line1_max_y  =cmp::max(p0.y, p1.y);

    let line2_min_x = cmp::min(p2.x, p3.x);
    let line2_max_x = cmp::max(p2.x, p3.x);
    let line2_min_y = cmp::min(p2.y, p3.y);
    let line2_max_y = cmp::max(p2.y, p3.y);

    let x_in_bound =  x >= line1_min_x && x <= line1_max_x &&
        x >= line2_min_x && x <= line2_max_x;
    let y_in_bound = y>= line1_min_y && y<= line1_max_y &&
        y >= line2_min_y && y<= line2_max_y;

    if x_in_bound && y_in_bound {
        return Some(Vertex::new(x, y));
    } else {
        return None;
    }

}

fn distance(x1:i32, y1:i32,x2:i32,y2:i32) -> i32 {
    return (x2 - x1) * (x2 - x1) + (y2-y1) * (y2-y1);
}


impl Polygon {
    // self is clip polygon,
    pub fn clip(self: &Self, primary_polygon: &Polygon) -> Vec<Polygon> {
        let intersect_points = self.intersect(primary_polygon);

        let mut primary_list = primary_polygon.generate_list(intersect_points.clone(), 1);
        let mut clip_list  = self.generate_list(intersect_points, 0);

        self.generate_degree(&mut primary_list);

        self.generate_clip_degree(&mut clip_list, &mut primary_list);

        return self.generate_area(&mut primary_list, &mut clip_list);
    }

    pub fn generate_area(self: &Self, primary_list:&mut Vec<Point>, clip_list:&mut Vec<Point>) -> Vec<Polygon> {
        let mut polygons = vec![];
        let mut polgon = Polygon::new(vec![]);
        let mut index = 0;
        for _ in primary_list.iter() {
            let point = primary_list[index];
            match point {
                Point::Intersect(inter)=>{
                    if matches!(inter.degree, DegreeType::In) {
                        break;
                    }
                }
                _ => {}
            }
            index+=1;
        }
        loop {
            if index== primary_list.len() {
                break;
            }
            polgon.points.push(Vertex::from_point(primary_list[index]));
            index+=1;

            while index < primary_list.len() {
                match primary_list[index] {
                    Point::Intersect(inter)=>{
                        if !matches!(inter.degree, DegreeType::In) {
                            break;
                        }
                    }
                    Point::Vertex(v) => {

                    }
                }
                polgon.points.push(Vertex::from_point(primary_list[index]));
                index+=1;
            }

            let mut index1 = 0;
            while index1 < clip_list.len() {
                if clip_list[index1] == primary_list[index] {
                    break;
                }
                index1+=1;
            }

            while index1 < clip_list.len() {
                match clip_list[index1] {
                    Point::Intersect(inter)=>{
                        if matches!(inter.degree, DegreeType::In) {
                            break;
                        }
                    }
                    Point::Vertex(v) => {

                    }
                }
                polgon.points.push(Vertex::from_point(clip_list[index]));
                index1+=1;
            }

            if polgon.points[0] == Vertex::from_point(clip_list[index1]) {
                polygons.push(polgon.clone());
                polgon.points.clear();
                while index < primary_list.len() {
                    match primary_list[index] {
                        Point::Intersect(inter)=>{
                            if matches!(inter.degree, DegreeType::In) {
                                break;
                            }
                        }
                        Point::Vertex(v) => {

                        }
                    }
                    polgon.points.push(Vertex::from_point(primary_list[index]));
                    index+=1;
                }
                continue;
            }
            while index < primary_list.len() {
                if primary_list[index]==clip_list[index1] {
                    break;
                }
                index +=1;
            }
        }
        return polygons;
    }

    // arrow line
    pub fn contains(self:&Self, x:i32,y:i32) -> bool {
        let mut count = 0;
        for (i,_) in self.points.iter().enumerate() {
            let p0 = self.points[i];
            let p1 = self.points[(i+1) % self.points.len()];
            if p0.y == p1.y {
                continue
            };
            if y < cmp::min(p0.y,p1.y) || y >= cmp::max(p0.y, p1.y) {
                continue;
            }
            let inter_x = (y-p0.y) * (p1.x - p0.x) / (p1.y - p0.y) + p0.x;
            if inter_x > x {
                count+=1;
            }

        }
        return count %2==1;
    }

    fn intersect(self: &Self, primary_polygon: &Polygon) -> Vec<Intersect> {
        let mut intersect_points = vec![];
        for (i, x) in self.points.iter().enumerate() {
            let p0 = self.points[i];
            let p1 = self.points[(i+1)% self.points.len()];
            for (j, y) in primary_polygon.points.iter().enumerate() {
                let p2 = primary_polygon.points[j];
                let p3 = primary_polygon.points[(j+1)% primary_polygon.points.len()];
                if let Some(v) = intersect_line(p0, p1, p2, p3)  {
                    let inter = Intersect::new(v, i, j);
                    intersect_points.push(inter);
                }
            }

        }
        return intersect_points;
    }

    fn generate_list(self: &Self, mut intersect_points: Vec<Intersect>, index:i32) -> Vec<Point> {
        let mut points_list = vec![];
        for (i, x) in self.points.iter().enumerate() {
            points_list.push(Point::Vertex(*x));
            let mut line = vec![];
            for y in intersect_points.iter_mut(){
                if (index==0 && i == y.l1 ) || (index==1 && i==y.l2) {
                    y.dis = distance(x.x, x.y, y.x, y.y);
                    line.push(*y);
                }
            }
            line.sort_by(|a,b| {
                return a.dis.cmp(&b.dis);
            });
            points_list.append(&mut line.iter().map(|v|{
                return Point::Intersect(*v)
            }).collect::<Vec<Point>>());
        }
        return points_list;
    }

    fn generate_degree(self: &Self, primary_list:&mut Vec<Point>) {
        let mut in_drgree = false;
        for x in primary_list.iter_mut() {
            match x {
                Point::Intersect(inter) => {
                    in_drgree = !in_drgree;
                    if in_drgree  {
                        inter.degree = DegreeType::In;
                    } else {
                        inter.degree = DegreeType::Out;
                    }
                }
                Point::Vertex(v) => {
                    if self.contains(v.x, v.y) {
                        in_drgree = true;
                    } else {
                        in_drgree = false;
                    }
                },
            }
        }
    }

    fn generate_clip_degree(self:&Self, clip_list:&mut Vec<Point>, primary_list:&mut Vec<Point>) {
        for x in clip_list.iter_mut() {
            match x {
                Point::Vertex(_) => {},
                Point::Intersect(inter) => {
                    for y in primary_list.iter() {
                        match y {
                            &Point::Vertex(_) => {}
                            &Point::Intersect(y) => {
                                if y.x == inter.x && y.y == inter.y {
                                    inter.degree = y.degree;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

