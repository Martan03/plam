/// Identifier within the code.
#[derive(Debug)]
pub struct Ident {
    name: String,
}

impl Ident {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}
