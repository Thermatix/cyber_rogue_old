use std::collections::HashMap;
use std::ops::{Index, IndexMut};
use std::marker::PhantomData;
use nanoid;


use game::entity::Entity;
use game::entity::Component;

const DELIMIT: char = '.';
const ID_LEN: usize = 15;

type ID = String;
type Category<'m> = &'m str;
type ComponentCat<'m, Comp>= HashMap<ID, Component<'m, ValueType=Comp::ValueType>>;
type EntityComponents<'m, Comp> = HashMap<Category<'m>, ComponentCat<'m, Comp>>;
type Entities = HashMap<ID,Entity>;

pub struct Manager<'m, Comp>
    where Comp: Component<'m>
    {
    pub entities: Entities,
    pub components: EntityComponents<'m, Comp>,
    phantom: PhantomData<&'m ()>
}


impl<'m, Comp: Component<'m>> Manager<'m, Comp> {
    pub fn new() -> Self {
        Self {
            entities: Entities::new(),
            components: EntityComponents::new(),
            phantom: PhantomData,
        }
    }

    /// Create entity of "kind" at Some("location") and add to manager
    pub fn create(&mut self) -> Option<ID> {
        let id: ID = self.generate_id();
        let entity: Entity = Entity::new(id.clone());
        self.entities.insert(id.clone(), entity);
        Some(id.clone())
    }


    /// add a <struct that impliments component> to a given entity 'id' with initial_value of
    /// component::valuetype
    pub fn add_component(&mut self, id: ID, initial_value: Comp::ValueType)
        where Comp: Component<'m>
        {
            if !self.components.contains_key(&Comp::IDName) {
                self.components.insert(Comp::IDName.clone(), ComponentCat::new());
            }
            self.components[&Comp::IDName].insert(id.clone(), Comp::new(initial_value));

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

impl<'m, Comp: Component<'m>> Index<ID> for Manager<'m, Comp> {

    type Output = Entity;

    /// Returns reference to an entity for a given 'id'
    fn index(&self, id: ID) -> &Self::Output {
        &self.entities[&id]
    }
}

impl<'m, Comp: Component<'m>> IndexMut<ID> for Manager<'m, Comp> {

    /// Returns a mutable reference to an entity for a given 'id'
    fn index_mut(&mut self, id: ID) -> &mut Entity {
        self.entities.get_mut(&id).unwrap()
    }
}

