// See https://github.com/banach-space/llvm-tutor/blob/main/HelloWorld/HelloWorld.cpp
// for a more detailed explanation.

use std::path::Path;

use llvm_plugin::inkwell::module::Module;
use llvm_plugin::inkwell::values::{AnyValue, CallSiteValue};
use llvm_plugin::{
    LlvmModulePass, ModuleAnalysisManager, PassBuilder, PipelineParsing, PreservedAnalyses,
};

use petgraph::dot::{Config, Dot};
use petgraph::Graph;
use types::FunctionBuilder;

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
        let json_path = Path::new("omega_tree.json");

        // Load or create graph
        let mut omega_tree = if json_path.exists() {
            types::graph_from_json(json_path).expect("Failed to load existing graph")
        } else {
            Graph::<types::Function, ()>::new()
        };
        let mut module_graph = Graph::<types::Function, ()>::new();

        let module_name = module.get_name().to_str().expect("");
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
            let mut fb = FunctionBuilder::new(function.get_name().to_str().expect(""));

            if !function.is_null() {
                fb.source_file(module_name);
            }
            let current_function = types::get_index_or_insert_node(&mut omega_tree, fb.build());

            for basic_block in function.get_basic_blocks() {
                for instruction in basic_block.get_instructions() {
                    if let Ok(call_site_value) = CallSiteValue::try_from(instruction) {
                        
                        
                        let called_fn = match std::panic::catch_unwind(|| {
                            call_site_value.get_called_fn_value()
                        }) {
                            Ok(fn_val) => fn_val,
                            Err(_) => {
                                eprintln!("Warning: Failed to get called function value");
                                continue;
                            }
                        };
                        
                        // Notice how we handle the Result with if let Ok(...) instead of Some(...)
                        if let Ok(fn_name) = called_fn.get_name().to_str() {
                            // Skip LLVM intrinsics
                            if fn_name.starts_with("llvm.") {
                                eprintln!("Skipping LLVM intrinsic: {}", fn_name);
                                continue;
                            }
                            
                            // Add to our call graph
                            let callee = types::get_index_or_insert_node(
                                &mut omega_tree,
                                called_fn.into(),
                            );
                            omega_tree.add_edge(current_function, callee, ());
                            
                            eprintln!("Successfully added call to: {}", fn_name);
                        } else {
                            eprintln!("Warning: Function name contains invalid UTF-8");
                        }
                        
                    }
                }
            }
        }
        eprintln!(
            "{:?}",
            Dot::with_config(&omega_tree, &[Config::EdgeNoLabel])
        );
        types::append_graph(&mut omega_tree, &mut module_graph)
            .expect("Failed to append graph :shrug:");

        types::graph_to_json(Path::new(json_path), &omega_tree)
            .expect("Could not Serialize `omega_tree` to file");

        PreservedAnalyses::All
    }
}
