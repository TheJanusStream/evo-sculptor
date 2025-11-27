// src/main.rs
use bevy::prelude::*;
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};
use bevy_panorbit_camera::PanOrbitCameraPlugin;

mod activations;
mod evolution;
mod generator;
mod io;
mod sculpt;
mod state;
mod ui;

#[derive(Component)]
pub struct Selectable {
    pub index: usize,
    pub is_selected: bool,
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
            EguiPlugin::default(),
            PanOrbitCameraPlugin,
            MeshPickingPlugin,
        ))
        .init_resource::<state::EvoState>()
        .add_systems(Startup, ui::setup_camera_lights)
        .add_systems(Update, ui::spawn_grid_system)
        .add_systems(EguiPrimaryContextPass, ui::ui_system)
        .add_systems(
            Update,
            (
                ui::update_selection_materials,
                evolution::log_activation_distribution,
                evolution::evolve_system,
                evolution::update_meshes_system,
            )
                .chain(),
        )
        .run();
}
