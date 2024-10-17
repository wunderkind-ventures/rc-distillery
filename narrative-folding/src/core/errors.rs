pub enum SimulationError {
    InvalidState,
    ConvergenceFailure,
    // Other error types...
}

pub enum OptimizationError {
    LocalMinimum,
    ConstraintViolation,
    // Other error types...
}
