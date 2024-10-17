pub trait Simulatable {
    fn step(&mut self) -> Result<(), SimulationError>;
    fn run(&mut self, steps: usize) -> Result<(), SimulationError>;
}

pub trait Optimizable {
    fn objective_function(&self) -> f64;
    fn optimize(&mut self) -> Result<(), OptimizationError>;
}

pub trait Visualizable {
    fn to_graph(&self) -> Graph;
    fn plot(&self) -> Plot;
}

pub trait Serializable {
    fn to_json(&self) -> String;
    fn from_json(json: &str) -> Result<Self, DeserializationError> where Self: Sized;
}
