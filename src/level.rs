mod gen;

use nalgebra::Point2;

use crate::systems::units::Alignment;

pub type LevelPoint = Point2<i32>;

// TODO(pwy) since there's just one level anyway, we could use a better name (World?)
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

    // TODO(pwy) add memoization
    pub fn bounds(&self) -> (i32, i32, i32, i32) {
        let mut min_x = 0;
        let mut min_y = 0;
        let mut max_x = 0;
        let mut max_y = 0;

        for chamber in &self.chambers {
            min_x = min_x.min(chamber.x - chamber.r);
            min_y = min_y.min(chamber.y - chamber.r);
            max_x = max_x.max(chamber.x + chamber.r);
            max_y = max_y.max(chamber.y + chamber.r);
        }

        (min_x, min_y, max_x, max_y)
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

        for dx in 0..=x_len {
            let x = self.x1.min(self.x2) + dx;
            let y = joint.1;

            add_walls.push((x, y - 2));
            remove_walls.push((x, y - 1));
            remove_walls.push((x, y));
            remove_walls.push((x, y + 1));
            add_walls.push((x, y + 2));
        }

        for dy in 0..=y_len {
            let x = joint.0;
            let y = self.y1.min(self.y2) + dy;

            add_walls.push((x - 2, y));
            remove_walls.push((x - 1, y));
            remove_walls.push((x, y));
            remove_walls.push((x + 1, y));
            add_walls.push((x + 2, y));
        }

        for dx in -2..=2 {
            for dy in -2..=2 {
                let x = joint.0 + dx;
                let y = joint.1 + dy;

                let has_add =
                    add_walls.iter().any(|(ax, ay)| (*ax == x) && (*ay == y));

                if !has_add {
                    add_walls.push((x, y));
                }
            }
        }

        for dx in -1..=1 {
            for dy in -1..=1 {
                let x = joint.0 + dx;
                let y = joint.1 + dy;

                let has_remove = remove_walls
                    .iter()
                    .any(|(ax, ay)| (*ax == x) && (*ay == y));

                if !has_remove {
                    remove_walls.push((x, y));
                }
            }
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
