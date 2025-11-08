use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_panorbit_camera::PanOrbitCameraPlugin;

mod sculpt;
mod generator;
mod state;
mod ui;
mod interaction;
mod evolution;
mod activations;

use crate::state::POPULATION_SIZE;

#[derive(Component)]
struct Selectable {
    index: usize,
    is_selected: bool,
}

fn main() {
    activations::register_custom_activations();
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Evo-Sculptor".into(),
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }),
            EguiPlugin,
            PanOrbitCameraPlugin,
        ))
        .init_resource::<state::EvoState>()
        .add_systems(Startup, ui::setup_scene)
        .add_systems(Update, (ui::ui_system, interaction::raycast_system, interaction::update_selection_materials, evolution::log_activation_distribution).chain())
        .add_systems(Update, (ui::ui_system, interaction::raycast_system, interaction::update_selection_materials).chain())
        .add_systems(Update, (evolution::evolve_system, evolution::update_meshes_system).chain())
        .run();
}
