use bevy::prelude::*;
use neat::NeuralNetworkTopology;
use neat::rand::thread_rng;

const POPULATION_SIZE: usize = 16;

#[derive(Resource)]
pub struct EvoState {
    pub genomes: Vec<NeuralNetworkTopology<3, 3>>,
    // We now store fitness alongside the genomes.
    pub fitness: Vec<f32>, 
    pub generation: u64,
}

impl Default for EvoState {
    fn default() -> Self {
        let mut rng = thread_rng();
        
        let genomes: Vec<NeuralNetworkTopology<3, 3>> = (0..POPULATION_SIZE)
            .map(|_| NeuralNetworkTopology::new(0.1, 1, &mut rng))
            .collect();

        Self {
            genomes,
            // Initialize all fitness scores to 0.
            fitness: vec![0.0; POPULATION_SIZE], 
            generation: 0,
        }
    }
}