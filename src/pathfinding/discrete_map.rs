use std::fmt;
use std::hash::Hash;
use std::ops::Deref;

use bevy::prelude::*;

use super::Map;
use crate::systems::bio::{Cell, LymphNode, Wall};
use crate::systems::physics::PHYSICS_SCALE;

type Row = usize;
type Col = usize;
type Cost = i32;

pub const FIELD_SIZE: usize = (Wall::SIZE * PHYSICS_SCALE) as _;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PathNode(usize);

impl Deref for PathNode {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct DiscreteMap {
    fields: Vec<Field>,
    map_size: usize,
    pathseeker: usize,
    target: usize,
}

impl DiscreteMap {
    pub fn new(
        map: &Map,
        mid: Vec2,
        target: Vec2,
        map_size: Option<usize>,
    ) -> Self {
        let map_size =
            map_size.unwrap_or_else(|| Self::map_size_from_pos(mid, target));
        let map_size = Self::standarize_map_size(map_size);
        let capacity = map_size.pow(2);
        let distance_to_edge = (FIELD_SIZE * map_size) as f32;
        let top_left_field_x = mid.x - distance_to_edge;
        let top_left_field_y = mid.y + distance_to_edge;

        let fields = (0..capacity)
            .map(|idx| {
                let (row, col) = Self::idx_to_coordinates(idx, map_size);
                let pos = Vec2::new(
                    top_left_field_x + (col * FIELD_SIZE * 2) as f32,
                    top_left_field_y - (row * FIELD_SIZE * 2) as f32,
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

        let mut this = Self {
            fields,
            map_size,
            pathseeker,
            target,
        };

        this.mark_obstacles(map);
        this.fields[target].kind = FieldKinds::Target;
        this.fields[pathseeker].kind = FieldKinds::Pathseeker;

        this
    }

    pub fn successors(
        &self,
        idx: usize,
    ) -> impl Iterator<Item = (PathNode, Cost)> + '_ {
        self.neighbours(idx)
            .filter(move |(idx2, _)| self.is_move_walkable(idx, *idx2))
            .map(|(idx, cost)| (PathNode(idx), cost))
    }

    pub fn heuristic(&self, idx: usize) -> Cost {
        let (target_row, target_col) =
            Self::idx_to_coordinates(idx, self.map_size);
        let (node_row, node_col) = Self::idx_to_coordinates(idx, self.map_size);

        let (target_row, target_col) = (target_row as i32, target_col as i32);
        let (node_row, node_col) = (node_row as i32, node_col as i32);

        (target_row - node_row).abs() + (target_col - node_col).abs()
    }

    pub fn success(&self, idx: usize) -> bool {
        self.fields[idx].kind == FieldKinds::Target
    }

    pub fn start(&self) -> PathNode {
        PathNode(self.pathseeker)
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

    pub fn target_pos(&self) -> Vec2 {
        self.fields[self.target].pos
    }

    pub fn obstacles(&self) -> Vec<Vec2> {
        self.fields
            .iter()
            .filter(|field| field.kind == FieldKinds::Occupied)
            .map(|field| field.pos)
            .collect()
    }

    pub fn map_size_from_distance(distance: f32) -> usize {
        distance as usize / FIELD_SIZE
    }

    pub fn map_size_from_pos(one: Vec2, other: Vec2) -> usize {
        Self::map_size_from_distance(one.distance(other))
    }

    pub fn standarize_map_size(map_size: usize) -> usize {
        if map_size % 2 == 0 {
            map_size + 1
        } else {
            map_size
        }
    }

    fn neighbours(&self, idx: usize) -> impl Iterator<Item = (usize, Cost)> {
        let (row, col) = Self::idx_to_coordinates(idx, self.map_size);
        let (row, col) = (row as i32, col as i32);
        let map_size = self.map_size as i32;

        let deltas = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            //
            (0, -1),
            (0, 0),
            (0, 1),
            //
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        deltas.into_iter().flat_map(move |(row_delta, col_delta)| {
            let row = row + row_delta;
            let col = col + col_delta;
            let cost = (4 * row_delta.abs() + 4 * col_delta.abs()) / 2;

            let idx =
                if row < 0 || col < 0 || row >= map_size || col >= map_size {
                    None
                } else {
                    Self::coordinates_to_idx(
                        row as usize,
                        col as usize,
                        map_size as usize,
                    )
                };

            idx.map(|idx| (idx, cost))
        })
    }

    fn mark_obstacles(&mut self, map: &Map) {
        let current_pos = self.fields[self.pathseeker].pos;
        let target = self.fields[self.target].pos;

        for node in map.lymph_nodes.iter() {
            for mut field in self.fields.iter_mut() {
                if field.pos.distance_squared(node.pos)
                    < (LymphNode::SIZE * PHYSICS_SCALE).powi(2)
                {
                    field.kind = FieldKinds::Occupied;
                }

                if field.pos.distance_squared(current_pos)
                    < (Cell::SIZE * PHYSICS_SCALE * 2.0).powi(2)
                {
                    // treat pathseeker pointlike
                    field.kind = FieldKinds::Empty;
                }

                if field.pos.distance_squared(target)
                    < (Cell::SIZE * PHYSICS_SCALE * 2.0).powi(2)
                {
                    // clear target
                    field.kind = FieldKinds::Empty;
                }
            }
        }

        // for cell in map.units.iter() {
        //     for mut field in self.fields.iter_mut() {
        //         if field.pos.distance_squared(cell.pos)
        //             < (Cell::SIZE * PHYSICS_SCALE).powi(2)
        //         {
        //             field.kind = FieldKinds::Occupied;
        //         }

        //         if field.pos.distance_squared(current_pos)
        //             < (Cell::SIZE * PHYSICS_SCALE * 2.0).powi(2)
        //         {
        //             // treat pathseeker pointlike
        //             field.kind = FieldKinds::Empty;
        //         }

        //         if field.pos.distance_squared(target)
        //             < (Cell::SIZE * PHYSICS_SCALE * 2.0).powi(2)
        //         {
        //             // clear target
        //             field.kind = FieldKinds::Empty;
        //         }
        //     }
        // }

        for wall in map.walls.iter() {
            for mut field in self.fields.iter_mut() {
                if field.pos.distance_squared(wall.pos)
                    < (Wall::SIZE * PHYSICS_SCALE).powi(2)
                {
                    field.kind = FieldKinds::Occupied;
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

    pub fn idx_to_pos(&self, idx: usize) -> Vec2 {
        self.fields[idx].pos
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

    fn is_move_walkable(&self, from: usize, to: usize) -> bool {
        if !self.fields[to].is_walkable() {
            return false;
        }

        let (x1, y1) = Self::idx_to_coordinates(from, self.map_size);
        let (x2, y2) = Self::idx_to_coordinates(to, self.map_size);

        let (x1, y1) = (x1 as i32, y1 as i32);
        let (x2, y2) = (x2 as i32, y2 as i32);

        let is_diagonal = (x1 - x2).abs() + (y1 - y2).abs() > 1;

        if is_diagonal {
            let x_idx =
                Self::coordinates_to_idx(x2 as _, y1 as _, self.map_size)
                    .unwrap();
            let x_blocked = !self.fields[x_idx].is_walkable();

            let y_idx =
                Self::coordinates_to_idx(x1 as _, y2 as _, self.map_size)
                    .unwrap();
            let y_blocked = !self.fields[y_idx].is_walkable();

            if x_blocked && y_blocked {
                return false;
            }
        }

        true
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
#[repr(u8)]
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
