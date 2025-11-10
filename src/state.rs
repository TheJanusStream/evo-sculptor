use crate::activations::*;
use bevy::prelude::*;
use neat::activation::{linear_activation, relu, sigmoid, ActivationScope};
use neat::rand::{thread_rng, Rng};
use neat::{activation_fn, ActivationFn, NeuralNetworkTopology};
use std::sync::Arc;

pub const POPULATION_SIZE: usize = 16;

#[derive(Debug, PartialEq, Clone, Copy, Default)]
pub enum StitchingType {
    #[default]
    Plane,
    Sphere,
    Cylinder,
    Torus,
}

#[derive(Resource)]
pub struct EvoState {
    pub genomes: Vec<NeuralNetworkTopology<3, 3>>,
    pub fitness: Vec<f32>,
    pub generation: u64,
    pub evolution_requested: bool,
    pub debug_requested: bool,
    pub stitching_type: StitchingType,
    pub redraw_requested: bool,
}

impl Default for EvoState {
    fn default() -> Self {
        let mut rng = thread_rng();

        let mut genomes: Vec<NeuralNetworkTopology<3, 3>> = (0..POPULATION_SIZE)
            .map(|_| NeuralNetworkTopology::new(0.2, 3, &mut rng))
            .collect();

        println!("Diversifying initial activation functions in the output layer...");
        let output_activations = activation_fn! {
            // Default functions from the `neat` crate
            sigmoid => ActivationScope::OUTPUT,
            relu => ActivationScope::OUTPUT,
            f32::tanh => ActivationScope::OUTPUT,
            linear_activation => ActivationScope::OUTPUT,
            // Our custom functions
            sin_activation => ActivationScope::OUTPUT,
            cos_activation => ActivationScope::OUTPUT,
            gaussian_activation => ActivationScope::OUTPUT,
            abs_activation => ActivationScope::OUTPUT,
            square_activation => ActivationScope::OUTPUT
        };

        for genome in &mut genomes {
            for neuron_arc in &genome.output_layer {
                let mut neuron = neuron_arc.write().unwrap();
                // Select a random function from our manually created list
                let new_activation =
                    output_activations[rng.gen_range(0..output_activations.len())].clone();
                neuron.activation = new_activation;
            }
        }
        println!("Diversification complete.");

        Self {
            genomes,
            fitness: vec![0.0; POPULATION_SIZE],
            generation: 0,
            evolution_requested: false,
            debug_requested: false,
            stitching_type: StitchingType::default(),
            redraw_requested: true, // Request a redraw for the initial population
        }
    }
}
