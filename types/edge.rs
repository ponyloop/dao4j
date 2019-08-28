use super::super::traits::relation::Relation;

pub enum Edge<T: Relation> {
    Empty,
    Relation(T)
}
