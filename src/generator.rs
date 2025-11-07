use bevy_egui::egui;
use neat::{NeuralNetwork, NeuralNetworkTopology};

/// Generates a 32x32 image by evaluating a neural network topology as a CPPN.
pub fn generate_image_from_topology(topology: &NeuralNetworkTopology<3, 3>) -> egui::ColorImage {
    let network = NeuralNetwork::from(topology);
    let width = 32;
    let height = 32;
    let mut image = egui::ColorImage::new([width, height], egui::Color32::BLACK);

    for y in 0..height {
        for x in 0..width {
            let norm_x = (x as f32 / (width - 1) as f32) * 2.0 - 1.0;
            let norm_y = (y as f32 / (height - 1) as f32) * 2.0 - 1.0;
            let dist_from_center = (norm_x.powi(2) + norm_y.powi(2)).sqrt();

            let inputs = [norm_x, norm_y, dist_from_center];
            
            network.flush_state();
            let outputs = network.predict(inputs);

            let r = (outputs[0].clamp(0.0, 1.0) * 255.0) as u8;
            let g = (outputs[1].clamp(0.0, 1.0) * 255.0) as u8;
            let b = (outputs[2].clamp(0.0, 1.0) * 255.0) as u8;
            
            image[(x, y)] = egui::Color32::from_rgb(r, g, b);
        }
    }
    image
}