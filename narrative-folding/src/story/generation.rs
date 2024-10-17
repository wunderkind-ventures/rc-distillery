use std::collections::HashMap;
use log::{debug, info, warn, error};

#[derive(Debug)]
enum StoryGenerationError {
    NoValidTransitions,
    MaxLengthExceeded,
    ConstraintViolation,
    // Add more error types as needed
}

fn generate_complex_story(
    graph: &NarrativeGraph,
    constraint_manager: &ConstraintManager,
    start_node: usize,
    max_length: usize,
) -> Result<Vec<StoryNode>, StoryGenerationError> {
    let mut story = Vec::new();
    let mut state = NarrativeState {
        current_node: start_node,
        story_variables: HashMap::new(),
    };

    debug!("Starting story generation from node {}", start_node);

    for _ in 0..max_length {
        let current_node = &graph.nodes[state.current_node];
        debug!("Current node: {:?}", current_node);
        story.push(current_node.clone());

        // Handle branching and nested narratives
        let valid_edges: Vec<&StoryEdge> = graph.edges.iter()
            .filter(|e| e.from == state.current_node)
            .filter(|e| {
                let mut next_state = state.clone();
                next_state.current_node = e.to;
                constraint_manager.check_all(&next_state, graph)
            })
            .collect();

        debug!("Valid edges: {:?}", valid_edges);

        if valid_edges.is_empty() {
            warn!("No valid edges found, ending story generation.");
            return Err(StoryGenerationError::NoValidTransitions);
        }

        // Select next edge based on scoring
        let next_edge = valid_edges.into_iter()
            .max_by(|a, b| {
                let mut state_a = state.clone();
                state_a.current_node = a.to;
                let mut state_b = state.clone();
                state_b.current_node = b.to;
                constraint_manager.get_score(&state_a, graph)
                    .partial_cmp(&constraint_manager.get_score(&state_b, graph))
                    .unwrap()
            })
            .ok_or(StoryGenerationError::ConstraintViolation)?;

        debug!("Next edge chosen: {:?}", next_edge);
        state.current_node = next_edge.to;

        // Update state variables based on the new node
        update_state_variables(&mut state, &graph.nodes[state.current_node]);
    }

    if story.len() >= max_length {
        warn!("Max length reached, story may be incomplete.");
        return Err(StoryGenerationError::MaxLengthExceeded);
    }

    info!("Story generation completed successfully.");
    Ok(story)
}

fn update_state_variables(state: &mut NarrativeState, node: &StoryNode) {
    // Update story variables based on the new node's metadata
    for (key, value) in &node.metadata {
        if key.starts_with("var_") {
            state.story_variables.insert(key[4..].to_string(), value.clone());
        }
    }

    // Handle special variables
    if let Some(emotion) = node.metadata.get("emotion") {
        let mut emotion_history = state.story_variables
            .get("emotion_history")
            .map(|s| s.split(',').collect::<Vec<_>>())
            .unwrap_or_default();
        emotion_history.push(emotion);
        if emotion_history.len() > 5 {
            emotion_history.remove(0);
        }
        state.story_variables.insert("emotion_history".to_string(), emotion_history.join(","));
    }

    // Update chapter events
    if node.metadata.get("is_chapter_start").map(|v| v == "true").unwrap_or(false) {
        state.story_variables.insert("current_chapter_events".to_string(), "0".to_string());
    } else {
        let current_events = state.story_variables
            .get("current_chapter_events")
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(0);
        state.story_variables.insert("current_chapter_events".to_string(), (current_events + 1).to_string());
    }

    debug!("Updated state variables: {:?}", state.story_variables);
}

// Story generation
pub fn generate_story(
    graph: &NarrativeGraph,
    constraint_manager: &ConstraintManager,
    start_node: usize,
    max_length: usize,
) -> Result<Vec<StoryNode>, String> {
    let mut story = Vec::new();
    let mut state = NarrativeState {
        current_node: start_node,
        story_variables: HashMap::new(),
    };

    for _ in 0..max_length {
        let current_node = &graph.nodes[state.current_node];
        story.push(current_node.clone());

        let valid_edges: Vec<&StoryEdge> = graph.edges.iter()
            .filter(|e| e.from == state.current_node)
            .filter(|e| {
                let mut next_state = state.clone();
                next_state.current_node = e.to;
                constraint_manager.check_all(&next_state, graph)
            })
            .collect();

        if valid_edges.is_empty() {
            break;
        }

        let next_edge = valid_edges.into_iter()
            .max_by(|a, b| {
                let mut state_a = state.clone();
                state_a.current_node = a.to;
                let mut state_b = state.clone();
                state_b.current_node = b.to;
                constraint_manager.get_score(&state_a, graph)
                    .partial_cmp(&constraint_manager.get_score(&state_b, graph))
                    .unwrap()
            })
            .unwrap();

        state.current_node = next_edge.to;
        // Update state variables based on the new node
        // ...
    }

    Ok(story)
}

fn main() {
    // Set up logging
    env_logger::init();

    // Create graph, constraint manager, etc.
    let graph = NarrativeGraph { /* ... */ };
    let constraint_manager = ConstraintManager { /* ... */ };

    match generate_complex_story(&graph, &constraint_manager, 0, 100) {
        Ok(story) => {
            println!("Generated story:");
            for node in story {
                println!("- {}", node.content);
            }
        },
        Err(e) => error!("Failed to generate story: {:?}", e),
    }
}