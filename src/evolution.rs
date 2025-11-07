use bevy::prelude::*;
use neat::rand::Rng;
use neat::CrossoverReproduction;
use std::mem;

use crate::{generator, sculpt, state, Selectable, POPULATION_SIZE};

pub fn evolve_system(mut evo_state: ResMut<state::EvoState>) {
    if !evo_state.evolution_requested {
        return;
    }
    println!("Evolving generation {}...", evo_state.generation);
    
    let genomes = mem::take(&mut evo_state.genomes);
    let fitnesses = mem::take(&mut evo_state.fitness);

    let mut population_with_fitness: Vec<_> =
        genomes.into_iter().zip(fitnesses.into_iter()).collect();

    population_with_fitness.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    
    let mut champions: Vec<_> =
        population_with_fitness.into_iter()
        .filter(|(_, fitness)| *fitness > 0.0)
        .map(|(genome, _)| genome)
        .collect();
    
    if champions.is_empty() {
        println!("No champions selected! This is a bug. Repopulating with random new genomes.");
        let mut rng = neat::rand::thread_rng();
        evo_state.genomes = (0..POPULATION_SIZE)
            .map(|_| neat::NeuralNetworkTopology::new(0.1, 1, &mut rng))
            .collect();
        evo_state.generation += 1;
        evo_state.fitness = vec![0.0; POPULATION_SIZE];
        evo_state.evolution_requested = false;
        return;
    }
    
    let champions_count = (POPULATION_SIZE / 2).max(1);
    champions.truncate(champions_count);

    let mut rng = neat::rand::thread_rng();
    let mut next_generation = champions.clone();
    
    while next_generation.len() < POPULATION_SIZE {
        let parent1 = &champions[rng.gen_range(0..champions.len())];
        let parent2 = &champions[rng.gen_range(0..champions.len())];

        let child = parent1.crossover(parent2, &mut rng);
        next_generation.push(child);
    }
    
    evo_state.genomes = next_generation;
    evo_state.generation += 1;
    evo_state.fitness = vec![0.0; POPULATION_SIZE];
    evo_state.evolution_requested = false;
    
    println!("Evolution complete. Now at generation {}.", evo_state.generation);
}

pub fn update_meshes_system(
    mut query: Query<(&mut Selectable, &Handle<Mesh>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    evo_state: Res<state::EvoState>,
) {
    if evo_state.is_changed() && !evo_state.is_added() {
        if !evo_state.evolution_requested {
            println!("Updating meshes for new generation...");
            for (mut selectable, mesh_handle) in query.iter_mut() {
                if let Some(mesh) = meshes.get_mut(mesh_handle) {
                    
                    let new_topology = &evo_state.genomes[selectable.index];

                    let image = generator::generate_image_from_topology(new_topology);
                    let sculpt_data = sculpt::create_sculpt_mesh(&image, 5.0);

                    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, sculpt_data.vertices);
                    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, sculpt_data.normals);
                    
                    selectable.is_selected = false;
                }
            }
        }
    }
}
