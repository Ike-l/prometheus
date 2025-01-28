use std::fmt::Display;


#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Identity<K> {
    pub id: K
}

impl<K> Display for Identity<K> where K: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl<K> Identity<K> {
    pub fn new(id: K) -> Self {
        Self {
            id
        }
    }
}

impl From<&'static str> for Identity<String> {
    fn from(id: &'static str) -> Self {
        Self::new(id.to_string())
    }
}
