use poly_clip::clip;


fn main(){
    let poly = clip::Polygon {
        rings: vec![vec![
            clip::Point {x: 310, y: 186},
            clip::Point {x: 598, y: 186},
            clip::Point {x: 600, y: 408},
            clip::Point {x: 305, y: 402},
        ], vec![
            clip::Point {x: 375, y: 326},
            clip::Point {x: 518, y: 328},
            clip::Point {x: 521, y: 252},
            clip::Point {x: 378, y: 252}
        ]],
    };
    let inter_polygon = clip::Polygon {
        rings: vec![vec![
            clip::Point {x: 361, y: 219},
            clip::Point {x: 665, y: 219},
            clip::Point {x: 668, y: 489},
            clip::Point {x: 341, y: 464},
        ]],
    };
    let res = poly.clip(&inter_polygon);
    println!("{:?}",&res);
}