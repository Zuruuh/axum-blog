use super::validation::ConstraintViolation;

#[derive(Debug)]
pub enum ApplicationLayerError {
    PersistenceError(Box<dyn std::error::Error>),
    ValidationError(Vec<ConstraintViolation>),
}
