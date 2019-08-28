use super::super::traits::label::Label;

pub enum Edge<T: Label> {
    Empty,
    ArrowTo(T),
    ArrowFrom(T)
}
