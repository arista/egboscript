use crate::ast;
use crate::model;
use crate::peg_parser::{ParsedRange};

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

    fn build_item<T, M, F, MIF>(&mut self, item: &T, range: &ParsedRange, model_items_f: MIF, f: F) -> model::ItemPtr<M>
    where
        F: Fn(&T, &mut Self) -> M,
        MIF: Fn(&mut model::Model) -> &mut model::ModelItems<M>,
    {
        let m = f(item, self);
        let model_items = model_items_f(self.model);
        let ptr = model_items.add(m);
        let source_location = model::SourceLocation {
            source_file: self.source_file,
            span: model:: Span {
                start: range.start,
                end: range.end,
            },
        };
        self.model.span_table.insert(ptr.key(), source_location);
        ptr
    }
    
    pub fn build_file(&mut self, src: &ast::File, range: &ParsedRange) -> model::ItemPtr<model::File> {
        self.build_item(src, range, |m| &mut m.files, |v, mb| {
            model::File {
                items: v.items.iter().map(|v| mb.build_file_item(&v.value, &v.range)).collect(),
            }
        })
    }
    
    pub fn build_file_item(&mut self, src: &ast::FileItem, range: &ParsedRange) -> model::ItemPtr<model::FileItem> {
        self.build_item(src, range, |m| &mut m.file_items, |v, mb| {
            match v {
                ast::FileItem::Statement(a) => model::FileItem::Statement(mb.build_statement(a, range)),
                ast::FileItem::TypeDecl(a) => model::FileItem::TypeDecl(mb.build_type_decl(a, range)),
                ast::FileItem::ImportDecl(a) => model::FileItem::ImportDecl(mb.build_import_decl(a, range)),
            }
        })
    }
    
    pub fn build_statement(&mut self, src: &ast::Statement, range: &ParsedRange) -> model::ItemPtr<model::Statement> {
        self.build_item(src, range, |m| &mut m.statements, |v, mb| {
            model::Statement {
            }
        })
    }
    
    pub fn build_type_decl(&mut self, src: &ast::TypeDecl, range: &ParsedRange) -> model::ItemPtr<model::TypeDecl> {
        self.build_item(src, range, |m| &mut m.type_decls, |v, mb| {
            model::TypeDecl {
            }
        })
    }
    
    pub fn build_import_decl(&mut self, src: &ast::ImportDecl, range: &ParsedRange) -> model::ItemPtr<model::ImportDecl> {
        self.build_item(src, range, |m| &mut m.import_decls, |v, mb| {
            model::ImportDecl {
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
