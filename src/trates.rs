pub trait Chromosome
where
    Self: Ord + Clone,
{
    fn new(uniform_rate: f64, mutation_rage: f64, genetic_len: Option<usize>) -> Self;
    fn get_fitness(&self) -> f64;
    fn mutate(&mut self);
    fn crossover(&self, other: &Self) -> Self;
}
