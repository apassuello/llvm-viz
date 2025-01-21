// See https://github.com/banach-space/llvm-tutor/blob/main/HelloWorld/HelloWorld.cpp
// for a more detailed explanation.

use llvm_plugin::inkwell::values::{CallSiteValue, FunctionValue};
use llvm_plugin::{
    FunctionAnalysisManager, LlvmFunctionPass, PassBuilder, PipelineParsing, PreservedAnalyses,
};

#[llvm_plugin::plugin(name = "HelloWorld", version = "0.1")]
fn plugin_registrar(builder: &mut PassBuilder) {
    builder.add_function_pipeline_parsing_callback(|name, manager| {
        if name == "hello-world" {
            manager.add_pass(HelloWorldPass);
            PipelineParsing::Parsed
        } else {
            PipelineParsing::NotParsed
        }
    });
}

struct HelloWorldPass;
impl LlvmFunctionPass for HelloWorldPass {
    fn run_pass(
        &self,
        function: &mut FunctionValue,
        _manager: &FunctionAnalysisManager,
    ) -> PreservedAnalyses {
        eprintln!("(llvm-tutor) -------------");
        eprintln!("(llvm-tutor) Hello from: {:?}", function.get_name());
        eprintln!(
            "(llvm-tutor)   number of arguments: {}",
            function.count_params()
        );

        for basic_block in function.get_basic_blocks() {
            for instruction in basic_block.get_instructions() {
                if let Ok(csv) = CallSiteValue::try_from(instruction) {
                    eprintln!(
                        "(llvm-tutor)   callees: {:?}",
                        csv.get_called_fn_value().get_name()
                    );
                }
            }
        }
        PreservedAnalyses::All
    }
}
