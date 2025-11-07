use bevy_egui::egui;

pub struct SculptMeshData {
    pub vertices: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
}

pub fn create_sculpt_mesh(image: &egui::ColorImage, size: f32) -> SculptMeshData {
    let width = image.width();
    let height = image.height();

    let vertices: Vec<[f32; 3]> = image
        .pixels
        .iter()
        .map(|pixel| {
            let x = (pixel.r() as f32 / 255.0 - 0.5) * size;
            let y = (pixel.g() as f32 / 255.0 - 0.5) * size;
            let z = (pixel.b() as f32 / 255.0 - 0.5) * size;
            [x, y, z]
        })
        .collect();
    
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
    
    let normals: Vec<[f32; 3]> = vec![[0.0, 1.0, 0.0]; vertices.len()];

    SculptMeshData {
        vertices,
        normals,
        indices,
    }
}
