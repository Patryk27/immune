use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_prototype_debug_lines::DebugLinesPlugin;
use unfair_advantage::systems::{
    camera, cell_node, debug, input, physics, units,
};
use unfair_advantage::{compiling, game, pathfinding, ui};

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::rgb_u8(1, 8, 90)))
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugLinesPlugin::default())
        .add_plugin(EguiPlugin)
        .add_plugin(input::InputPlugin)
        .add_plugin(ui::UiPlugin)
        .add_plugin(compiling::CompilingPlugin)
        .add_plugin(pathfinding::PathfindingPlugin)
        .add_plugin(game::GamePlugin);

    debug::initialize(&mut app);
    cell_node::initialize(&mut app);
    physics::initialize(&mut app);
    units::initialize(&mut app);
    camera::initialize(&mut app);

    app.run();
}
