use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Person {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CreatePerson {
    pub first_name: String,
    pub last_name: String,
}

