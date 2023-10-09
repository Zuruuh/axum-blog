#[derive(serde::Serialize)]
pub struct ConstraintViolation {
    message: String,
    property: String,
}

impl ConstraintViolation {
    pub fn new(message: String, property: String) -> Self {
        Self { message, property }
    }
}
