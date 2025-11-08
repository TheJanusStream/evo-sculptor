use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy_egui::EguiContexts;

use crate::{state, Selectable};

#[allow(clippy::too_many_arguments)]
pub fn raycast_system(
    mut contexts: EguiContexts,
    windows: Query<&Window>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    mouse_buttons: Res<Input<MouseButton>>,
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

                for (entity, _selectable, transform) in selectables.iter() {
                    if let Ok(mesh_handle) = mesh_handles.get(entity) {
                        if let Some(mesh) = meshes.get(mesh_handle) {
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
                        evo_state.fitness[selectable.index] =
                            if selectable.is_selected { 1.0 } else { 0.0 };
                    }
                }
            }
        }
    }
}

fn ray_mesh_intersection(ray: &Ray, mesh: &Mesh) -> Option<f32> {
    if let (
        Some(bevy::render::mesh::VertexAttributeValues::Float32x3(vertices)),
        Some(Indices::U32(indices)),
    ) = (mesh.attribute(Mesh::ATTRIBUTE_POSITION), mesh.indices())
    {
        for i in (0..indices.len()).step_by(3) {
            let v0 = Vec3::from(vertices[indices[i] as usize]);
            let v1 = Vec3::from(vertices[indices[i + 1] as usize]);
            let v2 = Vec3::from(vertices[indices[i + 2] as usize]);

            let edge1 = v1 - v0;
            let edge2 = v2 - v0;
            let pvec = ray.direction.cross(edge2);
            let det = edge1.dot(pvec);

            if det.abs() < 1e-6 {
                continue;
            }

            let inv_det = 1.0 / det;
            let tvec = ray.origin - v0;
            let u = tvec.dot(pvec) * inv_det;
            if !(0.0..=1.0).contains(&u) {
                continue;
            }

            let qvec = tvec.cross(edge1);
            let v = ray.direction.dot(qvec) * inv_det;
            if v < 0.0 || u + v > 1.0 {
                continue;
            }

            let t = edge2.dot(qvec) * inv_det;
            if t > 1e-6 {
                return Some(t);
            }
        }
    }
    None
}

pub fn update_selection_materials(
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(&Selectable, &Handle<StandardMaterial>)>,
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
