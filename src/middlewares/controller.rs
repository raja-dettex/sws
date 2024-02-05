use std::sync::{Arc, Mutex};

use super::init::Initializer;

#[derive(Clone)]
pub struct Controller {
    pub path : String,
    pub method : String,
    pub handler : HandlerClosure
}


pub type HandlerClosure = Arc<Mutex<Box<dyn Fn() + Send >>>;


impl Controller {
    pub fn new<F>(path : String, method: String, handler : F) -> Self
    where 
        F : Fn() + Send
    {
        Controller{path, method, handler: Arc::new(Mutex::new(Box::new(handler)))}
    }
}