mod gen;

use crate::systems::units::Alignment;

#[derive(Clone, Debug)]
pub struct Level {
    pub chambers: Vec<LevelChamber>,
    pub corridors: Vec<LevelCorridor>,
    pub wave: LevelWave,
    pub wave_idx: usize,
}

impl Level {
    pub fn start() -> Self {
        gen::start()
    }

    pub fn progress(&mut self) {
        gen::progress(self)
    }
}

#[derive(Clone, Debug)]
pub struct LevelChamber {
    pub x: i32,
    pub y: i32,
    pub r: i32,
}

impl LevelChamber {
    fn contains(&self, x: i32, y: i32) -> bool {
        (self.x - x).pow(2) + (self.y - y).pow(2) < self.r.pow(2)
    }

    fn distance_to_squared(&self, other: &Self) -> i32 {
        (other.x - self.x).pow(2) + (other.y - self.y).pow(2)
    }

    fn collides_with(&self, other: &Self) -> bool {
        (other.x - self.x).pow(2) + (other.y - self.y).pow(2)
            <= (self.r + other.r).pow(2)
    }
}

#[derive(Clone, Debug)]
pub struct LevelCorridor {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

impl LevelCorridor {
    fn walls(
        &self,
        chambers: &[LevelChamber],
    ) -> (Vec<(i32, i32)>, Vec<(i32, i32)>) {
        let mut add_walls = Vec::new();
        let mut remove_walls = Vec::new();

        let x_len = (self.x1 - self.x2).abs();
        let y_len = (self.y1 - self.y2).abs();

        let joint = (self.x1, self.y2);

        for dx in 0..x_len {
            let x = self.x1.min(self.x2) + dx;
            let y = joint.1;

            add_walls.push((x, y - 2));
            remove_walls.push((x, y - 1));
            remove_walls.push((x, y));
            remove_walls.push((x, y + 1));
            add_walls.push((x, y + 2));
        }

        for dy in 0..y_len {
            let x = joint.0;
            let y = self.y1.min(self.y2) + dy;

            add_walls.push((x - 2, y));
            remove_walls.push((x - 1, y));
            remove_walls.push((x, y));
            remove_walls.push((x + 1, y));
            add_walls.push((x + 2, y));
        }

        let add_walls = add_walls
            .into_iter()
            .filter(|(x, y)| !chambers.iter().any(|c| c.contains(*x, *y)))
            .collect();

        (add_walls, remove_walls)
    }
}

#[derive(Clone, Debug, Default)]
pub struct LevelWave {
    pub ops: Vec<LevelWaveOp>,
}

#[derive(Clone, Debug)]
pub enum LevelWaveOp {
    AddWall {
        x: i32,
        y: i32,
    },

    RemoveWall {
        x: i32,
        y: i32,
    },

    AddLymphNode {
        x: i32,
        y: i32,
        alignment: Alignment,
    },
}
