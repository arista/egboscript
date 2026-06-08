use crate::ast;
use crate::model;
use crate::peg_parser::Parsed;

struct ModelBuilder<'a> {
    model: &'a mut model::Model,
    source_file: model::SourceFilePtr,
}

impl<'a> ModelBuilder<'a> {
    pub fn new(model: &'a mut model::Model, source_file: model::SourceFilePtr) -> Self {
        Self {
            model,
            source_file,
        }
    }

    fn build_item<T, M, F, MIF>(&mut self, item: &Parsed<T>, model_items_f: MIF, f: F) -> model::ItemPtr<M>
    where
        F: Fn(&T, &mut Self) -> M,
        MIF: Fn(&mut model::Model) -> &mut model::ModelItems<M>,
    {
        let m = f(&item.value, self);
        let model_items = model_items_f(self.model);
        let ptr = model_items.add(m);
        let source_location = model::SourceLocation {
            source_file: self.source_file,
            span: model:: Span {
                start: item.range.start,
                end: item.range.end,
            },
        };
        self.model.span_table.insert(ptr.key(), source_location);
        ptr
    }
    
    pub fn build_file(&mut self, src: &Parsed<ast::File>) -> model::ItemPtr<model::File> {
        self.build_item(src, |m| &mut m.files, |v, mb| {
            model::File {
                items: v.items.iter().map(|v| mb.build_file_item(v)).collect(),
            }
        })
    }
    
    pub fn build_file_item(&mut self, src: &Parsed<ast::FileItem>) -> model::ItemPtr<model::FileItem> {
        self.build_item(src, |m| &mut m.file_items, |v, mb| {
            model::FileItem {
            }
        })
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
