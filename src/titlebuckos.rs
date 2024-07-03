use macroquad::{
    math::{vec2, Vec2},
    miniquad, rand,
};

#[derive(PartialEq)]
enum Quadrant {
    NW,
    NE,
    SW,
    SE,
}

#[derive(Debug, Default)]
pub struct TitleBuckos {
    positions:  Vec<Vec2>,
    directions: Vec<Vec2>,

    boundary: f32,

    time_acc: f32,
}

impl TitleBuckos {
    const BUCKO_RADIUS: f32 = 16.0;
    // const VELOCITY: f32 = 256.0;
    const TICK: f32 = 1.0 / 256.0;
    const LIMIT: u8 = 8;

    pub fn init(&mut self, amount: u8, boundary: f32) {
        rand::srand((miniquad::date::now() * 10e7) as u64);
        self.boundary = boundary;
        for _bucko in 0..amount {
            self.add();
        }
    }

    pub fn positions(&self) -> std::slice::Iter<Vec2> {
        self.positions.iter()
    }

    pub fn add(&mut self) {
        if self.positions.len() == Self::LIMIT as usize {
            return;
        }
        self.positions.push(vec2(
            rand::gen_range(Self::BUCKO_RADIUS, self.boundary - Self::BUCKO_RADIUS),
            rand::gen_range(Self::BUCKO_RADIUS, self.boundary - Self::BUCKO_RADIUS),
        ));
        self.directions.push(Vec2::from_angle(rand::gen_range(
            0.0,
            std::f32::consts::TAU,
        )))
    }

    pub fn remove(&mut self) {
        if self.positions.len() == 1 {
            return;
        }
        let index = rand::gen_range(0, self.positions.len());
        let _pos = self.positions.swap_remove(index);
        let _dir = self.directions.swap_remove(index);
    }

    pub fn update(&mut self, dt: f32) {
        self.time_acc += dt;
        while self.time_acc > Self::TICK {
            self.time_acc -= Self::TICK;
            self.update_fixed()
        }
    }

    fn quadrants(&self, index: usize) -> Vec<Quadrant> {
        let pos = &self.positions[index];
        let mid = &self.boundary / 2.0;
        let mut quadrants = Vec::<Quadrant>::new();

        if pos.x < mid + Self::BUCKO_RADIUS {
            if pos.y < mid + Self::BUCKO_RADIUS {
                quadrants.push(Quadrant::NW);
            }
            if pos.y > mid - Self::BUCKO_RADIUS {
                quadrants.push(Quadrant::SW);
            }
        }
        if pos.x > mid - Self::BUCKO_RADIUS {
            if pos.y < mid + Self::BUCKO_RADIUS {
                quadrants.push(Quadrant::NE);
            }
            if pos.y > mid - Self::BUCKO_RADIUS {
                quadrants.push(Quadrant::SE);
            }
        }

        quadrants
    }

    fn update_fixed(&mut self) {
        for index in 0..self.positions.len() {
            let pos = &mut self.positions[index];
            let dir = &mut self.directions[index];
            // let quad = &mut self.quadrants[i];

            if pos.x < Self::BUCKO_RADIUS || pos.x > self.boundary - Self::BUCKO_RADIUS {
                dir.x *= -1.0;
            }

            if pos.y < Self::BUCKO_RADIUS || pos.y > self.boundary - Self::BUCKO_RADIUS {
                dir.y *= -1.0;
            }

            *pos += *dir;
        }

        struct Collision(usize, usize);
        let mut collisions = Vec::<Collision>::new();

        for index in 0..self.positions.len() {
            let pos = &self.positions[index];
            self.positions
                .iter()
                .enumerate()
                .skip(index + 1)
                .filter(|(other_index, _)| {
                    self.quadrants(index).iter().any(|q| {
                        self.quadrants(*other_index)
                            .iter()
                            .any(|other_q| q == other_q)
                    })
                })
                .for_each(|(other_index, other_pos)| {
                    if (2.0 * Self::BUCKO_RADIUS).powi(2) > pos.distance_squared(*other_pos) {
                        collisions.push(Collision(index, other_index))
                    }
                });
        }

        for collision in collisions {
            let normal = (self.positions[collision.0] - self.positions[collision.1]).normalize();
            // let tangent = normal.perp();
            // self.positions[collision.0] += normal;
            // self.positions[collision.1] += -normal;

            self.directions[collision.0] = normal;
            self.directions[collision.1] = -normal;
        }
    }
}
