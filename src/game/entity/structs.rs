pub struct Entity {
    pub id: String,
}

impl Entity {

    pub fn new(id: String) -> Self {
        Self {
            id
        }
    }
}
