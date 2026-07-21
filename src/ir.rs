use crate::cpp;

impl cpp::ModuleTpde {
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
            basic_blocks: Vec::new(),
            instructions: vec![],
        }
    }
}