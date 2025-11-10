use bevy::prelude::*;
use neat::CrossoverReproduction;
use neat::rand::Rng;
use std::collections::HashMap;
use std::mem;

use crate::{POPULATION_SIZE, Selectable, generator, sculpt, state};

pub fn log_activation_distribution(mut evo_state: ResMut<state::EvoState>) {
    if !evo_state.debug_requested {
        return;
    }

    println!(
        "\n--- Activation Function Distribution (Generation {}) ---",
        evo_state.generation
    );
    let mut distribution: HashMap<String, usize> = HashMap::new();

    for genome in &evo_state.genomes {
        // Check input layer
        for neuron_arc in &genome.input_layer {
            let neuron = neuron_arc.read().unwrap();
            *distribution
                .entry(format!("{:?}", neuron.activation))
                .or_insert(0) += 1;
        }
        // Check hidden layer
        for neuron_arc in &genome.hidden_layers {
            let neuron = neuron_arc.read().unwrap();
            *distribution
                .entry(format!("{:?}", neuron.activation))
                .or_insert(0) += 1;
        }
        // Check output layer
        for neuron_arc in &genome.output_layer {
            let neuron = neuron_arc.read().unwrap();
            *distribution
                .entry(format!("{:?}", neuron.activation))
                .or_insert(0) += 1;
        }
    }

    for (name, count) in &distribution {
        // The debug format for ActivationFn includes a newline, so we trim it.
        println!("- {}: {}", name.trim(), count);
    }
    println!("---\n");

    // Reset the flag
    evo_state.debug_requested = false;
}

pub fn evolve_system(mut evo_state: ResMut<state::EvoState>) {
    if !evo_state.evolution_requested {
        return;
    }
    println!("Evolving generation {}...", evo_state.generation);

    let genomes = mem::take(&mut evo_state.genomes);
    let fitnesses = mem::take(&mut evo_state.fitness);

    let population_with_fitness: Vec<_> = genomes.into_iter().zip(fitnesses).collect();

    let champions: Vec<_> = population_with_fitness
        .iter()
        .filter(|(_, fitness)| *fitness > 0.0)
        .map(|(genome, _)| genome.clone())
        .collect();

    let parents = if champions.is_empty() {
        println!("No champions selected! Using the entire previous generation as parents.");
        population_with_fitness
            .into_iter()
            .map(|(genome, _)| genome)
            .collect()
    } else {
        champions
    };
    let mut rng = neat::rand::thread_rng();
    let mut next_generation = Vec::with_capacity(POPULATION_SIZE);

    while next_generation.len() < POPULATION_SIZE {
        let parent1 = &parents[rng.gen_range(0..parents.len())];
        let parent2 = &parents[rng.gen_range(0..parents.len())];

        let child = parent1.crossover(parent2, &mut rng);
        next_generation.push(child);
    }

    evo_state.genomes = next_generation;
    evo_state.generation += 1;
    evo_state.fitness = vec![0.0; POPULATION_SIZE];
    evo_state.evolution_requested = false;
    evo_state.redraw_requested = true;

    println!(
        "Evolution complete. Now at generation {}.",
        evo_state.generation
    );
}

pub fn update_meshes_system(
    mut query: Query<(&mut Selectable, &Mesh3d)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut evo_state: ResMut<state::EvoState>,
) {
    if evo_state.is_changed()
        && !evo_state.is_added()
        && !evo_state.evolution_requested
        && evo_state.redraw_requested
    {
        println!("Updating meshes for new generation...");
        for (mut selectable, mesh_handle) in query.iter_mut() {
            if let Some(mesh) = meshes.get_mut(mesh_handle) {
                let new_topology = &evo_state.genomes[selectable.index];

                let image = generator::generate_image_from_topology(new_topology);
                let sculpt_data = sculpt::create_sculpt_mesh(&image, 5.0, evo_state.stitching_type);

                mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, sculpt_data.vertices);
                mesh.insert_indices(sculpt_data.indices);
                mesh.compute_smooth_normals();

                selectable.is_selected = false;
            }
        }
        evo_state.redraw_requested = false;
    }
}
