use super::id::Id;

trait Naming {
    fn name(id: Id) -> Option<String>;
}