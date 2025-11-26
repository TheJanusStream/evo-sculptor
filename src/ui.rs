use crate::{Selectable, generator, io, sculpt, state};
use bevy::asset::RenderAssetUsages;
use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui};
use bevy_panorbit_camera::PanOrbitCamera;
use image::{DynamicImage, ImageBuffer, Rgba, imageops::FilterType};
use std::io::Cursor;

pub fn ui_system(mut contexts: EguiContexts, mut evo_state: ResMut<state::EvoState>) {
    if let Ok(ctx) = contexts.ctx_mut() {
        egui::Window::new("Evo-Sculptor Controls").show(ctx, |ui| {
            ui.heading(format!("Generation: {}", evo_state.generation));
            ui.separator();

            ui.horizontal(|ui| {
                if ui.button("Evolve").clicked() && !evo_state.evolution_requested {
                    evo_state.evolution_requested = true;
                }
                if ui.button("Reset Population").clicked() {
                    println!("Reset button clicked! Generating new random population.");
                    *evo_state = state::EvoState::default();
                }
                //if ui.button("Log Activations").clicked() {
                //    evo_state.debug_requested = true;
                //}
                if ui.button("Export Selected").clicked() {
                    // 1. Find the selected genome/image
                    if let Some((index, _)) = evo_state
                        .fitness
                        .iter()
                        .enumerate()
                        .find(|&(_, &f)| f > 0.0)
                    {
                        let topology = &evo_state.genomes[index];

                        // 2. Re-generate the image (or retrieve it if you cached it)
                        let bevy_image = crate::generator::generate_image_from_topology(topology);

                        // 3. Convert Bevy Image to TGA bytes
                        // Note: ColorImage pixels are [r, g, b, a] bytes
                        let width = bevy_image.size[0] as u32;
                        let height = bevy_image.size[1] as u32;

                        let raw_data = bevy_image.as_raw().to_vec();

                        if let Some(buffer) =
                            ImageBuffer::<Rgba<u8>, _>::from_raw(width, height, raw_data)
                        {
                            // Convert to DynamicImage for easy resizing
                            let dynamic_image = DynamicImage::ImageRgba8(buffer);

                            // Resize to 64x64 using Nearest Neighbor (No interpolation/blur)
                            let resized_image =
                                dynamic_image.resize_exact(64, 64, FilterType::Nearest);

                            let mut bytes: Vec<u8> = Vec::new();

                            // Write TGA to the byte vector
                            resized_image
                                .write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Tga)
                                .unwrap();

                            // 4. Call our cross-platform saver
                            io::save_sculpt_map(bytes, &format!("sculpt_genome_{}.tga", index));
                        }
                    }
                }
            });
            ui.separator();
            ui.heading("Stitching Type");
            ui.horizontal(|ui| {
                if ui
                    .radio_value(
                        &mut evo_state.stitching_type,
                        state::StitchingType::Plane,
                        "Plane",
                    )
                    .clicked()
                {
                    evo_state.redraw_requested = true;
                }
                if ui
                    .radio_value(
                        &mut evo_state.stitching_type,
                        state::StitchingType::Sphere,
                        "Sphere",
                    )
                    .clicked()
                {
                    evo_state.redraw_requested = true;
                }
                if ui
                    .radio_value(
                        &mut evo_state.stitching_type,
                        state::StitchingType::Cylinder,
                        "Cylinder",
                    )
                    .clicked()
                {
                    evo_state.redraw_requested = true;
                }
                if ui
                    .radio_value(
                        &mut evo_state.stitching_type,
                        state::StitchingType::Torus,
                        "Torus",
                    )
                    .clicked()
                {
                    evo_state.redraw_requested = true;
                }
            });
        });
    }
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
        let sculpt_data = sculpt::create_sculpt_mesh(&image, 5.0, evo_state.stitching_type);

        let mut mesh = Mesh::new(
            bevy::mesh::PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
        );
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, sculpt_data.vertices);
        mesh.insert_indices(sculpt_data.indices);
        mesh.compute_normals();

        let handle = meshes.add(mesh);
        let material_handle = materials.add(StandardMaterial {
            base_color: Color::srgb(0.8, 0.7, 0.6),
            metallic: 0.2,
            perceptual_roughness: 0.6,
            ..default()
        });
        commands
            .spawn((
                Mesh3d(handle),
                MeshMaterial3d(material_handle),
                Transform::from_xyz(x, 0.0, z),
                Selectable {
                    index: i,
                    is_selected: false,
                },
            ))
            .observe(on_click_mesh);
    }

    // Add a primary directional light to cast shadows
    commands.spawn((
        DirectionalLight {
            illuminance: 10000.0, // A bright, sun-like intensity
            ..default()
        },
        Transform {
            // Position the light to cast interesting shadows from an angle
            translation: Vec3::new(10.0, 10.0, 10.0),
            rotation: Quat::from_rotation_x(-std::f32::consts::PI / 4.)
                * Quat::from_rotation_y(-std::f32::consts::PI / 4.),
            ..default()
        },
    ));

    commands.spawn((
        PanOrbitCamera {
            focus: Vec3::ZERO,
            button_orbit: MouseButton::Right,
            button_pan: MouseButton::Middle,
            ..default()
        },
        MeshPickingCamera,
    ));
}

fn on_click_mesh(
    click: On<Pointer<Press>>,
    mut contexts: EguiContexts,
    mut query: Query<&mut Selectable>,
    mut evo_state: ResMut<state::EvoState>,
) {
    // Prevent clicking "through" the UI
    if let Ok(ctx) = contexts.ctx_mut() {
        if ctx.is_pointer_over_area() {
            return;
        }
    }

    if let Ok(mut selectable) = query.get_mut(click.original_event_target()) {
        selectable.is_selected = !selectable.is_selected;
        evo_state.fitness[selectable.index] = if selectable.is_selected { 1.0 } else { 0.0 };
        println!(
            "Clicked index {}: is_selected = {}",
            selectable.index, selectable.is_selected
        );
    }
}

pub fn update_selection_materials(
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(&Selectable, &MeshMaterial3d<StandardMaterial>), Changed<Selectable>>,
) {
    for (selectable, mesh_material_handle) in &query {
        if let Some(material) = materials.get_mut(&mesh_material_handle.0) {
            println!(
                "Updating material for index {}: is_selected = {}",
                selectable.index, selectable.is_selected
            );
            material.emissive = if selectable.is_selected {
                Color::srgb(0.6, 0.8, 1.0).into()
            } else {
                Color::NONE.into()
            };
        }
    }
}
