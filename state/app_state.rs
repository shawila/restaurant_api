use std::sync::Mutex;
use std::collections::HashMap;
use crate::models::order::MenuItem;

#[derive(Default)]
pub struct AppState {
    pub orders: Mutex<HashMap<String, Vec<MenuItem>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self { orders: Mutex::new(HashMap::new()) }
    }
}

