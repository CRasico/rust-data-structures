#[derive(Debug, PartialEq)]
pub struct GraphNode<T> {
    data: T,
    nodes: Vec<GraphNode<T>>,
}

impl<T> GraphNode<T> {
    pub fn new(data: T) -> GraphNode<T> {
        return GraphNode {
            data,
            nodes: Vec::new(),
        };
    }

    pub fn push(&mut self, node: GraphNode<T>) -> &Self {
        self.nodes.push(node);
        return self;
    }

    pub fn pop(&mut self) -> &Self {
        self.nodes.pop();
        return self;
    }
}

#[test]
fn test_graph_node_constructor() {
    let expected: GraphNode<i32> = GraphNode {
        data: 1,
        nodes: Vec::new(),
    };

    let result = GraphNode::new(1);

    assert_eq!(expected, result);
}

#[test]
fn test_graph_node_push() {
    let expected: GraphNode<i32> = GraphNode {
        data: 1,
        nodes: vec![GraphNode {
            data: 2,
            nodes: Vec::new(),
        }],
    };

    let mut result: GraphNode<i32> = GraphNode {
        data: 1,
        nodes: Vec::new(),
    };

    result.push(GraphNode {
        data: 2,
        nodes: Vec::new(),
    });

    assert_eq!(expected, result)
}
