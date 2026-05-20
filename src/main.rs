#[derive(Debug)]
struct Vec2 {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Planet {
    name: String,
    variation: String,
    size: i32,
    pos: Vec2,
}

fn main() {
    let planet1: Planet = Planet {
        name: "RedStar".to_string(),
        variation: "RedGiant".to_string(),
        size: 10,
        pos: Vec2 { x: 1, y: 2 },
    };

    println!("{:?}", planet1);

    println!("Start Of AstraCore");
}