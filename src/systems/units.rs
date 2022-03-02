use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::cell_node::CellBody;
use super::physics::pixel_to_world;

const MAX_SPEED: f32 = 5.0;
const FORCE_FACTOR: f32 = 1.0;
const STOPPING_FORCE_FACTOR: f32 = 2.0;

const MOVEMENT_STRETCH_FACTOR: f32 = 1.4;
const MOVEMENT_SQUEEZE_FACTOR: f32 = 0.6;

const MAX_HEALTH: f32 = 1.0;
const BASE_DAMAGE: f32 = 0.25; // By default a cell can take 4 hits

#[derive(Component)]
pub struct Unit {
    // TODO(dzejkop): Should be enum, target can be unit, etc.
    pub target: Option<Vec2>,
    pub path: Vec<Vec2>,
    pub step: usize,
    pub health: f32,
}

impl Default for Unit {
    fn default() -> Self {
        Self {
            target: Default::default(),
            path: Default::default(),
            step: Default::default(),
            health: MAX_HEALTH,
        }
    }
}

impl Unit {
    pub fn set_path(&mut self, path: Vec<Vec2>) {
        self.step = 0;
        self.path = path;
    }
}

pub fn initialize(app: &mut App) {
    app.add_system(movement)
        .add_system(animate)
        .add_system(display_events);
}

pub fn movement(
    mut units: Query<(
        &RigidBodyVelocityComponent,
        &mut RigidBodyForcesComponent,
        &mut Unit,
        &Transform,
    )>,
) {
    for (velocity, mut forces, mut unit, transform) in units.iter_mut() {
        if let Some(target) = unit.target {
            let current_pos = transform.translation.truncate();
            let default_force_direction = target - current_pos;

            let force_direction = if let Some(_) =
                unit.path.get(unit.step).copied()
            {
                let mut min_distance_from_node = f32::INFINITY;
                let mut new_step = unit.step;

                // Follow through path
                for (idx, pos) in unit.path.iter().enumerate().skip(unit.step) {
                    let actual_distance_from_node = pos.distance(current_pos);

                    if min_distance_from_node > actual_distance_from_node {
                        new_step = idx;
                        min_distance_from_node = actual_distance_from_node
                    }
                }

                unit.step = new_step;

                if unit.step < unit.path.len() - 1 {
                    let next_pos = unit.path[unit.step + 1];
                    let path_direction = next_pos - current_pos;

                    // Compensate direction by path
                    rotate(default_force_direction, path_direction)
                } else {
                    default_force_direction
                }
            } else {
                // Wait till you have path
                Vec2::ZERO
            };

            let force_direction = pixel_to_world(force_direction);
            let desired_linvel: Vector<Real> =
                if force_direction.magnitude() < 1.0 {
                    force_direction * MAX_SPEED
                } else {
                    force_direction.normalize() * MAX_SPEED
                };

            let current_linvel = velocity.linvel;

            let diff = desired_linvel - current_linvel;

            forces.force = diff * FORCE_FACTOR;
        } else {
            // Try to stop moving
            let desired_linvel: Vector<Real> = [0.0, 0.0].into();
            let current_linvel = velocity.linvel;

            let diff = desired_linvel - current_linvel;
            forces.force = diff * STOPPING_FORCE_FACTOR;
        }
    }
}

fn display_events(
    mut commands: Commands,
    mut contact_events: EventReader<ContactEvent>,
    mut units: Query<&mut Unit>,
) {
    for contact_event in contact_events.iter() {
        match contact_event {
            ContactEvent::Started(left, right) => {
                deal_damage(left, &mut units, &mut commands);
                deal_damage(right, &mut units, &mut commands);
            }
            ContactEvent::Stopped(_, _) => (),
        }
    }
}

fn deal_damage(
    handle: &ColliderHandle,
    units: &mut Query<&mut Unit>,
    commands: &mut Commands,
) {
    let entity = handle.entity();
    if let Ok(mut unit) = units.get_mut(entity) {
        unit.health -= BASE_DAMAGE;

        if unit.health <= 0.0 {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn rotate(v: Vec2, to: Vec2) -> Vec2 {
    let angle = to.angle_between(v);
    let x = v.x * angle.cos() - v.y * angle.sin();
    let y = v.x * angle.sin() + v.y * angle.cos();

    Vec2::new(x, y)
}

fn animate(
    units: Query<(&Children, &RigidBodyVelocityComponent)>,
    mut child_sprites: Query<(&CellBody, &mut Transform)>,
) {
    for (children, velocity) in units.iter() {
        if velocity.linvel.magnitude() < 0.1 {
            continue;
        }

        let direction = velocity.linvel.normalize();

        let up: Vector<Real> = [0.0, 1.0].into();
        let angle = direction.angle(&up);

        let angle = if direction.x < 0.0 {
            angle
        } else {
            std::f32::consts::PI - angle
        };

        for child in children.iter() {
            if let Ok((_, mut transform)) = child_sprites.get_mut(*child) {
                let stretch_x = remap(
                    velocity.linvel.magnitude(),
                    0.0,
                    MAX_SPEED,
                    1.0,
                    MOVEMENT_SQUEEZE_FACTOR,
                );
                let stretch_y = remap(
                    velocity.linvel.magnitude(),
                    0.0,
                    MAX_SPEED,
                    1.0,
                    MOVEMENT_STRETCH_FACTOR,
                );

                transform.rotation = Quat::from_rotation_z(angle);
                transform.scale = Vec3::new(stretch_x, stretch_y, 1.0);
            }
        }
    }
}

fn remap(x: f32, in_min: f32, in_max: f32, out_min: f32, out_max: f32) -> f32 {
    (x - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}
