use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_prototype_debug_lines::DebugLinesPlugin;
use unfair_advantage::systems::{camera, cell_node, input, physics, ui, units};

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::rgb_u8(1, 8, 90)))
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugLinesPlugin::default())
        .add_plugin(EguiPlugin)
        .add_plugin(ui::UiPlugin)
        // Map / Cells / Nodes
        .add_startup_system(cell_node::setup)
        .add_system(cell_node::process)
        .add_system(cell_node::track_cells_position);

    physics::initialize(&mut app);
    units::initialize(&mut app);
    input::initialize(&mut app);
    camera::initialize(&mut app);

    app.run();
}
