pub struct Config {
    pub simulation_steps: usize,
    pub optimization_method: OptimizationMethod,
    pub visualization_format: VisualizationFormat,
}

pub enum OptimizationMethod {
    GradientDescent,
    SimulatedAnnealing,
    GeneticAlgorithm,
}

pub enum VisualizationFormat {
    PNG,
    SVG,
    Interactive,
}
