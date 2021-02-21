mod components;

mod structs;

pub use self::structs::*;
pub use self::components::*;

// use ::sys::InitialValue;

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(tag="name")]
// enum Feature {
//     Kind {value: <Kind as Component>::ValueType },
//     Location {value: <Location as Component>::ValueType },
//     Name {value: <Name as Component>::ValueType },
//     Position {value: <Position as Component>::ValueType },
//     Char {value: <Char as Component>::ValueType },
// }

// pub fn get(comp_name: &String, ini_v: &InitialValue) -> Box<Component<>> {
//     match comp_name.as_ref() {
//         "kind"      => Box::new(Kind::new(ini_v)),
//         "location"  => Box::new(Location::new(ini_v)),
//         "name"      => Box::new(Name::new(ini_v)),
//         "position"  => Box::new(Position::new(ini_v)),
//         "char"      => Box::new(Char::new(ini_v)),
//     }
// }
