class NarrativeState:
    def __init__(self, current_node: int, story_variables: Optional[Dict[str, Any]] = None):
        self.current_node = current_node
        self.story_variables = story_variables if story_variables is not None else {}

    def transition(self, graph: 'NarrativeGraph', edge: 'StoryEdge') -> Optional[str]:
        if edge.from_node == self.current_node:
            self.current_node = edge.to_node
            # Update story variables based on the new node
            new_node = graph.nodes[self.current_node]
            self.update_story_variables(new_node)
            return None  # Successful transition
        else:
            return "Invalid transition: current node does not match edge's from_node."

    def update_story_variables(self, node: 'StoryNode'):
        # Update visited nodes
        if 'visited_nodes' not in self.story_variables:
            self.story_variables['visited_nodes'] = set()
        self.story_variables['visited_nodes'].add(node.id)

        # Update character states
        for character, state in node.metadata.get('character_states', {}).items():
            self.story_variables[f"{character}_state"] = state
            if f"{character}_visited_states" not in self.story_variables:
                self.story_variables[f"{character}_visited_states"] = set()
            self.story_variables[f"{character}_visited_states"].add(state)

        # Update plot points
        if 'plot_point' in node.metadata:
            self.story_variables['current_plot_point'] = node.metadata['plot_point']
            if 'plot_points' not in self.story_variables:
                self.story_variables['plot_points'] = []
            self.story_variables['plot_points'].append(node.metadata['plot_point'])

        # Update any other custom variables defined in the node's metadata
        for key, value in node.metadata.get('custom_variables', {}).items():
            self.story_variables[key] = value