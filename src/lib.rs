// See https://github.com/banach-space/llvm-tutor/blob/main/HelloWorld/HelloWorld.cpp
// for a more detailed explanation.

use std::path::Path;

use llvm_plugin::inkwell::module::Module;
use llvm_plugin::inkwell::values::CallSiteValue;
use llvm_plugin::{
    LlvmModulePass, ModuleAnalysisManager, PassBuilder, PipelineParsing, PreservedAnalyses,
};

use petgraph::dot::{Config, Dot};
use petgraph::Graph;

pub mod types;

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

struct CustomPass;
impl LlvmModulePass for CustomPass {
    fn run_pass(&self, module: &mut Module, _manager: &ModuleAnalysisManager) -> PreservedAnalyses {
        let mut omega_tree = Graph::<types::Function, ()>::new();

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

            let current_function = types::get_index_or_insert_node(&mut omega_tree, function.into());

            for basic_block in function.get_basic_blocks() {
                for instruction in basic_block.get_instructions() {
                    if let Ok(call_site_value) = CallSiteValue::try_from(instruction) {
                        let callee = types::get_index_or_insert_node(
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

        types::graph_to_json(Path::new("omega_tree.json"), &omega_tree)
            .expect("Could not Serialize `omega_tree` to file");

        PreservedAnalyses::All
    }
}
