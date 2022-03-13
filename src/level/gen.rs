//! lord forgive me for i have sinned, but at some point i _will_ refactor it

use std::f32::consts::TAU;

use itertools::Itertools;
use rand::prelude::SliceRandom;
use rand::Rng;

use super::{Level, LevelChamber, LevelCorridor, LevelWave, LevelWaveOp};
use crate::systems::units::Alignment;

pub fn start() -> Level {
    let mut ops = Vec::new();

    add_circle_wall(&mut ops, 0, 0, 15);
    add_lymph_node(&mut ops, 0, 0, Alignment::Player);
    add_lymph_node(&mut ops, -4, -4, Alignment::Player);
    add_lymph_node(&mut ops, 4, -4, Alignment::Player);
    add_lymph_node(&mut ops, -4, 4, Alignment::Player);
    add_lymph_node(&mut ops, 4, 4, Alignment::Player);

    Level {
        chambers: vec![LevelChamber { x: 0, y: 0, r: 15 }],
        corridors: vec![],
        wave: LevelWave { ops },
        wave_idx: 0,
    }
}

pub fn progress(level: &mut Level) {
    level.wave = Default::default();
    level.wave_idx += 1;

    let mut rng = rand::thread_rng();

    let chamber_count: i32 = if level.wave_idx < 3 {
        1
    } else {
        rng.gen_range(1..=3)
    };

    for _ in 0..chamber_count {
        let chamber = spawn_chamber(level);

        add_circle_wall(&mut level.wave.ops, chamber.x, chamber.y, chamber.r);

        if level.wave_idx < 3 {
            for _ in 0..2 {
                spawn_chamber_lymph_node(&mut level.wave.ops, &chamber, true);
            }
        } else {
            for n in 0..10 {
                spawn_chamber_lymph_node(&mut level.wave.ops, &chamber, n <= 3);
            }
        }

        let chambers = level
            .chambers
            .iter()
            .sorted_by(|a, b| {
                let a = a.distance_to_squared(&chamber);
                let b = b.distance_to_squared(&chamber);

                a.cmp(&b)
            })
            .take(rng.gen_range(1..=3))
            .cloned()
            .collect_vec();

        level.chambers.push(chamber.clone());

        for linked in chambers {
            spawn_corridor(level, &chamber, &linked);
        }
    }

    for corridor in &level.corridors {
        let (_, remove) = corridor.walls(&level.chambers);
        remove_walls(&mut level.wave.ops, remove);
    }
}

fn spawn_chamber(level: &Level) -> LevelChamber {
    let mut rng = rand::thread_rng();
    let (mut min_x, mut min_y, mut max_x, mut max_y) = aabb(level);

    for _ in 0..100 {
        let r = rng.gen_range(8..20);
        let xs = (min_x + r)..(max_x - r);
        let ys = (min_y + r)..(max_y - r);

        if xs.is_empty() || ys.is_empty() {
            continue;
        }

        let chamber = LevelChamber {
            x: rng.gen_range(xs),
            y: rng.gen_range(ys),
            r,
        };

        let collides = level
            .chambers
            .iter()
            .any(|chamber2| chamber2.collides_with(&chamber));

        if !collides {
            return chamber;
        }
    }

    let chamber = LevelChamber {
        x: rng.gen_range(8..20),
        y: rng.gen_range(10..30),
        r: rng.gen_range(10..30),
    };

    match rng.gen_range(0..4) {
        0 => {
            min_x -= chamber.x + chamber.r;

            LevelChamber {
                x: min_x,
                ..chamber
            }
        }

        1 => {
            min_y -= chamber.y + chamber.r;

            LevelChamber {
                y: min_y,
                ..chamber
            }
        }

        2 => {
            max_x += chamber.x + chamber.r;

            LevelChamber {
                x: max_x,
                ..chamber
            }
        }

        _ => {
            max_y += chamber.y + chamber.r;

            LevelChamber {
                y: max_y,
                ..chamber
            }
        }
    }
}

fn spawn_chamber_lymph_node(
    ops: &mut Vec<LevelWaveOp>,
    c: &LevelChamber,
    force: bool,
) {
    let mut rng = rand::thread_rng();

    while force {
        let angle = rng.gen_range(0f32..=TAU);
        let direction = rng.gen_range(0..(c.r - 1)) as f32;

        let x = (angle.sin() * direction) as i32;
        let y = (angle.cos() * direction) as i32;

        if c.contains(c.x + x, c.y + y) {
            let collides = ops.iter().any(|op| {
                if let LevelWaveOp::AddLymphNode { x: x2, y: y2, .. } = op {
                    *x2 == x && *y2 == y
                } else {
                    false
                }
            });

            if !collides {
                add_lymph_node(ops, c.x + x, c.y + y, Alignment::Enemy);
                return;
            }
        }
    }
}

fn aabb(level: &Level) -> (i32, i32, i32, i32) {
    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;

    for chamber in &level.chambers {
        min_x = min_x.min(chamber.x - chamber.r);
        min_y = min_y.min(chamber.y - chamber.r);
        max_x = max_x.max(chamber.x + chamber.r);
        max_y = max_y.max(chamber.y + chamber.r);
    }

    (min_x, min_y, max_x, max_y)
}

fn add_circle_wall(ops: &mut Vec<LevelWaveOp>, x: i32, y: i32, r: i32) {
    let mut coords = Vec::new();
    let mut remove_coords = Vec::new();

    for dx in -r..=r {
        for dy in -r..=r {
            let dist = dx.pow(2) + dy.pow(2);

            if dist <= (r - 1).pow(2) {
                remove_coords.push((x + dx, y + dy));
            } else if dist >= (r - 1).pow(2) && dist <= r.pow(2) {
                coords.push((x + dx, y + dy));
            }
        }
    }

    add_walls(ops, coords);

    for (x, y) in remove_coords {
        ops.push(LevelWaveOp::RemoveWall { x, y });
    }
}

fn spawn_corridor(level: &mut Level, c1: &LevelChamber, c2: &LevelChamber) {
    let corridor = LevelCorridor {
        x1: c1.x,
        y1: c1.y,
        x2: c2.x,
        y2: c2.y,
    };

    let (add, remove) = corridor.walls(&level.chambers);

    add_walls(&mut level.wave.ops, add);
    remove_walls(&mut level.wave.ops, remove);

    level.corridors.push(corridor);
}

fn add_lymph_node(
    ops: &mut Vec<LevelWaveOp>,
    x: i32,
    y: i32,
    alignment: Alignment,
) {
    ops.push(LevelWaveOp::AddLymphNode { x, y, alignment });
}

fn add_walls(
    ops: &mut Vec<LevelWaveOp>,
    walls: impl IntoIterator<Item = (i32, i32)>,
) {
    let mut walls = walls.into_iter().collect_vec();

    walls.shuffle(&mut rand::thread_rng());

    ops.extend(
        walls
            .into_iter()
            .map(|(x, y)| LevelWaveOp::AddWall { x, y }),
    );
}

fn remove_walls(
    ops: &mut Vec<LevelWaveOp>,
    walls: impl IntoIterator<Item = (i32, i32)>,
) {
    ops.extend(
        walls
            .into_iter()
            .map(|(x, y)| LevelWaveOp::RemoveWall { x, y }),
    );
}
