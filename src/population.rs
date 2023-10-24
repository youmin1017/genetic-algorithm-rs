use crate::trates::Chromosome;

pub struct Population<T>
where
    T: Chromosome,
{
    uniform_rate: f64,
    mutation_rate: f64,
    pub chromosomes: Vec<T>,
}

impl<T: Chromosome> Population<T> {
    pub fn new(uniform_rate: f64, mutation_rate: f64) -> Self {
        Self {
            uniform_rate,
            mutation_rate,
            chromosomes: vec![],
        }
    }

    pub fn initialize(&mut self, size: i32) {
        for _ in 0..size {
            let c = T::new(self.uniform_rate, self.mutation_rate, None);
            self.chromosomes.push(c);
        }
    }

    pub fn get_fittest(&self) -> T {
        self.chromosomes
            .iter()
            .max_by(|a, b| a.get_fitness().partial_cmp(&b.get_fitness()).unwrap())
            .unwrap()
            .clone()
    }

    pub fn size(&self) -> usize {
        self.chromosomes.len()
    }
}
