fn identify_sink_states(graph: &NarrativeGraph) -> Vec<usize> {
    let mut sink_states = Vec::new();
    for node in &graph.nodes {
        let has_outgoing_edges = graph.edges.iter().any(|edge| edge.from == node.id);
        if !has_outgoing_edges {
            sink_states.push(node.id);
        }
    }
    sink_states
}