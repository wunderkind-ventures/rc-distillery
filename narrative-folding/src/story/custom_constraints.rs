use crate::story::constraints::Constraint;
use crate::story::state::NarrativeState;
use crate::story::graph::NarrativeGraph;

/*
Custom constraints are defined in the `custom_constraints.rs` file. 

The `CustomConstraint` struct is used to define a custom constraint. It contains the following fields:

- `name`: A string representing the name of the constraint.
- `constraint_type`: An enum representing the type of constraint.
- `elements`: A vector of strings representing the elements of the constraint.
- `condition`: A string representing the condition of the constraint.

let hero_journey = Constraint::new(
    "Hero's Journey Compliance",
    ConstraintType::Sequence,
    vec!["Ordinary World", "Call to Adventure", "Refusal of the Call", "Meeting the Mentor", "Crossing the Threshold"],
    "All elements must be present in order"
);

let multiple_act2s = Constraint::new(
    "Multiple Act 2s",
    ConstraintType::Quantity,
    vec!["Act 2"],
    "At least 2 occurrences"
);

let character_growth = Constraint::new(
    "Character Growth Arc",
    ConstraintType::Presence,
    vec!["Character Flaw", "Challenge", "Growth Moment", "Resolution"],
    "All elements must be present"
);
*/



enum ConstraintType {
    Sequence,
    Quantity,
    Presence,
    Exclusion,
    Custom(String),
}

struct CustomConstraint {
    name: String,
    constraint_type: ConstraintType,
    elements: Vec<String>,
    condition: String,
}

impl CustomConstraint {
    fn new(name: &str, constraint_type: ConstraintType, elements: Vec<&str>, condition: &str) -> Self {
        CustomConstraint {
            name: name.to_string(),
            constraint_type,
            elements: elements.iter().map(|&s| s.to_string()).collect(),
            condition: condition.to_string(),
        }
    }
}

impl Constraint for CustomConstraint {
    fn check(&self, state: &NarrativeState, graph: &NarrativeGraph) -> bool {
        match self.constraint_type {
            ConstraintType::Sequence => {
                // Check if elements are in sequence
            }
            ConstraintType::Quantity => {
                // Check if elements are in quantity
            }
            ConstraintType::Presence => {
                // Check if elements are present
            }
            ConstraintType::Exclusion => {
                // Check if elements are excluded
            }
            ConstraintType::Custom(ref condition) => {
                // Check custom condition
                eval(condition, state, graph)
            }
        }
    }

    fn get_weight(&self) -> f64 {
        // Implement weight retrieval based on constraint type
    }
}

// ConstraintManager to handle multiple constraints
struct ConstraintManager {
    constraints: Vec<Box<dyn Constraint>>,
}

impl ConstraintManager {
    fn new() -> Self {
        ConstraintManager {
            constraints: Vec::new(),
        }
    }

    fn add_constraint(&mut self, constraint: Box<dyn Constraint>) {
        self.constraints.push(constraint);
    }

    fn check_all(&self, state: &NarrativeState, graph: &NarrativeGraph) -> bool {
        self.constraints.iter().all(|c| c.check(state, graph))
    }

    fn get_score(&self, state: &NarrativeState, graph: &NarrativeGraph) -> f64 {
        self.constraints.iter().map(|c| {
            if c.check(state, graph) { c.get_weight() } else { 0.0 }
        }).sum()
    }
}
