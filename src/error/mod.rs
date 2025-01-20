use thiserror::Error;

#[derive(Error, Debug)]
pub enum GraphVizError {
    #[error("LLVM initialization error: {0}")]
    LLVMInit(String),
    
    #[error("Failed to parse LLVM module: {0}")]
    ModuleParsing(String),
    
    #[error("Graph construction error: {0}")]
    GraphConstruction(String),
    
    #[error("Visualization error: {0}")]
    Visualization(String),
    
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, GraphVizError>;