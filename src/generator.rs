use bevy_egui::egui::{self, Color32};
use neat::{NeuralNetwork, NeuralNetworkTopology};

pub fn generate_image_from_topology(topology: &NeuralNetworkTopology<3, 3>) -> egui::ColorImage {
    let network = NeuralNetwork::from(topology);
    let width = 32;
    let height = 32;
    let mut image = egui::ColorImage::new([width, height], vec![Color32::BLACK; width * height]);
    const EPSILON: f32 = 1e-6;

    // --- PASS 1: Collect raw f32 outputs and find min/max for each channel ---
    let mut r_values = Vec::with_capacity(width * height);
    let mut g_values = Vec::with_capacity(width * height);
    let mut b_values = Vec::with_capacity(width * height);

    let mut min_r = f32::MAX;
    let mut max_r = f32::MIN;
    let mut min_g = f32::MAX;
    let mut max_g = f32::MIN;
    let mut min_b = f32::MAX;
    let mut max_b = f32::MIN;

    for y in 0..height {
        for x in 0..width {
            let norm_x = (x as f32 / (width - 1) as f32) * 2.0 - 1.0;
            let norm_y = (y as f32 / (height - 1) as f32) * 2.0 - 1.0;
            let dist_from_center = (norm_x.powi(2) + norm_y.powi(2)).sqrt();

            let inputs = [norm_x, norm_y, dist_from_center];

            network.flush_state();
            let outputs = network.predict(inputs);

            let r = outputs[0];
            let g = outputs[1];
            let b = outputs[2];

            r_values.push(r);
            g_values.push(g);
            b_values.push(b);

            min_r = min_r.min(r);
            max_r = max_r.max(r);
            min_g = min_g.min(g);
            max_g = max_g.max(g);
            min_b = min_b.min(b);
            max_b = max_b.max(b);
        }
    }

    let range_r = max_r - min_r;
    let range_g = max_g - min_g;
    let range_b = max_b - min_b;

    // --- PASS 2: Normalize raw values and create the final image ---
    for (i, pixel) in image.pixels.iter_mut().enumerate() {
        let norm_r = if range_r < EPSILON {
            0.5
        } else {
            (r_values[i] - min_r) / range_r
        };
        let norm_g = if range_g < EPSILON {
            0.5
        } else {
            (g_values[i] - min_g) / range_g
        };
        let norm_b = if range_b < EPSILON {
            0.5
        } else {
            (b_values[i] - min_b) / range_b
        };

        *pixel = egui::Color32::from_rgb(
            (norm_r.clamp(0.0, 1.0) * 255.0) as u8,
            (norm_g.clamp(0.0, 1.0) * 255.0) as u8,
            (norm_b.clamp(0.0, 1.0) * 255.0) as u8,
        );
    }

    image
}
