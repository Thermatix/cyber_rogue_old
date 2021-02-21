use super::Component;
use sys::data_manager::manager::Component as TComponent;

pub fn select_component(tcomponent: &TComponent) -> impl Component {
    match tcomponent.name {
        "location".to_string()  => simple::Location::new(component.initial_value.clone()),
        "kind".to_string()      => simple::Kind::new(component.initial_value.clone()),
        "position".to_string()  => simple::Position::new(component.initial_value.clone()),
        "name".to_string()      => simple::Name::new(component.initial_value.clone()),
    }
}
