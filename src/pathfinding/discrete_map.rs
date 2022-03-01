use std::fmt;
use std::hash::{Hash, Hasher};

use bevy::prelude::*;

use super::Map;

type Row = usize;
type Col = usize;

pub const FIELD_SIZE: usize = 30;

#[derive(Debug, Clone)]
pub struct DiscreteMap {
    fields: Vec<Field>,
    map_size: usize,
    pathseeker: usize,
    target: usize,
}

impl PartialEq for DiscreteMap {
    fn eq(&self, other: &DiscreteMap) -> bool {
        self.pathseeker == other.pathseeker
    }
}

impl Eq for DiscreteMap {}

impl Hash for DiscreteMap {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pathseeker.hash(state);
    }
}

impl DiscreteMap {
    pub fn new(map: &Map, mid: Vec2, target: Vec2) -> Self {
        let map_size = (mid.distance(target) / FIELD_SIZE as f32) as usize + 4;
        let map_size = if map_size % 2 == 0 {
            map_size + 1
        } else {
            map_size
        };
        let capacity = map_size.pow(2);
        let distance_to_edge = (FIELD_SIZE * map_size / 2) as f32;
        let top_left_field_x = mid.x - distance_to_edge;
        let top_left_field_y = mid.y + distance_to_edge;

        let mut fields = (0..capacity)
            .map(|idx| {
                let (row, col) = Self::idx_to_coordinates(idx, map_size);
                let pos = Vec2::new(
                    top_left_field_x + (col * FIELD_SIZE) as f32,
                    top_left_field_y - (row * FIELD_SIZE) as f32,
                );

                Field {
                    idx,
                    pos,
                    kind: FieldKinds::Empty,
                }
            })
            .collect();

        let pathseeker = capacity / 2;
        let target = Self::pos_to_idx(&fields, target);

        fields[target].kind = FieldKinds::Target;
        fields[pathseeker].kind = FieldKinds::Pathseeker;

        let mut this = Self {
            fields,
            map_size,
            pathseeker,
            target,
        };

        this.mark_obstacles(map);

        this
    }

    pub fn arrived(&self) -> bool {
        self.fields[self.pathseeker].kind == FieldKinds::Target
    }

    pub fn successors(&self) -> Vec<Self> {
        self.neighbours(self.pathseeker)
            .into_iter()
            .filter(|neigbour| self.fields[*neigbour].is_walkable())
            .map(|neighbour| {
                let mut new = self.clone();

                new.fields[self.pathseeker].kind = FieldKinds::Empty;

                if new.fields[neighbour].kind != FieldKinds::Target {
                    new.fields[neighbour].kind = FieldKinds::Pathseeker;
                }

                new.pathseeker = neighbour;

                new
            })
            .collect()
    }

    pub fn mark(&mut self, idx: usize, kind: FieldKinds) {
        let (row, col) = Self::idx_to_coordinates(idx, self.map_size);
        let idx = Self::coordinates_to_idx(row, col, self.map_size);

        if let Some(idx) = idx {
            self.fields[idx].kind = kind;
        }
    }

    pub fn pathseeker(&self) -> usize {
        self.pathseeker
    }

    pub fn target(&self) -> usize {
        self.target
    }

    pub fn pathseeker_pos(&self) -> Vec2 {
        self.fields[self.pathseeker].pos
    }

    fn neighbours(&self, idx: usize) -> Vec<usize> {
        let (row, col) = Self::idx_to_coordinates(idx, self.map_size);
        let mut neighbours = vec![];

        if row > 0 {
            neighbours.extend(vec![
                (row + 1, col),
                (row + 1, col + 1),
                (row, col + 1),
                (row - 1, col),
                (row - 1, col + 1),
            ]);
        }

        if col > 0 {
            neighbours.extend(vec![(row + 1, col - 1), (row, col - 1)]);
        }

        if row > 0 && col > 0 {
            neighbours.push((row - 1, col - 1));
        }

        neighbours
            .into_iter()
            .flat_map(|(row, col)| {
                Self::coordinates_to_idx(
                    row as usize,
                    col as usize,
                    self.map_size,
                )
            })
            .collect()
    }

    fn mark_obstacles(&mut self, map: &Map) {
        let current_pos = self.fields[self.pathseeker].pos;

        for node in map.lymph_nodes.iter() {
            for mut field in self.fields.iter_mut() {
                if field.pos.distance(node.pos) < node.size {
                    field.kind = FieldKinds::Occupied
                }
            }
        }

        for cell in map.units.iter() {
            for mut field in self.fields.iter_mut() {
                if field.pos.distance(cell.pos) < cell.size {
                    field.kind = FieldKinds::Occupied
                }

                if field.pos.distance(current_pos) < cell.size * 1.2 {
                    // treat pathseeker pointlike
                    field.kind = FieldKinds::Empty
                }
            }
        }
    }

    fn pos_to_idx(fields: &Vec<Field>, pos: Vec2) -> usize {
        let mut idx = 0;
        let mut min_distance = f32::INFINITY;

        for (i, field) in fields.iter().enumerate() {
            let distance = field.pos.distance(pos);

            if distance < min_distance {
                idx = i;
                min_distance = distance
            }
        }

        idx
    }

    fn idx_to_coordinates(idx: usize, map_size: usize) -> (Row, Col) {
        let row = idx / map_size;
        let col = idx % map_size;

        (row, col)
    }

    fn coordinates_to_idx(
        row: Row,
        col: Col,
        map_size: usize,
    ) -> Option<usize> {
        if col >= map_size || row >= map_size {
            None
        } else {
            Some(col + (row * map_size))
        }
    }
}

impl fmt::Display for DiscreteMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut current_row = 0;
        for field in self.fields.iter() {
            let (row, _) = Self::idx_to_coordinates(field.idx, self.map_size);

            if current_row < row {
                write!(f, "\n")?;
                current_row = row;
            }

            write!(f, "[{}]", field.kind)?;
        }

        write!(f, "\n")
    }
}

#[derive(Debug, Clone)]
pub struct Field {
    kind: FieldKinds,
    pos: Vec2,
    idx: usize,
}

impl Field {
    pub fn is_walkable(&self) -> bool {
        match self.kind {
            FieldKinds::Empty | FieldKinds::Target | FieldKinds::Path => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum FieldKinds {
    Empty,
    Occupied,
    Pathseeker,
    Target,
    Path,
}

impl fmt::Display for FieldKinds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Empty => write!(f, " "),
            Self::Occupied => write!(f, "x"),
            Self::Pathseeker => write!(f, "o"),
            Self::Target => write!(f, "$"),
            Self::Path => write!(f, "-"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_coords(
        idx: usize,
        map_size: usize,
        expected_row: usize,
        expected_col: usize,
    ) {
        let (row, col) = DiscreteMap::idx_to_coordinates(idx, map_size);

        assert_eq!(row, expected_row);
        assert_eq!(col, expected_col);
    }

    #[test]
    fn test_cords() {
        let map_size = 10;
        assert_coords(9, map_size, 0, 9);
        assert_coords(10, map_size, 1, 0);
        assert_coords(11, map_size, 1, 1);
        let map_size = 11;
        assert_coords(10, map_size, 0, 10);
        assert_coords(11, map_size, 1, 0);
        assert_coords(12, map_size, 1, 1);
    }

    fn assert_index(
        row: usize,
        col: usize,
        map_size: usize,
        expected_idx: Option<usize>,
    ) {
        let idx = DiscreteMap::coordinates_to_idx(row, col, map_size);

        assert_eq!(idx, expected_idx);
    }

    #[test]
    fn test_idx() {
        let map_size = 11;
        assert_index(0, 0, map_size, Some(0));
        assert_index(0, 1, map_size, Some(1));
        assert_index(0, 9, map_size, Some(9));
        assert_index(1, 0, map_size, Some(11));
        assert_index(1, 1, map_size, Some(12));
        assert_index(1, 2, map_size, Some(13));
        assert_index(1, 3, map_size, Some(14));
        assert_index(1, 9, map_size, Some(20));
        assert_index(10, 10, map_size, Some(120));
        assert_index(11, 0, map_size, None);
        assert_index(11, 1, map_size, None);
        assert_index(10, 11, map_size, None);
        assert_index(0, 11, map_size, None);
    }
}
