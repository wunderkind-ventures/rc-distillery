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