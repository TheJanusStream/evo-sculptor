use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy_egui::{egui, EguiContexts};
use bevy_panorbit_camera::PanOrbitCamera;

use crate::{generator, sculpt, state, Selectable};

pub fn ui_system(mut contexts: EguiContexts, mut evo_state: ResMut<state::EvoState>) {
    egui::Window::new("Evo-Sculptor Controls").show(contexts.ctx_mut(), |ui| {
        ui.heading(format!("Generation: {}", evo_state.generation));
        ui.separator();

        ui.horizontal(|ui| {
            if ui.button("Evolve").clicked() {
                if !evo_state.evolution_requested {
                    evo_state.evolution_requested = true;
                }
            }
            if ui.button("Reset Population").clicked() {
                println!("Reset button clicked! Generating new random population.");
                *evo_state = state::EvoState::default();
            }
            if ui.button("Log Activations").clicked() {
                evo_state.debug_requested = true;
            }
        });
    });
}

pub fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    evo_state: Res<state::EvoState>,
) {
    let grid_size = 4;
    let spacing = 10.0;

    for (i, topology) in evo_state.genomes.iter().enumerate() {
        let x = (i % grid_size) as f32 * spacing - (spacing * (grid_size - 1) as f32) / 2.0;
        let z = (i / grid_size) as f32 * spacing - (spacing * (grid_size - 1) as f32) / 2.0;

        let image = generator::generate_image_from_topology(topology);
        let sculpt_data = sculpt::create_sculpt_mesh(&image, 5.0);

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, sculpt_data.vertices);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, sculpt_data.normals);
        mesh.set_indices(Some(Indices::U32(sculpt_data.indices)));
        let handle = meshes.add(mesh);

        commands.spawn((
            PbrBundle {
                mesh: handle,
                material: materials.add(StandardMaterial {
                    base_color: Color::rgb(0.8, 0.7, 0.6),
                    ..default()
                }),
                transform: Transform::from_xyz(x, 0.0, z),
                ..default()
            },
            Selectable {
                index: i,
                is_selected: false,
            },
        ));
    }

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-12.0, 15.0, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PanOrbitCamera {
            focus: Vec3::ZERO,
            button_orbit: MouseButton::Right,
            button_pan: MouseButton::Middle,
            ..default()
        },
    ));

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 6000.0,
            range: 100.0,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 20.0, 0.0),
        ..default()
    });
}
