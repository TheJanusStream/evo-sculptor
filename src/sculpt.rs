use bevy_egui::egui;

// A struct to hold the generated mesh data before we hand it to Bevy
pub struct SculptMeshData {
    pub vertices: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
}

/// Generates a 3D mesh from a 2D ColorImage, interpreting RGB channels as XYZ coordinates.
pub fn create_sculpt_mesh(image: &egui::ColorImage, size: f32) -> SculptMeshData {
    let width = image.width();
    let height = image.height();

    // --- THIS IS THE CORE CHANGE ---
    // Create vertices by mapping each pixel's RGB to an XYZ coordinate.
    let vertices: Vec<[f32; 3]> = image
        .pixels
        .iter()
        .map(|pixel| {
            // Map R, G, B channels (0-255) to a coordinate space of [-0.5, 0.5] and then scale by size.
            let x = (pixel.r() as f32 / 255.0 - 0.5) * size;
            let y = (pixel.g() as f32 / 255.0 - 0.5) * size;
            let z = (pixel.b() as f32 / 255.0 - 0.5) * size;
            [x, y, z]
        })
        .collect();
    
    // The triangle indices remain the same, as they define the topology of the grid mesh.
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
    
    // --- NORMAL CALCULATION SIMPLIFIED ---
    // The old heightmap-based normal calculation is no longer valid.
    // For now, we will assign a default "up" normal to every vertex.
    // Bevy can use this for basic lighting. A more advanced calculation can be a future improvement.
    let normals: Vec<[f32; 3]> = vec![[0.0, 1.0, 0.0]; vertices.len()];

    SculptMeshData {
        vertices,
        normals,
        indices,
    }
}