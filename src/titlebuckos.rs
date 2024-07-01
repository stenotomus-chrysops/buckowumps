use macroquad::{
    math::{vec2, Vec2},
    miniquad, rand,
};

#[derive(Default)]
pub struct TitleBuckos {
    positions:  Vec<Vec2>,
    directions: Vec<Vec2>,

    boundary: f32,
}

impl TitleBuckos {
    const BUCKO_RADIUS: f32 = 32.0;
    const VELOCITY: f32 = 16.0;
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
        self.positions
            .iter_mut()
            .zip(self.directions.iter_mut())
            .for_each(|(pos, dir)| {
                if pos.x < Self::BUCKO_RADIUS || pos.x > self.boundary - Self::BUCKO_RADIUS {
                    *dir *= vec2(-1.0, 1.0);
                }
                if pos.y < Self::BUCKO_RADIUS || pos.y > self.boundary - Self::BUCKO_RADIUS {
                    *dir *= vec2(1.0, -1.0);
                }
                *pos += *dir * Self::VELOCITY * dt;
            })
    }
}
