use std::{any::{self, Any}, net::TcpStream, sync::{Arc, Mutex}};

use super::init::Initializer;
use std::string::String;

#[derive(Debug)]
pub struct ControllerError {}

pub enum Controller { 
    StringController(StringController),
    IntController(IntController),
    CustomController(CustomController),
    CustomPostController(CustomPostController)
}

pub enum ControllerResult {
    StringResult(String),
    IntResult(i32),
    AnyResult(Box<dyn Any + Send>)
}
// pub type Req_Handler_closure = Box<dyn Fn() + Send
pub type StringHandlerClosure = Box<dyn Fn(Arc<Mutex<TcpStream>>, Arc<Mutex<String>>) + Send>;
pub type IntHandlerClosure = Box<dyn Fn(Arc<Mutex<TcpStream>>, Arc<Mutex<i32>>) + Send>;
pub type CustomHandlerClosure = Box<dyn Fn(Arc<Mutex<TcpStream>>, Arc<Mutex<boxedAnyType>>) + Send>;
pub type boxedAnyType = Box<dyn Any + Send>;
pub struct StringController {
    pub path: String,
    pub method: String,
    pub req_handler: Arc<Mutex<Box<dyn Fn() -> Result<ControllerResult, ControllerError> + Send>>>,
    pub handler: Arc<Mutex<Box<dyn Fn(Arc<Mutex<TcpStream>>, Arc<Mutex<String>>) + Send>>>,
}

pub struct IntController {
    pub path: String,
    pub method: String,
    pub req_handler: Arc<Mutex<Box<dyn Fn() -> Result<ControllerResult, ControllerError> + Send>>>,
    pub handler: Arc<Mutex<Box<dyn Fn(Arc<Mutex<TcpStream>>, Arc<Mutex<i32>>) + Send>>>,
}
pub struct CustomController {
    pub path: String,
    pub method: String,
    pub req_handler: Arc<Mutex<Box<dyn Fn() -> Result<ControllerResult, ControllerError> + Send>>>,
    pub handler: Arc<Mutex<CustomHandlerClosure>>,
}
pub struct CustomPostController {
    pub path: String,
    pub method: String,
    pub req_handler: Arc<Mutex<Box<dyn Fn(boxedAnyType) -> Result<ControllerResult, ControllerError> + Send>>>,
    pub handler: Arc<Mutex<CustomHandlerClosure>>,
}

impl Controller {
    pub fn new_string_controller(path: String, method: String, req_handler: Box<dyn Fn() -> Result<ControllerResult, ControllerError> + Send>, handler: StringHandlerClosure) -> Self {
        Controller::StringController(StringController {
            path,
            method,
            req_handler: Arc::new(Mutex::new(req_handler)),
            handler: Arc::new(Mutex::new(handler)),
        })
    }

    pub fn new_int_controller(path: String, method: String, req_handler: Box<dyn Fn() -> Result<ControllerResult, ControllerError> + Send>, handler: IntHandlerClosure) -> Self {
        Controller::IntController(IntController {
            path,
            method,
            req_handler: Arc::new(Mutex::new(req_handler)),
            handler: Arc::new(Mutex::new(handler)),
        })
    }
    pub fn custom_controller<T>(path: String, method: String, req_handler: T, handler : CustomHandlerClosure) -> Controller 
    where 
        T :  Fn() -> Result<Box<dyn Any + Send>, ControllerError> + Send + 'static
    {
        let adapted_handler: Box<dyn Fn() -> Result<ControllerResult, ControllerError> + Send> =
            Box::new(move || req_handler().map(|result| ControllerResult::AnyResult(result)));
         Controller::CustomController(CustomController {
            path,
            method,
            req_handler: Arc::new(Mutex::new(adapted_handler)),
            handler: Arc::new(Mutex::new(handler)),
        })
    }

    pub fn custom_post_controller<T>(path : String, method : String, req_handler: T, handler : CustomHandlerClosure) -> Self 
    where
        T : Fn(boxedAnyType) -> Result<boxedAnyType, ControllerError> + Send + 'static
    {
        let adapted_handler: Box<dyn Fn(boxedAnyType) -> Result<ControllerResult, ControllerError> + Send> =
            Box::new(move |val| req_handler(val).map(|result| ControllerResult::AnyResult(result)));
        Controller::CustomPostController(CustomPostController{
            path, 
            method,
            req_handler: Arc::new(Mutex::new(adapted_handler)),
            handler: Arc::new(Mutex::new(handler)),
        })
    }
}