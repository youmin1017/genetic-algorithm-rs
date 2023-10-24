use rand::{thread_rng, Rng};

use crate::trates::Chromosome;

#[derive(Debug, Clone)]
pub struct SqrtChromosome {
    genetic_len: usize,
    genetic: Vec<bool>,
    fitness: f64,
    target: u8,
    uniform_rate: f64,
    mutation_rate: f64,
}

impl Chromosome for SqrtChromosome {
    fn new(uniform_rate: f64, mutation_rate: f64, genetic_len: Option<usize>) -> Self {
        let len = genetic_len.unwrap_or(128);
        let mut rng = thread_rng();
        Self {
            genetic_len: len,
            genetic: (0..len).map(|_| rng.gen()).collect(),
            fitness: 0_f64,
            target: 5,
            uniform_rate,
            mutation_rate,
        }
    }

    // [0..16] -> number part
    // [16..] -> fraction part
    fn get_fitness(&self) -> f64 {
        if self.fitness == 0_f64 {
            self.calculate_fitness()
        } else {
            self.fitness
        }
    }

    fn mutate(&mut self) {
        let mut rng = thread_rng();
        for i in 1..self.genetic_len {
            if rng.gen_bool(self.mutation_rate) {
                self.genetic[i] = rng.gen();
            }
        }
    }

    fn crossover(&self, other: &Self) -> Self {
        let mut new_chromosome = Self::new(self.uniform_rate, self.mutation_rate, None);
        let mut rng = thread_rng();
        new_chromosome.genetic = self
            .genetic
            .iter()
            .zip(other.genetic.iter())
            .map(|(x, y)| {
                if rng.gen_bool(self.uniform_rate) {
                    x.clone()
                } else {
                    y.clone()
                }
            })
            .collect();
        new_chromosome
    }
}

impl SqrtChromosome {
    fn calculate_fitness(&self) -> f64 {
        let (x, y) = self.genetic.split_at(16);
        let num: u32 = x.iter().fold(0, |acc, &x| acc << 1 | x as u32);
        let fra: f64 = y
            .iter()
            .enumerate()
            .fold(0_f64, |acc, (i, &cur)| match cur {
                true => acc + 2_f64.powi(-(i as i32 + 1)),
                false => acc,
            });
        // y.iter().map(|&b| if b { 1 } else { 0 }).for_each(|b| eprint!("{}", b));
        // sqrt 5 is 2.2360679775
        let dec = num as f64 + fra;
        let dis = (self.target as f64 - dec.powi(2)).abs();
        // eprintln!(", {:.10}", fra);
        1_f64 - dis
    }

    pub fn get_result(&self) -> f64 {
        let (x, y) = self.genetic.split_at(16);
        let num: u32 = x.iter().fold(0, |acc, &x| acc << 1 | x as u32);
        let fra: f64 = y
            .iter()
            .enumerate()
            .fold(0_f64, |acc, (i, &cur)| match cur {
                true => acc + 2_f64.powi(-(i as i32 + 1)),
                false => acc,
            });
        let dec = num as f64 + fra;
        dec
    }

    pub fn set_target(&mut self, target: u8) {
        self.target = target;
    }
}

impl Eq for SqrtChromosome {}
impl PartialEq for SqrtChromosome {
    fn eq(&self, other: &Self) -> bool {
        self.genetic.eq(&other.genetic)
    }
}

impl Ord for SqrtChromosome {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self
            .fitness
            .partial_cmp(&other.fitness)
            .unwrap_or(std::cmp::Ordering::Equal);
    }
}

impl PartialOrd for SqrtChromosome {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.fitness.partial_cmp(&other.fitness)
    }
}
