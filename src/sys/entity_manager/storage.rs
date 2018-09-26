use std::collections::HashMap;
use std::ops::{Index, IndexMut};
use game::entity::Component;

pub type ID = String;
pub type Components<Comp: Component> = HashMap<ID, Comp>;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Storage<Component> {
    components: Components<Box<Component>>,
}

impl<Comp: Component> Storage<Comp>
where Comp: Component+'static
{

    /// Returns a New Component Storage
    pub fn new() -> Self
    {
        Self{
            components: Components::new()
        }
    }

    pub fn insert(&self, id: ID, comp: Comp) {
        self.components.insert(id, Box::new(comp));
    }
}

impl<Comp: Component> Index<ID> for Storage<Comp> {
    type Output = Comp;

    /// returns a Component for a given ID
    fn index(&self, id: ID) -> &Self::Output
    {
        & *self.components[&id]
    }
}

impl<Comp: Component> IndexMut<ID> for Storage<Comp> {

    /// Returns a mutable Component for a given ID
    fn index_mut(&mut self, id: ID) -> &mut Self::Output {
        &mut *self.components.get_mut(&id).unwrap()
    }
}
