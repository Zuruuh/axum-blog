#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ConstraintViolationLocation {
    Body,
    Path,
    // Query,
}

#[derive(Debug, serde::Serialize)]
pub struct ConstraintViolation {
    message: String,
    property: String,
    r#in: ConstraintViolationLocation,
}

impl ConstraintViolation {
    pub fn new(message: String, property: String, r#in: ConstraintViolationLocation) -> Self {
        Self {
            message,
            property,
            r#in,
        }
    }
}
