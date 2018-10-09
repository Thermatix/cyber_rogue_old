
use sys::TemplateManager;
use sys::EntityManager;
use game::entity::Entity;


pub struct Builder {
    t_manager: TemplateManager,
    e_manager: EntityManager,
}


impl Builder {

    pub fn new(t_manager: TemplateManager, e_manager: EntityManager) -> Self {
        Self {
            t_manager,
            e_manager
        }
    }

    pub fn build_entity(&mut self, entity_type: String, template: String) { // -> Result<Entity, &str> {
        let id = self.e_manager.create();
        'components: for component in self.t_manager[entity_type][template].components {

        }
    }

}
