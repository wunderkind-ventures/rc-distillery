use std::collections::HashMap;

pub struct StoryNode {
    pub id: usize,
    pub content: String,
    pub metadata: HashMap<String, String>,
}

pub struct StoryEdge {
    pub from: usize,
    pub to: usize,
    pub weight: f64,
}

pub struct NarrativeGraph {
    pub nodes: Vec<StoryNode>,
    pub edges: Vec<StoryEdge>,
}


impl NarrativeGraph {
    fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    fn add_node(&mut self, node: StoryNode) {
        self.nodes.push(node);
    }

    fn add_edge(&mut self, edge: StoryEdge) {
        self.edges.push(edge);
    }

    fn to_petgraph(&self) -> DiGraph<&str, f64> {
        let mut graph = DiGraph::new();
        let mut node_map = HashMap::new();

        for node in &self.nodes {
            let index = graph.add_node(node.content.as_str());
            node_map.insert(node.id, index);
        }

        for edge in &self.edges {
            if let (Some(&from), Some(&to)) = (node_map.get(&edge.from), node_map.get(&edge.to)) {
                graph.add_edge(from, to, edge.weight);
            }
        }

        graph
    }
}

pub struct NarrativeState {
    pub current_node: usize,
    pub story_variables: HashMap<String, String>,
}

impl NarrativeState {
    pub fn new(current_node: usize) -> Self {
        NarrativeState {
            current_node,
            story_variables: HashMap::new(),
        }
    }

    pub fn transition(&mut self, graph: &NarrativeGraph, edge: &StoryEdge) -> Result<(), String> {
        if edge.from == self.current_node {
            self.current_node = edge.to;
            self.update_story_variables(&graph.nodes[self.current_node]);
            Ok(())
        } else {
            Err("Invalid transition: current node does not match edge's from_node.".to_string())
        }
    }

    fn update_story_variables(&mut self, node: &StoryNode) {
        // Implementation similar to the Python version
        // (Simplified for brevity)
        self.story_variables.insert("visited_nodes".to_string(), node.id.to_string());
        // Add more variable updates based on node metadata
    }
}

impl NarrativeState {
    fn transition(&mut self, edge: &StoryEdge) -> Result<(), TransitionError> {
        // Implement state transition logic
    }
}

impl Clone for StoryNode {
    fn clone(&self) -> Self {
        StoryNode {
            id: self.id,
            content: self.content.clone(),
            metadata: self.metadata.clone(),
        }
    }
}

impl Clone for StoryEdge {
    fn clone(&self) -> Self {
        StoryEdge {
            from: self.from,
            to: self.to,
            weight: self.weight,
        }
    }
}


struct NarrativeApp {
    narrative_graph: NarrativeGraph,
}

impl Default for NarrativeApp {
    fn default() -> Self {
        let mut narrative_graph = NarrativeGraph::new();
        narrative_graph.add_node(StoryNode { id: 0, content: "Start".into(), metadata: HashMap::new() });
        narrative_graph.add_node(StoryNode { id: 1, content: "Conflict".into(), metadata: HashMap::new() });
        narrative_graph.add_node(StoryNode { id: 2, content: "Resolution".into(), metadata: HashMap::new() });
        narrative_graph.add_edge(StoryEdge { from: 0, to: 1, weight: 1.5 });
        narrative_graph.add_edge(StoryEdge { from: 1, to: 2, weight: 2.0 });

        Self { narrative_graph }
    }
}


impl epi::App for NarrativeApp {
    fn name(&self) -> &str {
        "Narrative Graph Visualizer"
    }

    fn update(&mut self, ctx: &egui::Context, _: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Narrative Graph Visualization");

            if ui.button("Add Node").clicked() {
                // Logic to add a node
            }

            if ui.button("Add Edge").clicked() {
                // Logic to add an edge
            }

            if ui.button("Save Graph").clicked() {
                // Logic to save the graph
            }

            if ui.button("Load Graph").clicked() {
                // Logic to load the graph
            }

            let graph = self.narrative_graph.to_petgraph();
            for edge in graph.edge_references() {
                ui.label(format!(
                    "{} -> {} (weight {})",
                    graph[edge.source()],
                    graph[edge.target()],
                    edge.weight()
                ));
            }
        });
    }
}

// fn main() -> Result<(), eframe::Error> {
//     let app = NarrativeApp::default();
//     let native_options = eframe::NativeOptions::default();
//     eframe::run_native(Box::new(app), native_options)
// }