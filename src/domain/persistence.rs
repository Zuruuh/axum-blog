#[derive(Debug)]
pub enum PersistenceError {
    UncheckedError(Box<dyn std::error::Error>),
}
