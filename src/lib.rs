// See https://github.com/banach-space/llvm-tutor/blob/main/HelloWorld/HelloWorld.cpp
// for a more detailed explanation.

use llvm_plugin::inkwell::module::Module;
use llvm_plugin::inkwell::values;
use llvm_plugin::inkwell::values::CallSiteValue;
use llvm_plugin::{
    LlvmModulePass, ModuleAnalysisManager, PassBuilder, PipelineParsing, PreservedAnalyses,
};
use serde::Serialize;

use petgraph::dot::{Config, Dot};
use petgraph::graph::NodeIndex;
use petgraph::Graph;

#[llvm_plugin::plugin(name = "plugin_name", version = "0.1")]
fn plugin_registrar(builder: &mut PassBuilder) {
    builder.add_module_pipeline_parsing_callback(|name, manager| {
        if name == "hello-world" {
            manager.add_pass(CustomPass);
            PipelineParsing::Parsed
        } else {
            PipelineParsing::NotParsed
        }
    });
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
struct Function {
    name: String,
}

impl From<values::FunctionValue<'_>> for Function {
    fn from(item: values::FunctionValue) -> Self {
        Self {
            name: item.get_name().to_str().expect("").into(),
        }
    }
}

fn get_index_or_insert<E>(graph: &mut Graph<Function, E>, node: Function) -> NodeIndex {
    graph
        .node_indices()
        .find(|ix| graph[*ix].name == node.name)
        .unwrap_or_else(|| graph.add_node(node))
}

struct CustomPass;
impl LlvmModulePass for CustomPass {
    fn run_pass(&self, module: &mut Module, _manager: &ModuleAnalysisManager) -> PreservedAnalyses {
        let mut omega_tree = Graph::<Function, ()>::new();

        for function in module.get_functions() {
            /* Equivalent code in C++

               for (auto &bb : function) {
                   for (auto &instruction : bb) {
                       if (CallInst *callInst = dyn_cast<CallInst>(&instruction)) {
                           if (Function *calledFunction = callInst->getCalledFunction()) {
                               std::cerr << calledFunction->getName();
                           }
                       }
                   }
               }
            */

            let current_function = get_index_or_insert(&mut omega_tree, function.into());

            for basic_block in function.get_basic_blocks() {
                for instruction in basic_block.get_instructions() {
                    if let Ok(call_site_value) = CallSiteValue::try_from(instruction) {
                        let callee = get_index_or_insert(
                            &mut omega_tree,
                            call_site_value.get_called_fn_value().into(),
                        );
                        omega_tree.add_edge(current_function, callee, ());
                    }
                }
            }
        }
        eprintln!(
            "{:?}",
            Dot::with_config(&omega_tree, &[Config::EdgeNoLabel])
        );

        eprintln!("==========");

        eprintln!(
            "{:?}",
            serde_json::to_string_pretty(&omega_tree).expect("Could not Serialize the tree")
        );

        PreservedAnalyses::All
    }
}
