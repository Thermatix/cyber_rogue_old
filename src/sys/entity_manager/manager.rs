use std::collections::HashMap;
use std::ops::{Index, IndexMut};
use std::any::TypeId;
use nanoid;


use game::entity::Entity;
use game::entity::{Component, ComponentFields} ;
use super::storage::ID;
use super::Storage;

const DELIMIT: char = '.';
const ID_LEN: usize = 15;

type EntityComponents<Comp> = HashMap<ComponentId, Storage<Comp>>;
type Entities = HashMap<ID,Entity>;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ComponentId(pub TypeId);

impl ComponentId {

    /// Creates a new resource id from a given type.
    pub fn new<'c, C: Component+'c>() -> Self {
        ComponentId(TypeId::of::<C>())
    }
}

pub struct Manager< Comp>
    where Comp: Component
    {
    pub entities: Entities,
    pub components: EntityComponents<Comp>,
}


impl< Comp: Component> Manager<Comp> {
    pub fn new() -> Self {
        Self {
            entities: Entities::new(),
            components: EntityComponents::new(),
        }
    }

    /// Create entity and return it's "ID"
    pub fn create(&mut self) -> ID {
        let id: ID = self.generate_id();
        let entity: Entity = Entity::new(id.clone());
        self.entities.insert(id.clone(), entity);
        id.clone()
    }


    /// add a <struct that impliments component> to a given entity 'id' with initial_value of
    /// component::valuetype
    pub fn add_component(&mut self, id: ID, initial_value: Comp::ValueType)
    where Comp: Component + ComponentFields
    {
            let comp_type = ComponentId::new::<Comp>();
            if !self.components.contains_key(&comp_type) {
                self.components.insert(comp_type.clone(), Storage::new());
            }
            self.components[&comp_type].insert(id.clone(), Comp::new(initial_value));
    }


    /// Generate new id that doesn't exist
    fn generate_id(&self) -> ID{
        let mut id: ID = nanoid::generate(ID_LEN);
        while self.entities.contains_key(&id) {
            id = nanoid::generate(ID_LEN);
        };
        id
    }
}

impl<Comp: Component> Index<ID> for Manager<Comp> {

    type Output = Entity;

    /// Returns reference to an entity for a given 'id'
    fn index(&self, id: ID) -> &Self::Output {
        &self.entities[&id]
    }
}

impl<Comp: Component> IndexMut<ID> for Manager<Comp> {

    /// Returns a mutable reference to an entity for a given 'id'
    fn index_mut(&mut self, id: ID) -> &mut Entity {
        self.entities.get_mut(&id).unwrap()
    }
}
