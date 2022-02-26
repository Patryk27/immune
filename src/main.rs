use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLinesPlugin;
use bevy_smud::SmudPlugin;
use unfair_advantage::systems::{camera, cell_node, input, units};

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::rgb_u8(1, 8, 90)))
        .add_plugins(DefaultPlugins)
        .add_plugin(SmudPlugin)
        .add_plugin(DebugLinesPlugin::default())
        // Map / Cells / Nodes
        .add_startup_system(cell_node::setup)
        .add_system(cell_node::factory_node_system);

    units::initialize(&mut app);
    input::initialize(&mut app);
    camera::initialize(&mut app);

    app.run();
}
