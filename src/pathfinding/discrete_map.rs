use std::fmt;
use std::hash::Hash;
use std::ops::Deref;

use bevy::prelude::*;

use super::{Map, MapBounds, PathfindingPlugin};
use crate::level::LevelPoint;
use crate::systems::bio::Wall;
use crate::systems::physics::PHYSICS_SCALE;

type Cost = i32;

pub const FIELD_SIZE: usize = (Wall::SIZE * PHYSICS_SCALE) as _;

#[derive(Debug, Clone)]
pub struct DiscreteMap {
    bounds: MapBounds,
    fields: Vec<Field>,
    pathseeker: usize,
    target: usize,
}

impl DiscreteMap {
    pub fn new(map: &Map, pathseeker: Vec2, target: Vec2) -> Option<Self> {
        let pathseeker = map
            .bounds
            .try_pos_to_idx(PathfindingPlugin::world_to_level(pathseeker))?;

        let target = map
            .bounds
            .try_pos_to_idx(PathfindingPlugin::world_to_level(target))?;

        let mut fields = vec![
            Field::default();
            (map.bounds.width() * map.bounds.height())
                as usize
        ];

        if fields.is_empty() {
            return None;
        }

        for node in map.lymph_nodes.iter() {
            // TODO(pwy) should we consider node's sizes?
            fields[map.bounds.pos_to_idx(node.pos)].kind = FieldKind::Occupied;
        }

        for wall in map.walls.iter() {
            fields[map.bounds.pos_to_idx(wall.pos)].kind = FieldKind::Occupied;
        }

        fields[pathseeker].kind = FieldKind::Pathseeker;
        fields[target].kind = FieldKind::Target;

        Some(Self {
            bounds: map.bounds,
            fields,
            pathseeker,
            target,
        })
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
        let target = self.bounds.idx_to_pos(self.target);
        let node = self.bounds.idx_to_pos(idx);

        (target.x - node.x).abs() + (target.y - node.y).abs()
    }

    pub fn success(&self, idx: usize) -> bool {
        self.fields[idx].kind == FieldKind::Target
    }

    pub fn start(&self) -> PathNode {
        PathNode(self.pathseeker)
    }

    pub fn obstacles(&self) -> Vec<Vec2> {
        self.fields
            .iter()
            .enumerate()
            .filter(|(_, field)| field.kind == FieldKind::Occupied)
            .map(|(idx, _)| self.bounds.idx_to_pos(idx))
            .map(PathfindingPlugin::level_to_world)
            .collect()
    }

    fn neighbours(
        &self,
        idx: usize,
    ) -> impl Iterator<Item = (usize, Cost)> + '_ {
        let pos = self.bounds.idx_to_pos(idx);
        let (x, y) = (pos.x, pos.y);

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

        deltas.into_iter().flat_map(move |(x_d, y_d)| {
            let x = x + x_d;
            let y = y + y_d;
            let cost = (4 * x_d.abs() + 4 * y_d.abs()) / 2;

            self.bounds
                .try_pos_to_idx(LevelPoint::new(x, y))
                .map(|idx| (idx, cost))
        })
    }

    pub fn idx_to_pos(&self, idx: usize) -> LevelPoint {
        self.bounds.idx_to_pos(idx)
    }

    fn is_move_walkable(&self, from: usize, to: usize) -> bool {
        if !self.fields[to].is_walkable() {
            return false;
        }

        let from = self.bounds.idx_to_pos(from);
        let (x1, y1) = (from.x, from.y);

        let to = self.bounds.idx_to_pos(to);
        let (x2, y2) = (to.x, to.y);

        let is_diagonal = (x1 - x2).abs() + (y1 - y2).abs() > 1;

        if is_diagonal {
            let x_idx = self.bounds.pos_to_idx(LevelPoint::new(x2, y1));
            let x_blocked = !self.fields[x_idx].is_walkable();

            let y_idx = self.bounds.pos_to_idx(LevelPoint::new(x1, y2));
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

        for (field_idx, field) in self.fields.iter().enumerate() {
            let row = self.bounds.idx_to_pos(field_idx).x;

            if current_row < row {
                writeln!(f)?;
                current_row = row;
            }

            write!(f, "[{}]", field.kind)?;
        }

        writeln!(f)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PathNode(usize);

impl Deref for PathNode {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Field {
    kind: FieldKind,
}

impl Field {
    pub fn is_walkable(&self) -> bool {
        match self.kind {
            FieldKind::Empty | FieldKind::Target | FieldKind::Path => true,
            _ => false,
        }
    }
}

impl Default for Field {
    fn default() -> Self {
        Self {
            kind: FieldKind::Empty,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum FieldKind {
    Empty,
    Occupied,
    Pathseeker,
    Target,
    Path,
}

impl fmt::Display for FieldKind {
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
