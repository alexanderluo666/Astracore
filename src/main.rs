#[derive(Debug, Clone, Copy)]
struct Vec2 {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Planet {
    name: String,
    variation: String,
    mass: f32,
    pos: Vec2,
    velocity: Vec2,
    radius: f32,
}

impl std::ops::Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, scalar: f32) -> Vec2 {
        Vec2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Vec2 {
    fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

impl Vec2 {
    fn normalize(&self) -> Vec2 {
        let len = self.length();
        if len == 0.0 {
            return Vec2 { x: 0.0, y: 0.0 };
        }

        Vec2 {
            x: self.x / len,
            y: self.y / len,
        }
    }
}

fn main() {
    let G = 10.0;

    let body_a: Planet = Planet {
        name: "RedStar".to_string(),
        variation: "RedGiant".to_string(),
        mass: 10.0,
        pos: Vec2 { x: 1.0, y: 2.0 },
        velocity: Vec2 { x: 1.0, y: 2.0},
        radius: 10.0
    };
    let body_b: Planet = Planet{
        name: "BlueStar".to_string(),
        variation: "BlueGiant".to_string(),
        mass: 20.0,
        pos: Vec2 {x: 10.0, y: 5.0},
        velocity: Vec2 { x: 5.0, y: 3.0},
        radius: 8.2
    };

    let direction = body_b.pos - body_a.pos;

    let distance = direction.length();

    let normalized = direction.normalize();

    let force_strength =
        G * body_a.mass * body_b.mass / (distance * distance);

    let gravity_force = normalized * force_strength;

    println!("{:#?}", body_a);
    println!("{:#?}", body_b);
}