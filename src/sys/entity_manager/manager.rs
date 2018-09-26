use std::collections::HashMap;
use std::ops::{Index, IndexMut};
use std::any::TypeId;
use nanoid;


use game::entity::Entity;
use game::entity::{Component, ComponentFields} ;
use super::storage::ID;
use super::Storage;

use typemap::{TypeMap, Key};

const DELIMIT: char = '.';
const ID_LEN: usize = 15;

type EntityComponents = TypeMap;
type Entities = HashMap<ID,Entity>;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ComponentId<Comp>;

impl<Comp: Component> Key for ComponentId<Comp>  {
    type Value = Storage<Comp>;
}

impl<Comp: Component> ComponentId<Comp> {
    fn new() -> Self {
        Self {}
    }
}

pub struct Manager {
    pub entities: Entities,
    pub components: EntityComponents,
}


impl Manager {
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
    pub fn add_component< Comp: Component+'static>(&mut self, id: ID, initial_value: Comp::ValueType)
    where Comp: Component + ComponentFields
    {
            let comp_storage: Storage<Comp> = Storage::new();
            if !self.components.contains::<ComponentId<Comp>>() {
                self.components.insert::<ComponentId<Comp>>(comp_storage);
            }
            self.components.get_mut::<ComponentId<Comp>>().unwrap().insert(id.clone(), Comp::new(initial_value));
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

impl Index<ID> for Manager {

    type Output = Entity;

    /// Returns reference to an entity for a given 'id'
    fn index(&self, id: ID) -> &Self::Output {
        &self.entities[&id]
    }
}

impl IndexMut<ID> for Manager {

    /// Returns a mutable reference to an entity for a given 'id'
    fn index_mut(&mut self, id: ID) -> &mut Entity {
        self.entities.get_mut(&id).unwrap()
    }
}
