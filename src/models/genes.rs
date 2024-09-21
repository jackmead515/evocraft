
pub trait Genes {
    fn code(&self) -> Vec<String>;
    fn mutate(&mut self);
}