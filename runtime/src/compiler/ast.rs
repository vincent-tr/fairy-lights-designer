use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Program {
    pub variables: Vec<String>,
    pub body: Node,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Node {
  Sequence(SequenceNode),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequenceNode {
    pub items: Vec<Node>,
}