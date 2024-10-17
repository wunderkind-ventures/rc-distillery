/*
This implementation provides more detailed logic for each constraint and adds two new constraint types:
PacingConstraint: Ensures that each chapter has an appropriate number of events, maintaining a good story pace.
EmotionalVarietyConstraint: Checks that the story includes a variety of emotions within a given window of recent events.
The existing constraints have been enhanced with more detailed logic:
ThematicConsistencyConstraint now checks for the presence of any defined theme in the current node's metadata.
CharacterArcConstraint ensures that the character has gone through all the required previous stages of their arc.
ConflictProgressionConstraint checks that the conflict level is increasing over time.
AudienceAdaptationConstraint verifies that the current node's content is suitable for the target audience.
To use these constraints, you would create instances of each constraint and add them to the ConstraintManager. 

For example:
```rust
let mut constraint_manager = ConstraintManager::new();

constraint_manager.add_constraint(Box::new(ThematicConsistencyConstraint {
    themes: vec!["adventure".to_string(), "mystery".to_string()],
    weight: 1.0,
}));

constraint_manager.add_constraint(Box::new(CharacterArcConstraint {
    character: "protagonist".to_string(),
    arc_stages: vec!["ordinary_world".to_string(), "call_to_adventure".to_string(), "refusal".to_string(), "acceptance".to_string()],
    weight: 1.5,
}));

constraint_manager.add_constraint(Box::new(ConflictProgressionConstraint {
    weight: 1.2,
}));

constraint_manager.add_constraint(Box::new(AudienceAdaptationConstraint {
    target_audience: "young_adult".to_string(),
    weight: 1.0,
}));

constraint_manager.add_constraint(Box::new(PacingConstraint {
    min_events_per_chapter: 3,
    max_events_per_chapter: 7,
    weight: 0.8,
}));

constraint_manager.add_constraint(Box::new(EmotionalVarietyConstraint {
    required_emotions: ["joy", "fear", "anger", "sadness"].iter().map(|&s| s.to_string()).collect(),
    window_size: 5,
    weight: 1.1,
}));
*/

// Constraint trait
trait Constraint {
    fn check(&self, state: &NarrativeState, graph: &NarrativeGraph) -> bool;
    fn get_weight(&self) -> f64;
}

// Thematic Consistency Constraint
struct ThematicConsistencyConstraint {
    themes: Vec<String>,
    weight: f64,
}

impl Constraint for ThematicConsistencyConstraint {
    fn check(&self, state: &NarrativeState, graph: &NarrativeGraph) -> bool {
        let current_node = &graph.nodes[state.current_node];
        let present_themes: HashSet<String> = current_node.metadata.get("themes")
            .map(|t| t.split(',').map(String::from).collect())
            .unwrap_or_default();
        
        self.themes.iter().any(|theme| present_themes.contains(theme))
    }

    fn get_weight(&self) -> f64 {
        self.weight
    }
}

// Character Arc Constraint
struct CharacterArcConstraint {
    character: String,
    arc_stages: Vec<String>,
    weight: f64,
}

impl Constraint for CharacterArcConstraint {
    fn check(&self, state: &NarrativeState, graph: &NarrativeGraph) -> bool {
        let current_character_state = state.story_variables.get(&format!("{}_state", self.character));
        let visited_states: HashSet<String> = state.story_variables.get(&format!("{}_visited_states", self.character))
            .map(|s| s.split(',').map(String::from).collect())
            .unwrap_or_default();

        if let Some(current_state) = current_character_state {
            if let Some(arc_index) = self.arc_stages.iter().position(|s| s == current_state) {
                let required_states: HashSet<_> = self.arc_stages[..=arc_index].iter().cloned().collect();
                required_states.is_subset(&visited_states)
            } else {
                false
            }
        } else {
            false
        }
    }

    fn get_weight(&self) -> f64 {
        self.weight
    }
}

// Conflict Progression Constraint
struct ConflictProgressionConstraint {
    weight: f64,
}

impl Constraint for ConflictProgressionConstraint {
    fn check(&self, state: &NarrativeState, graph: &NarrativeGraph) -> bool {
        let current_node = &graph.nodes[state.current_node];
        if let Some(conflict_level) = current_node.metadata.get("conflict_level") {
            let current_conflict_level: i32 = conflict_level.parse().unwrap_or(0);
            let previous_conflict_level: i32 = state.story_variables.get("conflict_level")
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);
            current_conflict_level > previous_conflict_level
        } else {
            true
        }
    }

    fn get_weight(&self) -> f64 {
        self.weight
    }
}

// Audience Adaptation Constraint
struct AudienceAdaptationConstraint {
    target_audience: String,
    weight: f64,
}

impl Constraint for AudienceAdaptationConstraint {
    fn check(&self, state: &NarrativeState, graph: &NarrativeGraph) -> bool {
        let current_node = &graph.nodes[state.current_node];
        current_node.metadata.get("audience")
            .map(|audience| audience.contains(&self.target_audience))
            .unwrap_or(false)
    }

    fn get_weight(&self) -> f64 {
        self.weight
    }
}

// New constraint: Pacing Constraint
struct PacingConstraint {
    min_events_per_chapter: usize,
    max_events_per_chapter: usize,
    weight: f64,
}

impl Constraint for PacingConstraint {
    fn check(&self, state: &NarrativeState, graph: &NarrativeGraph) -> bool {
        let current_chapter_events = state.story_variables.get("current_chapter_events")
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(0);
        
        current_chapter_events >= self.min_events_per_chapter && 
        current_chapter_events <= self.max_events_per_chapter
    }

    fn get_weight(&self) -> f64 {
        self.weight
    }
}

// New constraint: Emotional Variety Constraint
struct EmotionalVarietyConstraint {
    required_emotions: HashSet<String>,
    window_size: usize,
    weight: f64,
}

impl Constraint for EmotionalVarietyConstraint {
    fn check(&self, state: &NarrativeState, graph: &NarrativeGraph) -> bool {
        let emotion_history: Vec<String> = state.story_variables.get("emotion_history")
            .map(|s| s.split(',').map(String::from).collect())
            .unwrap_or_default();
        
        let recent_emotions: HashSet<_> = emotion_history.iter().rev().take(self.window_size).cloned().collect();
        !self.required_emotions.is_disjoint(&recent_emotions)
    }

    fn get_weight(&self) -> f64 {
        self.weight
    }
}
