pub struct State<T> {
    data: T,
}

pub struct Transition<T> {
    from: State<T>,
    to: State<T>,
    probability: f64,
}

pub struct Model<T> {
    states: Vec<State<T>>,
    transitions: Vec<Transition<T>>,
}
