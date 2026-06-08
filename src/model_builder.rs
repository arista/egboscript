use crate::ast;
use crate::model;

pub fn ast_to_model(ast: &ast::File) -> model::Model {
    let mut model = model::Model::new();
    let mut mb = ModelBuilder::new(&mut model);
    let _ = mb.build_file(ast);
    model
}

struct ModelBuilder<'a> {
    model: &'a mut model::Model
}

impl<'a> ModelBuilder<'a> {
    pub fn new(model: &'a mut model::Model) -> Self {
        Self {
            model,
        }
    }

    pub fn build_file(&mut self, ast: &ast::File) -> model::ItemPtr<model::File> {
        self.model.add_file(model::File {
        })
    }
}
