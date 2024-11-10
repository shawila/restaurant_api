use std::sync::Mutex;
use std::collections::HashMap;
use crate::models::order::MenuItem;

#[derive(Default)]
pub struct AppState {
    pub orders: Mutex<HashMap<String, Vec<MenuItem>>>, // Table orders
}

