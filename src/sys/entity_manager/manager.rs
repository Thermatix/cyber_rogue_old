use std::collections::HashMap;
use std::ops::{Index, IndexMut};
use std::marker::PhantomData;
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
pub struct ComponentId<Comp: Component> {
    _phantom: PhantomData<Comp>
}

impl<Comp: Component+'static> Key for ComponentId<Comp>  {
    type Value = Storage<Comp>;
}

impl<Comp: Component+'static> ComponentId<Comp> {
    fn new() -> Self {
        Self {_phantom: PhantomData }
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
    pub fn add_component<Comp: Component+'static>(&mut self, id: ID, initial_value: Comp::ValueType)
    where Comp: Component + ComponentFields
    {
            let comp_storage: Storage<Comp> = Storage::new();
            if !self.components.contains::<ComponentId<Comp>>() {
                self.components.insert::<ComponentId<Comp>>(comp_storage);
            }
            self.components.get_mut::<ComponentId<Comp>>().unwrap().insert(id.clone(), Comp::new(initial_value));
    }

    pub fn get_component<Comp: Component+'static>(&self, id: ID) -> Option<& impl Component> {
        match self.components.get::<ComponentId<Comp>>() {
            Some(storage) => Some(&storage[id]),
            None => None
        }
    }

    pub fn get_mut_component<Comp: Component+'static>(&mut self, id: ID) -> Option<&mut impl Component> {
        match self.components.get_mut::<ComponentId<Comp>>() {
            Some(storage) => Some(&mut storage[id]),
            None => None
        }
    }

    /// Generate new id that doesn't exist
    fn generate_id(&self) -> ID{
        let mut id: ID = nanoid::generate(ID_LEN);
        while self.entities.contains_key(&id) {
            id = nanoid::generate(ID_LEN);
        };
        id
    }

    fn build(&mut self, entity_type: String, template_name: String, data_manager: &::sys::DataManager ) {
        let template = &data_manager.template_types[&entity_type][template_name];
        let id = self.create();


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
