use poly_clip::clip;


fn main(){
    let poly = clip::Polygon {
        rings: vec![vec![
            clip::Point {x: 292, y: 137},
            clip::Point {x: 439, y: 137},
            clip::Point {x: 445, y: 345},
            clip::Point {x: 285, y: 336},
        ], vec![
            clip::Point {x: 320, y: 198},
            clip::Point {x: 320, y: 268},
            clip::Point {x: 383, y: 260},
            clip::Point {x: 385, y: 194}
        ]],
    };
    let inter_polygon = clip::Polygon {
        rings: vec![vec![
            clip::Point {x: 353, y: 227},
            clip::Point {x: 535, y: 214},
            clip::Point {x: 533, y: 437},
            clip::Point {x: 366, y: 426},
        ]],
    };
    let res = poly.clip(&inter_polygon);
    println!("{:?}",&res);
}