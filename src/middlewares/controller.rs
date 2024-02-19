use std::{any::{self, Any}, net::TcpStream, sync::{Arc, Mutex}};

use super::init::Initializer;
use std::string::String;

#[derive(Debug)]
pub struct ControllerError {}


pub enum ControllerResult {
    StringResult(String),
    IntResult(i32),
    AnyResult(Box<dyn Any + Send>)
}
pub type CustomHandlerClosure = Box<dyn Fn(Arc<Mutex<TcpStream>>, Arc<Mutex<boxedAnyType>>) + Send>;
pub type boxedAnyType = Box<dyn Any + Send>;

pub struct Controller {
    pub path: String,
    pub method: String,
    pub req_handler: Arc<Mutex<Box<dyn Fn(boxedAnyType) -> boxedAnyType + Send>>>,
    pub handler: Arc<Mutex<CustomHandlerClosure>>,
}

impl Controller {
    pub fn new(path : String, method: String, req_handler: Box<dyn Fn(boxedAnyType) -> boxedAnyType + Send>, handler: CustomHandlerClosure ) -> Self {
        Controller{
            path : path,
            method: method,
            req_handler: Arc::new(Mutex::new(req_handler)),
            handler : Arc::new(Mutex::new(handler))
        }
    }
}
