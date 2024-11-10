use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MenuItem {
    pub name: String,
    #[serde(default)] 
    pub cooking_time: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TableOrder {
    pub table_number: String,
    pub items: Vec<MenuItem>,
}

