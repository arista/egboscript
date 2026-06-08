use crate::ast;
use crate::model;
use crate::peg_parser::Parsed;

pub fn ast_to_model(ast: &Parsed<ast::File>) -> model::Model {
    let mut model = model::Model::new();
    let mut mb = ModelBuilder::new(&mut model);
//    let _ = mb.build_file(ast);
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

    // pub fn build_file(&mut self, ast: &Parsed<ast::File>) -> model::ItemPtr<model::File> {
    //     self.model.files.add(model::File {
    //         items: ast.value.items.iter().map(|v| self.build_file_item(v)),
    //     })
    // }

    // pub fn build_file_item(&mut self, ast: &Parsed<ast::FileItem>) -> model::ItemPtr<model::FileItem> {
    //     match ast {
    //         ast::FileItem::ImportDecl(v) => self.model.file_items.add(model::FileItem::ImportDecl(self.build_import_decl(v)))
    //     }
    // }

    // pub fn build_import_decl(&mut self, ast: &Parsed<ast::ImportDecl>) -> model::ItemPtr<model::ImportDecl> {
    //     self.model.import_decls.add(model::ImportDecl {
    //     })
    // }
}
