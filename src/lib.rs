// See https://github.com/banach-space/llvm-tutor/blob/main/HelloWorld/HelloWorld.cpp
// for a more detailed explanation.

use regex::Regex;
use std::path::Path;

use llvm_plugin::inkwell::module::Module;
use llvm_plugin::inkwell::values::{AnyValue, CallSiteValue};
use llvm_plugin::{
    LlvmModulePass, ModuleAnalysisManager, PassBuilder, PipelineParsing, PreservedAnalyses,
};

use petgraph::dot::{Config, Dot};
use petgraph::Graph;
use types::{Function, FunctionBuilder};

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
        // Regex: ^ -> Start of the line
        // [^(]* -> All the chars that are not opening parenthesis
        // % -> Percent sign (Denoting a register in IR)
        // \d+ -> A number (name of the register. Auto assigned by LLVM)
        // \( -> The opening parenthesis (Denoting a function signature)
        let direct_call_pattern = Regex::new(r"^[^(]*%\d+\(").unwrap();

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
                        eprintln!("1){:?}", call_site_value.as_any_value_enum());
                        let inst_str = instruction.print_to_string().to_string();
                        // eprint!(
                        //         "2){:?}",
                        //         inst_str
                        //     );

                        if !direct_call_pattern.is_match(&inst_str) {
                            eprint!("{:?}", inst_str);
                            let called_fn = call_site_value.get_called_fn_value();
                            if let Ok(fn_name) = called_fn.get_name().to_str() {
                                // Skip LLVM intrinsics
                                if fn_name.starts_with("llvm.") {
                                    eprintln!("Skipping LLVM intrinsic: {}", fn_name);
                                    continue;
                                }

                                // Add to our call graph
                                let callee = types::get_index_or_insert_node(
                                    &mut omega_tree,
                                    FunctionBuilder::new(called_fn.get_name().to_str().expect(""))
                                        .build(),
                                );
                                omega_tree.add_edge(current_function, callee, ());
                            }
                        }
                    }
                }
            }
        }
        // eprintln!(
        //     "{:?}",
        //     Dot::with_config(&omega_tree, &[Config::EdgeNoLabel])
        // );
        types::append_graph(&mut omega_tree, &mut module_graph)
            .expect("Failed to append graph :shrug:");

        types::graph_to_json(Path::new(json_path), &omega_tree)
            .expect("Could not Serialize `omega_tree` to file");

        PreservedAnalyses::All
    }
}
