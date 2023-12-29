#[derive(Debug, PartialEq)]
pub struct RotationalGraph<T> {
    node_ptr: Option<usize>,
    size: usize,
    list: Vec<GraphNode<T>>,
}

impl<T> RotationalGraph<T> {
    pub fn new() -> RotationalGraph<T> {
        return RotationalGraph {
            node_ptr: None,
            size: 0,
            list: Vec::new(),
        };
    }

    pub fn add(&mut self, node: GraphNode<T>) -> &Self {
        self.list.push(node);
        if self.node_ptr.is_none() {
            self.node_ptr = Some(0);
        }
        return self;
    }

    pub fn remove(&mut self) -> &Self {
        match self.node_ptr {
            Some(value) => {
                self.list.remove(value);
                if self.list.len() == 0 {
                    self.node_ptr = None;
                }
            }
            None => {}
        }

        return self;
    }

    pub fn get_current(&self) -> Option<&GraphNode<T>> {
        match self.node_ptr {
            Some(value) => {
                let node = &self.list[value];
                return Some(node);
            }
            None => return None,
        }
    }

    pub fn rotate_current(&mut self) -> &Self {
        match self.node_ptr {
            Some(ptr) => {
                if (ptr + 1) == self.size {
                    self.node_ptr = Some(0)
                } else {
                    self.node_ptr = Some(ptr + 1);
                }
            }
            None => {}
        }
        return self;
    }
}

#[test]
fn test_rotational_graph_constructor() {
    let expected: RotationalGraph<i32> = RotationalGraph {
        node_ptr: None,
        size: 0,
        list: Vec::new(),
    };

    let result: RotationalGraph<i32> = RotationalGraph::new();

    assert_eq!(result, expected);
}

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
