use bevy::prelude::*;
use bevy_egui::egui;

// A struct to hold the generated mesh data before we hand it to Bevy
pub struct SculptMeshData {
    pub vertices: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
}

/// Generates a 3D mesh from a 2D ColorImage, interpreting brightness as height.
pub fn create_sculpt_mesh(image: &egui::ColorImage, size: f32) -> SculptMeshData {
    let width = image.width();
    let height = image.height();

    // Create vertices from image pixels
    let vertices: Vec<[f32; 3]> = image
        .pixels
        .iter()
        .enumerate()
        .map(|(i, pixel)| {
            // Use luminance for height to handle color images
            let height_value = pixel.r() as f32 * 0.299 + pixel.g() as f32 * 0.587 + pixel.b() as f32 * 0.114;
            [
                size * ((i % width) as f32 / (width - 1) as f32 - 0.5),
                (height_value / 255.0) * 0.5, // Scale height to a reasonable range
                size * (height as f32 / width as f32) * ((i / width) as f32 / (height - 1) as f32 - 0.5),
            ]
        })
        .collect();
    
    // Create the triangle indices
    let mut indices: Vec<u32> = Vec::with_capacity((width - 1) * (height - 1) * 6);
    for y in 0..(height - 1) {
        for x in 0..(width - 1) {
            let base = (y * width + x) as u32;
            indices.push(base);
            indices.push(base + width as u32);
            indices.push(base + 1);

            indices.push(base + 1);
            indices.push(base + width as u32);
            indices.push(base + width as u32 + 1);
        }
    }
    
    // Calculate normals for lighting
    let mut normals: Vec<[f32; 3]> = vec![[0.0, 1.0, 0.0]; vertices.len()];
    for y in 1..(height - 1) {
        for x in 1..(width - 1) {
            let h_l = vertices[(y * width) + x - 1][1];
            let h_r = vertices[(y * width) + x + 1][1];
            let h_d = vertices[((y - 1) * width) + x][1];
            let h_u = vertices[((y + 1) * width) + x][1];
            let normal = Vec3::new(h_l - h_r, 2.0, h_d - h_u).normalize();
            normals[y * width + x] = [normal.x, normal.y, normal.z];
        }
    }

    SculptMeshData {
        vertices,
        normals,
        indices,
    }
}