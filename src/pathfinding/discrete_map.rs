use std::fmt;

use bevy::prelude::*;

use super::Map;

type Row = usize;
type Col = usize;

pub struct DiscreteMap {
    fields: Vec<Field>,
    map_size: usize,
    field_size: usize,
    pathseeker: usize,
    target: usize,
}

impl DiscreteMap {
    pub fn new(map: &Map, mid: Vec2, target: Vec2) -> Self {
        let field_size = 30; // should be in config
        let map_size = (mid.distance(target) / field_size as f32) as usize + 4;
        let map_size = if map_size % 2 == 0 {
            map_size + 1
        } else {
            map_size
        };
        let capacity = map_size.pow(2);
        let distance_to_edge = (field_size * map_size / 2) as f32;
        let top_left_field_x = mid.x - distance_to_edge;
        let top_left_field_y = mid.y + distance_to_edge;

        let mut fields = (0..capacity)
            .map(|idx| {
                let (row, col) = Self::idx_to_coordinates(idx, map_size);
                let pos = Vec2::new(
                    top_left_field_x + (col * field_size) as f32,
                    top_left_field_y - (row * field_size) as f32,
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
            field_size,
            pathseeker,
            target,
        };

        this.mark_obstacles(map);

        this
    }

    // pub fn successors(&self) -> Vec<Self> {

    // }

    // fn neighbours(&self, idx: usize) -> Vec<usize> {

    // }

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

                if field.pos.distance(current_pos) < cell.size {
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

    fn coordinates_to_idx(row: Row, col: Col, map_size: usize) -> Option<usize> {
        let idx = col * (row + 1);

        if idx <= map_size.pow(2) {
            Some(idx)
        } else {
            None
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

pub struct Field {
    kind: FieldKinds,
    pos: Vec2,
    idx: usize,
}

pub enum FieldKinds {
    Empty,
    Occupied,
    Target,
    Pathseeker
}

impl fmt::Display for FieldKinds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Empty => write!(f, " "),
            Self::Occupied => write!(f, "x"),
            Self::Target => write!(f, "$"),
            Self::Pathseeker => write!(f, "o"),
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
        assert_index(1, 0, map_size, Some(11));
    }
}
