use std::{borrow::Cow, collections::HashMap};

mod id;
mod ident;

pub use self::{id::*, ident::*};

#[derive(Debug)]
pub struct ITab {
    next_id: usize,
    idents: HashMap<Id, Ident>,
    unbound: HashMap<String, Id>,
    top: HashMap<String, Id>,
    scopes: Vec<HashMap<String, Id>>,
}

impl ITab {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            unbound: HashMap::new(),
            idents: HashMap::new(),
            top: HashMap::new(),
            scopes: vec![],
        }
    }

    pub fn name_of(&self, id: Id) -> Cow<'_, str> {
        self.idents
            .get(&id)
            .map(|a| a.name().into())
            .unwrap_or_else(|| format!("<{}>", id.0).into())
    }

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

    pub fn insert(&mut self, name: &str) -> Id {
        let id = Id(self.next_id);
        self.next_id += 1;
        let scope = self.scopes.last_mut().unwrap_or(&mut self.top);
        scope.insert(name.to_string(), id);
        self.idents.insert(id, Ident::new(name.to_string()));
        id
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn reset_scopes(&mut self) {
        self.scopes.clear();
    }
}
