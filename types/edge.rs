use super::super::traits::relation::Label;

pub enum Edge<T: Label> {
    Empty,
    ArrowTo(T),
    ArrowFrom(T)
}
