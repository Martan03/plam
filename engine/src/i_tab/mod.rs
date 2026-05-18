use std::{borrow::Cow, collections::HashMap};

mod id;
mod ident;

pub use self::{id::*, ident::*};

/// Table of identifiers.
#[derive(Debug)]
pub struct ITab {
    next_id: usize,
    idents: HashMap<Id, Ident>,
    unbound: HashMap<String, Id>,
    top: HashMap<String, Id>,
    scopes: Vec<HashMap<String, Id>>,
}

impl ITab {
    /// Create new empty table of identifiers.
    pub fn new() -> Self {
        Self {
            next_id: 0,
            unbound: HashMap::new(),
            idents: HashMap::new(),
            top: HashMap::new(),
            scopes: vec![],
        }
    }

    /// Get the name of the given identifier as it is shown in code or its
    /// unique if the name is not known.
    pub fn name_of(&self, id: Id) -> Cow<'_, str> {
        self.idents
            .get(&id)
            .map(|a| a.name().into())
            .unwrap_or_else(|| format!("<{}>", id.0).into())
    }

    /// Get identifer with the given name. If no such identifier exists, new
    /// unbound identifier is created and returned.
    pub fn get(&mut self, name: &str) -> Id {
        for s in self.scopes.iter().rev() {
            if let Some(id) = s.get(name) {
                return *id;
            }
        }
        if let Some(id) = self.top.get(name) {
            return *id;
        }
        let id = Id(self.next_id);
        self.next_id += 1;
        self.unbound.insert(name.to_string(), id);
        self.idents.insert(id, Ident::new(name.to_string()));
        id
    }

    /// Insert new identifier with the given name. This will cover any
    /// identifiers with the same scope and name.
    pub fn insert(&mut self, name: &str) -> Id {
        let id = Id(self.next_id);
        self.next_id += 1;
        let scope = self.scopes.last_mut().unwrap_or(&mut self.top);
        scope.insert(name.to_string(), id);
        self.idents.insert(id, Ident::new(name.to_string()));
        id
    }

    /// Create new scope.
    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    /// Remove one scope.
    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    /// Remove all scopes.
    pub fn reset_scopes(&mut self) {
        self.scopes.clear();
    }
}
