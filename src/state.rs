use crate::activations::*;
use bevy::prelude::*;
use neat::activation::{ActivationScope, linear_activation, relu, sigmoid};
use neat::rand::{Rng, thread_rng};
use neat::{ActivationFn, NeuralNetworkTopology, activation_fn};
use std::sync::Arc;

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
    pub grid_size: usize,
    pub grid_spawn_requested: bool,
}

impl EvoState {
    pub fn get_population_size(&self) -> usize {
        self.grid_size * self.grid_size
    }

    pub fn resize_grid(&mut self, new_size: usize) {
        if self.grid_size == new_size {
            return;
        }

        self.grid_size = new_size;
        let target_pop = new_size * new_size;
        let mut rng = thread_rng();

        if self.genomes.len() < target_pop {
            let additional = target_pop - self.genomes.len();
            for _ in 0..additional {
                let mut genome = NeuralNetworkTopology::new(0.2, 3, &mut rng);
                Self::diversify_genome(&mut genome, &mut rng);
                self.genomes.push(genome);
            }
        } else {
            self.genomes.truncate(target_pop);
        }

        self.fitness.resize(target_pop, 0.0);
        self.grid_spawn_requested = true;
    }

    fn diversify_genome(genome: &mut NeuralNetworkTopology<3, 3>, rng: &mut impl Rng) {
        let output_activations = activation_fn! {
            sigmoid => ActivationScope::OUTPUT,
            relu => ActivationScope::OUTPUT,
            f32::tanh => ActivationScope::OUTPUT,
            linear_activation => ActivationScope::OUTPUT,
            sin_activation => ActivationScope::OUTPUT,
            cos_activation => ActivationScope::OUTPUT,
            gaussian_activation => ActivationScope::OUTPUT,
            abs_activation => ActivationScope::OUTPUT,
            square_activation => ActivationScope::OUTPUT,
            step_activation => ActivationScope::OUTPUT,
            clamp_activation => ActivationScope::OUTPUT,
            pulse_activation => ActivationScope::OUTPUT,
            staircase_activation => ActivationScope::OUTPUT
        };

        for neuron_arc in &genome.output_layer {
            let mut neuron = neuron_arc.write().unwrap();
            let new_activation =
                output_activations[rng.gen_range(0..output_activations.len())].clone();
            neuron.activation = new_activation;
        }
    }
}

impl Default for EvoState {
    fn default() -> Self {
        let mut rng = thread_rng();
        let grid_size = 4;
        let pop_size = grid_size * grid_size;

        let mut genomes: Vec<NeuralNetworkTopology<3, 3>> = (0..pop_size)
            .map(|_| NeuralNetworkTopology::new(0.2, 3, &mut rng))
            .collect();

        for genome in &mut genomes {
            Self::diversify_genome(genome, &mut rng);
        }

        Self {
            genomes,
            fitness: vec![0.0; pop_size],
            generation: 0,
            evolution_requested: false,
            debug_requested: false,
            stitching_type: StitchingType::default(),
            redraw_requested: true,
            grid_size,
            grid_spawn_requested: true,
        }
    }
}
