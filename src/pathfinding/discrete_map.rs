use std::fmt;

use bevy::prelude::*;

use crate::map::Map;

type Row = usize;
type Col = usize;

pub struct DiscreteMap {
    fields: Vec<Field>,
    map_size: usize,
    field_size: usize,
    current: usize,
}

impl DiscreteMap {
    pub fn new(map: &Map, mid: Vec2, map_size: usize) -> Self {
        let map_size = if map_size % 2 == 0 {
            map_size + 1
        } else {
            map_size
        };
        let capacity = map_size.pow(2);
        let field_size = 30; // should be in config
        let distance_to_edge = (field_size * map_size / 2) as f32;
        let top_left_field_x = mid.x - distance_to_edge;
        let top_left_field_y = mid.y + distance_to_edge;

        let fields = (0..capacity)
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

        let current = capacity / 2;

        let mut this = Self {
            fields,
            map_size,
            field_size,
            current,
        };

        this.mark_obstacles(map);

        this
    }

    fn mark_obstacles(&mut self, map: &Map) {
        let cell_node_size = 60.0; // should be in CellNode
        let lymph_node_size = 100.0; // should be in LymphNode
        let current_pos = self.fields[self.current].pos;

        for cell in map.cell_nodes.iter() {
            for mut field in self.fields.iter_mut() {
                if field.pos.distance(cell.pos) < cell_node_size / 2f32 {
                    field.kind = FieldKinds::Occupied
                }

                if field.pos.distance(current_pos) < cell_node_size {
                    // treat pathseeker pointlike
                    field.kind = FieldKinds::Empty
                }
            }
        }

        for lymph in map.lymph_nodes.iter() {
            for mut field in self.fields.iter_mut() {
                if field.pos.distance(lymph.pos) < lymph_node_size {
                    field.kind = FieldKinds::Occupied
                }
            }
        }
    }

    fn pos_to_idx(&self, pos: Vec2) -> usize {
        let mut idx = 0;
        let mut min_distance = f32::INFINITY;

        for (i, field) in self.fields.iter().enumerate() {
            let distance = field.pos.distance(pos);

            if distance < min_distance {
                idx = i;
                min_distance = distance
            }
        }

        idx
    }

    fn idx_to_coordinates(idx: usize, map_size: usize) -> (Row, Col) {
        let row = if idx == 0 { 0 } else { idx / map_size };
        let col = idx % map_size;

        (row, col)
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

            if field.idx == self.current {
                write!(f, "[o]")?;
            } else {
                write!(f, "[{}]", field.kind)?;
            }
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
}

impl fmt::Display for FieldKinds {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Empty => write!(f, " "),
            Self::Occupied => write!(f, "x"),
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
}
