
pub mod trates;
pub mod genetic_algorithm;
pub mod population;
pub mod sqrt_chromosome;

use population::Population;
use sqrt_chromosome::SqrtChromosome;
use genetic_algorithm::GeneticAlgorithm;
use trates::Chromosome;
// use trates::Chromosome;

fn main() {
    let uniform_rate = 0.5;
    let mutation_rate = 0.015;
    let tournament_size = 5;
    let elitism = true;
    let mut population = Population::<SqrtChromosome>::new(uniform_rate, mutation_rate);
    let mut ga = GeneticAlgorithm::new(uniform_rate, mutation_rate, tournament_size, elitism);
    let mut generation_count = 0;
    population.initialize(500);

    while population.get_fittest().get_fitness() <= 1_f64 - 10_f64.powi(-11) {
        // 2.2360679775
        generation_count += 1;
        println!(
            "Generation: {} Fittest: {:.10} Reutls: {:.10}",
            generation_count,
            population.get_fittest().get_fitness(),
            population.get_fittest().get_result()
        );
        population = ga.evolve_population(&mut population);
    }
}