use llvm_plugin::inkwell::values;
use petgraph::graph::NodeIndex;
use petgraph::Graph;
use serde::{Deserialize, Serialize};

use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

pub fn get_index_or_insert<N, E>(graph: &mut Graph<N, E>, node: N) -> NodeIndex
where
    N: PartialEq,
{
    graph
        .node_indices()
        .find(|ix| graph[*ix] == node)
        .unwrap_or_else(|| graph.add_node(node))
}

pub fn append_graph<E>(
    source: &mut Graph<Function, E>,
    dest: &mut Graph<Function, E>,
) -> Result<(), Box<dyn Error>> {
    Ok(())
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

#[cfg(test)]
mod tests {
    use super::*;

    // Credit: https://github.com/petgraph/petgraph/issues/199#issuecomment-484077775
    fn graph_eq<N, E, Ty, Ix>(
        a: &petgraph::Graph<N, E, Ty, Ix>,
        b: &petgraph::Graph<N, E, Ty, Ix>,
    ) -> bool
    where
        N: PartialEq,
        E: PartialEq,
        Ty: petgraph::EdgeType,
        Ix: petgraph::graph::IndexType + PartialEq,
    {
        let a_ns = a.raw_nodes().iter().map(|n| &n.weight);
        let b_ns = b.raw_nodes().iter().map(|n| &n.weight);
        let a_es = a
            .raw_edges()
            .iter()
            .map(|e| (e.source(), e.target(), &e.weight));
        let b_es = b
            .raw_edges()
            .iter()
            .map(|e| (e.source(), e.target(), &e.weight));
        a_ns.eq(b_ns) && a_es.eq(b_es)
    }

    #[test]
    fn merge_two_nodes() {
        let mut g = Graph::<_, ()>::new();
        _ = g.add_node(Function {
            name: "fn1".to_owned(),
        });
        _ = g.add_node(Function {
            name: "fn2".to_owned(),
        });

        let mut g1 = Graph::<_, ()>::new();
        _ = g1.add_node(Function {
            name: "fn1".to_owned(),
        });
        let mut g2 = Graph::<_, ()>::new();
        _ = g2.add_node(Function {
            name: "fn2".to_owned(),
        });
        append_graph(&mut g1, &mut g2).unwrap();

        assert!(graph_eq(&g, &g1));
    }

    #[test]
    fn insert_new_node_twice() {
        let mut g = Graph::<_, ()>::new();
        let fn1_id = get_index_or_insert(
            &mut g,
            Function {
                name: "fn1".to_owned(),
            },
        );
        let fn1_id_bis = get_index_or_insert(
            &mut g,
            Function {
                name: "fn1".to_owned(),
            },
        );

        assert_eq!(fn1_id, fn1_id_bis);
    }

    #[test]
    fn insert_two_nodes() {
        let mut g = Graph::<_, ()>::new();
        let fn1_id = get_index_or_insert(
            &mut g,
            Function {
                name: "fn1".to_owned(),
            },
        );
        let fn2_id = get_index_or_insert(
            &mut g,
            Function {
                name: "fn2".to_owned(),
            },
        );

        assert_ne!(fn1_id, fn2_id);
    }
}
