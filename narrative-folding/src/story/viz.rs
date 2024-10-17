use plotters::prelude::*;


fn visualize_narrative_graph(graph: &NarrativeGraph) -> String {
    let mut dot_representation = String::from("digraph NarrativeGraph {\n");
    for node in &graph.nodes {
        dot_representation.push_str(&format!("    {} [label=\"{}\"];\n", node.id, node.content));
    }
    for edge in &graph.edges {
        dot_representation.push_str(&format!("    {} -> {};\n", edge.from, edge.to));
    }
    dot_representation.push_str("}\n");
    dot_representation
}

fn plot_energy_landscape(states: &[NarrativeState], energies: &[f64]) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("energy_landscape.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Narrative Energy Landscape", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f32..states.len() as f32, *energies.iter().min_by(|a, b| a.partial_cmp(b).unwrap()).unwrap()..*energies.iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap())?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(
        states.iter().enumerate().map(|(i, _)| (i as f32, energies[i])),
        &RED,
    ))?;

    Ok(())
}

