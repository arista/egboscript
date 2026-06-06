use std::{collections::HashMap, marker::PhantomData};

pub enum ModelError {
    ItemNotFound(Id),
    TypeMismatch(&'static str)
}

pub type ModelResult<T> = Result<T, ModelError>;

pub struct Model {
    pub items: HashMap<Id, Item>
}

impl Model {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }

    pub fn get_item(&self, id: &Id) -> ModelResult<&Item> {
        self.items.get(id).ok_or(ModelError::ItemNotFound(*id))
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub struct Id {
    pub id: u32
}

pub struct Handle<'a, T> {
    pub model: &'a Model,
    pub id: Id,

    // To allow the <T> even if it hasn't been used yet in a field
    _marker: PhantomData<T>,
}

pub enum Item {
    IntLiteral(IntLiteral),
    BooleanLiteral(BooleanLiteral),
    StringLiteral(StringLiteral),
}

impl Item {
    pub fn int_literal(id: Id, value: u32) -> Self {
        Self::IntLiteral(IntLiteral {id, value})
    }
    
    pub fn as_int_literal(&self) -> ModelResult<&IntLiteral> {
        match self {
            Self::IntLiteral(r) => Ok(r),
            _ => Err(ModelError::TypeMismatch("Item is not an IntLiteral"))
        }
    }

    pub fn string_literal(id: Id, value: String) -> Self {
        Self::StringLiteral(StringLiteral {id, value})
    }
    
    pub fn as_string_literal(&self) -> ModelResult<&StringLiteral> {
        match self {
            Self::StringLiteral(r) => Ok(r),
            _ => Err(ModelError::TypeMismatch("Item is not a StringLiteral"))
        }
    }

    pub fn boolean_literal(id: Id, value: bool) -> Self {
        Self::BooleanLiteral(BooleanLiteral {id, value})
    }
    
    pub fn as_boolean_literal(&self) -> ModelResult<&BooleanLiteral> {
        match self {
            Self::BooleanLiteral(r) => Ok(r),
            _ => Err(ModelError::TypeMismatch("Item is not a BooleanLiteral"))
        }
    }
}

//----------------------------------------
// IntLiteral

pub struct IntLiteral {
    pub id: Id,
    pub value: u32,
}

impl<'a> Handle<'a, IntLiteral> {
    pub fn item(&self) -> ModelResult<&IntLiteral> {
        self.model.get_item(&self.id)?.as_int_literal()
    }

    pub fn value(&self) -> ModelResult<u32> {
        Ok(self.item()?.value)
    }
}


//----------------------------------------
// BooleanLiteral

pub struct BooleanLiteral {
    pub id: Id,
    pub value: bool,
}

impl<'a> Handle<'a, BooleanLiteral> {
    pub fn item(&self) -> ModelResult<&BooleanLiteral> {
        self.model.get_item(&self.id)?.as_boolean_literal()
    }

    pub fn value(&self) -> ModelResult<bool> {
        Ok(self.item()?.value)
    }
}


//----------------------------------------
// StringLiteral

pub struct StringLiteral {
    pub id: Id,
    pub value: String,
}

impl<'a> Handle<'a, StringLiteral> {
    pub fn item(&self) -> ModelResult<&StringLiteral> {
        self.model.get_item(&self.id)?.as_string_literal()
    }

    pub fn value(&self) -> ModelResult<&String> {
        Ok(&self.item()?.value)
    }
}
