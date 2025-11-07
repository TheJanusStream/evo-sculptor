use bevy::prelude::*;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::render::mesh::Indices;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

mod sculpt;
mod generator;
mod state;

#[derive(Component)]
struct Selectable {
    index: usize,
    is_selected: bool,
}

fn main() {
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
        ))
        .init_resource::<state::EvoState>()
        .add_systems(Startup, setup_scene)
        .add_systems(Update, (ui_system, raycast_system, update_selection_materials))
        .run();
}

fn ui_system(mut contexts: EguiContexts, evo_state: Res<state::EvoState>) {
    egui::Window::new("Evo-Sculptor Controls").show(contexts.ctx_mut(), |ui| {
        ui.heading(format!("Generation: {}", evo_state.generation));
        ui.separator();
        
        ui.horizontal(|ui| {
            if ui.button("Evolve").clicked() {
                println!("Evolve button clicked!");
            }
            if ui.button("Reset Population").clicked() {
                println!("Reset button clicked!");
            }
        });
    });
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    evo_state: Res<state::EvoState>,
) {
    let grid_size = 4;
    let spacing = 6.0;

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
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: Transform::from_xyz(x, 0.0, z),
                ..default()
            },
            Selectable { 
                index: i,
                is_selected: false,
            },
        ));
    }

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-12.0, 15.0, 12.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

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

/// Detects mouse clicks and toggles the selection state of sculptures.
fn raycast_system(
    mut contexts: EguiContexts,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mouse_buttons: Res<Input<MouseButton>>,
    // We now query for the GlobalTransform of each selectable entity as well.
    mut selectables: Query<(Entity, &mut Selectable, &GlobalTransform)>,
    meshes: Res<Assets<Mesh>>,
    mesh_handles: Query<&Handle<Mesh>>,
    mut evo_state: ResMut<state::EvoState>,
) {
    if contexts.ctx_mut().is_pointer_over_area() {
        return;
    }

    if mouse_buttons.just_pressed(MouseButton::Left) {
        let window = windows.single();
        let (camera, camera_transform) = cameras.single();

        if let Some(cursor_pos) = window.cursor_position() {
            if let Some(world_ray) = camera.viewport_to_world(camera_transform, cursor_pos) {
                let mut closest_intersection = f32::MAX;
                let mut closest_entity = None;

                // Iterate through entities with their transforms
                for (entity, _selectable, transform) in selectables.iter() {
                    if let Ok(mesh_handle) = mesh_handles.get(entity) {
                        if let Some(mesh) = meshes.get(mesh_handle) {
                            
                            // Calculate the inverse transform to move the ray into the mesh's local space
                            let inverse_transform = transform.compute_matrix().inverse();
                            let local_ray = Ray {
                                origin: inverse_transform.transform_point3(world_ray.origin),
                                direction: inverse_transform.transform_vector3(world_ray.direction),
                            };

                             if let Some(intersection) = ray_mesh_intersection(&local_ray, mesh) {
                                if intersection < closest_intersection {
                                    closest_intersection = intersection;
                                    closest_entity = Some(entity);
                                }
                            }
                        }
                    }
                }

                if let Some(entity) = closest_entity {
                    if let Ok((_, mut selectable, _)) = selectables.get_mut(entity) {
                        selectable.is_selected = !selectable.is_selected;
                        evo_state.fitness[selectable.index] = if selectable.is_selected { 1.0 } else { 0.0 };
                    }
                }
            }
        }
    }
}

/// A simple helper function to check for ray-mesh intersection.
fn ray_mesh_intersection(ray: &Ray, mesh: &Mesh) -> Option<f32> {
    if let (Some(bevy::render::mesh::VertexAttributeValues::Float32x3(vertices)), Some(Indices::U32(indices))) = 
        (mesh.attribute(Mesh::ATTRIBUTE_POSITION), mesh.indices()) {
            
        for i in (0..indices.len()).step_by(3) {
            let v0 = Vec3::from(vertices[indices[i] as usize]);
            let v1 = Vec3::from(vertices[indices[i+1] as usize]);
            let v2 = Vec3::from(vertices[indices[i+2] as usize]);

            let edge1 = v1 - v0;
            let edge2 = v2 - v0;
            let pvec = ray.direction.cross(edge2);
            let det = edge1.dot(pvec);

            if det.abs() < 1e-6 { continue; }

            let inv_det = 1.0 / det;
            let tvec = ray.origin - v0;
            let u = tvec.dot(pvec) * inv_det;
            if u < 0.0 || u > 1.0 { continue; }

            let qvec = tvec.cross(edge1);
            let v = ray.direction.dot(qvec) * inv_det;
            if v < 0.0 || u + v > 1.0 { continue; }

            let t = edge2.dot(qvec) * inv_det;
            if t > 1e-6 {
                return Some(t);
            }
        }
    }
    None
}


/// Updates the material of selected sculptures to give visual feedback.
fn update_selection_materials(
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(&Selectable, &Handle<StandardMaterial>)>
) {
    for (selectable, material_handle) in query.iter() {
        if let Some(material) = materials.get_mut(material_handle) {
            if selectable.is_selected {
                material.base_color = Color::rgb(0.8, 0.9, 1.0); 
            } else {
                material.base_color = Color::rgb(0.8, 0.7, 0.6); 
            }
        }
    }
}