use bevy_egui::egui;

use crate::state::StitchingType;

pub struct SculptMeshData {
    pub vertices: Vec<[f32; 3]>,
    pub indices: bevy::mesh::Indices,
}

pub fn create_sculpt_mesh(
    image: &egui::ColorImage,
    size: f32,
    stitching_type: StitchingType,
) -> SculptMeshData {
    let width = image.width();
    let height = image.height();

    let base_vertices: Vec<[f32; 3]> = image
        .pixels
        .iter()
        .map(|pixel| {
            let x = (pixel.r() as f32 / 255.0 - 0.5) * size;
            let y = (pixel.g() as f32 / 255.0 - 0.5) * size;
            let z = (pixel.b() as f32 / 255.0 - 0.5) * size;
            [x, y, z]
        })
        .collect();

    let mut indices: Vec<u32> = match stitching_type {
        StitchingType::Plane => Vec::with_capacity((width - 1) * (height - 1) * 6),
        StitchingType::Sphere => Vec::with_capacity((width) * (height) * 6),
        StitchingType::Cylinder => Vec::with_capacity((width) * (height - 1) * 6),
        StitchingType::Torus => Vec::with_capacity((width) * (height) * 6),
    };

    match stitching_type {
        // No stitching
        StitchingType::Plane => {
            for y in 0..(height - 1) {
                for x in 0..(width - 1) {
                    insert_quad(
                        &mut indices,
                        y * width + x,
                        y * width + x + width,
                        y * width + x + 1,
                        y * width + x + width + 1,
                    );
                }
            }
        }
        StitchingType::Sphere => {
            // Cap Bottom
            for x in 0..width {
                insert_quad(&mut indices, 0, width + x, 0, width + ((x + 1) % width));
            }
            // Stitching right to left
            for y in 0..(height - 1) {
                for x in 0..width {
                    insert_quad(
                        &mut indices,
                        y * width + x,
                        y * width + x + width,
                        y * width + ((x + 1) % width),
                        y * width + width + ((x + 1) % width),
                    );
                }
            }
            // Cap Top
            for x in 0..width {
                insert_quad(
                    &mut indices,
                    (height - 2) * width + x,
                    (height - 2) * width + width,
                    (height - 2) * width + ((x + 1) % width),
                    (height - 2) * width + width,
                );
            }
        }
        StitchingType::Cylinder => {
            // Stitching right to left
            for y in 0..(height - 1) {
                for x in 0..width {
                    insert_quad(
                        &mut indices,
                        y * width + x,
                        y * width + x + width,
                        y * width + ((x + 1) % width),
                        y * width + width + ((x + 1) % width),
                    );
                }
            }
        }
        StitchingType::Torus => {
            // Stitching Right to Reft & Top to Bottom
            for y in 0..height {
                for x in 0..width {
                    insert_quad(
                        &mut indices,
                        y * width + x,
                        (((y + 1) * width) % (height * width)) + x,
                        y * width + ((x + 1) % width),
                        (((y + 1) * width) % (height * width)) + ((x + 1) % width),
                    );
                }
            }
        }
    }

    SculptMeshData {
        vertices: base_vertices,
        indices: bevy::mesh::Indices::U32(indices),
    }
}

fn insert_quad(indices: &mut Vec<u32>, a: usize, b: usize, c: usize, d: usize) {
    indices.push(a as u32);
    indices.push(b as u32);
    indices.push(c as u32);

    indices.push(c as u32);
    indices.push(b as u32);
    indices.push(d as u32);
}
