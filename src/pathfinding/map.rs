use crate::level::LevelPoint;

#[derive(Clone, Debug, Default)]
pub struct Map {
    pub lymph_nodes: Vec<MapLymphNode>,
    pub walls: Vec<MapWall>,
    pub bounds: MapBounds,
}

#[derive(Clone, Debug)]
pub struct MapLymphNode {
    pub pos: LevelPoint,
}

#[derive(Clone, Debug)]
pub struct MapWall {
    pub pos: LevelPoint,
}

// TODO(pwy) use LevelPoint
#[derive(Clone, Copy, Debug, Default)]
pub struct MapBounds {
    pub min_x: i32,
    pub min_y: i32,
    pub max_x: i32,
    pub max_y: i32,
}

impl MapBounds {
    pub fn pos_to_idx(&self, pos: LevelPoint) -> usize {
        let x = pos.x - self.min_x;
        let y = pos.y - self.min_y;

        ((x * self.height()) + y) as usize
    }

    pub fn try_pos_to_idx(&self, pos: LevelPoint) -> Option<usize> {
        if self.contains(pos) {
            Some(self.pos_to_idx(pos))
        } else {
            None
        }
    }

    pub fn idx_to_pos(&self, idx: usize) -> LevelPoint {
        let idx = idx as i32;
        let height = self.height();

        LevelPoint::new(self.min_x + idx / height, self.min_y + idx % height)
    }

    pub fn width(&self) -> i32 {
        1 + self.max_x - self.min_x
    }

    pub fn height(&self) -> i32 {
        1 + self.max_y - self.min_y
    }

    pub fn contains(&self, p: LevelPoint) -> bool {
        let x_ok = (self.min_x..=self.max_x).contains(&p.x);
        let y_ok = (self.min_y..=self.max_y).contains(&p.y);

        x_ok && y_ok
    }
}
