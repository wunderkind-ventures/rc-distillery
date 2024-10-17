
// Main function to demonstrate usage
fn main() {
    let mut graph = NarrativeGraph {
        nodes: vec![
            StoryNode { id: 0, content: "Start".to_string(), metadata: HashMap::new() },
            StoryNode { id: 1, content: "Middle".to_string(), metadata: HashMap::new() },
            StoryNode { id: 2, content: "End".to_string(), metadata: HashMap::new() },
        ],
        edges: vec![
            StoryEdge { from: 0, to: 1, weight: 1.0 },
            StoryEdge { from: 1, to: 2, weight: 1.0 },
        ],
    };

    let mut constraint_manager = ConstraintManager::new();
    constraint_manager.add_constraint(Box::new(ThematicConsistencyConstraint {
        themes: vec!["adventure".to_string()],
        weight: 1.0,
    }));
    constraint_manager.add_constraint(Box::new(CharacterArcConstraint {
        character: "hero".to_string(),
        arc_stages: vec!["call".to_string(), "journey".to_string(), "return".to_string()],
        weight: 1.0,
    }));

    match generate_story(&graph, &constraint_manager, 0, 3) {
        Ok(story) => {
            println!("Generated story:");
            for node in story {
                println!("- {}", node.content);
            }
        },
        Err(e) => println!("Error generating story: {}", e),
    }

    println!("\nNarrative Graph Visualization:");
    println!("{}", visualize_narrative_graph(&graph));
}