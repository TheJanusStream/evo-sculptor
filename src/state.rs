use bevy::prelude::*;
use neat::NeuralNetworkTopology;
use neat::rand::thread_rng;

pub const POPULATION_SIZE: usize = 16;

#[derive(Resource)]
pub struct EvoState {
    pub genomes: Vec<NeuralNetworkTopology<3, 3>>,
    pub fitness: Vec<f32>, 
    pub generation: u64,
    pub evolution_requested: bool, // Added this flag
}

impl Default for EvoState {
    fn default() -> Self {
        let mut rng = thread_rng();
        
        let genomes: Vec<NeuralNetworkTopology<3, 3>> = (0..POPULATION_SIZE)
            .map(|_| NeuralNetworkTopology::new(0.1, 1, &mut rng))
            .collect();

        Self {
            genomes,
            fitness: vec![0.0; POPULATION_SIZE], 
            generation: 0,
            evolution_requested: false, // Default to false
        }
    }
}