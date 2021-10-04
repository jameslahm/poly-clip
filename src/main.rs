use poly_clip::clip::*;

fn main() {
    let p1 = Vertex::new(1, 1);
    let p2 = Vertex::new(3, 1);
    let p3 = Vertex::new(3, 3);
    let p4 = Vertex::new(1, 3);

    let points = vec![p1, p2, p3, p4];

    let clip_polygon = Polygon::new(points);

    let p5 = Vertex::new(0, 0);
    let p6 = Vertex::new(2, 0);
    let p7 = Vertex::new(2, 2);
    let p8 = Vertex::new(0, 2);

    let points = vec![p5, p6, p7, p8];
    let primary_polygon = Polygon::new(points);

    let res = clip_polygon.clip(&primary_polygon);
    println!("{:?}", &res);
}
