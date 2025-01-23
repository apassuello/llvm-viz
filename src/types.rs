use llvm_plugin::inkwell::values;
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use serde::{Deserialize, Serialize};

use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Function {
    name: String,
}

impl From<values::FunctionValue<'_>> for Function {
    fn from(item: values::FunctionValue) -> Self {
        Self {
            name: item.get_name().to_str().expect("").into(),
        }
    }
}

pub fn get_index_or_insert<E>(graph: &mut Graph<Function, E>, node: Function) -> NodeIndex {
    graph
        .node_indices()
        .find(|ix| graph[*ix].name == node.name)
        .unwrap_or_else(|| graph.add_node(node))
}

pub fn graph_from_json(path: &Path) -> Result<Graph<Function, ()>, Box<dyn Error>> {
    let mut file = File::open(path)?;
    let mut file_content = String::new();
    file.read_to_string(&mut file_content)?;
    Ok(serde_json::from_str(&file_content.clone())?)
}

pub fn graph_to_json(path: &Path, data: &Graph<Function, ()>) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(path)?;
    Ok(file.write_all(serde_json::to_string_pretty(data)?.as_bytes())?)
}
