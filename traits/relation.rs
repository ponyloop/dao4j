pub trait Relation {
    fn to_query_string(&self) -> String;
}
