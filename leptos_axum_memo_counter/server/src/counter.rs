use leptos::prelude::*;
use once_cell::sync::OnceCell;
use std::sync::{Arc, Mutex};

//////////
// Repo //
//////////

#[derive(Clone)]
pub struct CounterRepo {
    value: Arc<Mutex<i32>>,
}

static INSTANCE: OnceCell<Arc<CounterRepo>> = OnceCell::new();

pub fn get_instance() -> Arc<CounterRepo> {
    INSTANCE.get_or_init(|| CounterRepo::init()).clone()
}

impl CounterRepo {
    pub fn init() -> Arc<Self> {
        Arc::new(CounterRepo {
            value: Arc::new(Mutex::new(0)),
        })
    }

    pub fn new() -> Self {
        CounterRepo {
            value: Arc::new(Mutex::new(0)),
        }
    }

    pub async fn increment(&self) {
        let mut value = self.value.lock().unwrap();
        *value += 1;
    }

    pub async fn decrement(&self) {
        let mut value = self.value.lock().unwrap();
        *value -= 1;
    }

    pub async fn get_value(&self) -> i32 {
        let value = self.value.lock().unwrap();
        *value
    }
}
