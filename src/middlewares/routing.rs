use std::{any::{self, Any}, collections::HashMap, net::TcpStream, sync::{Arc, Mutex}};
use std::io::Write;
use serde::Serialize;

use crate::parser::http::{response_string, Response};

use super::{controller::*, init::*};

pub struct Router { 
    pub routes : Vec<Controller>
}


impl  Router {
    pub fn init() -> Self {
        Router{routes : vec![]}
    }

    pub fn add<T, F>(&mut self, path : String, method : String, req_handler: Box<dyn Fn(boxedAnyType) -> boxedAnyType + Send>)
    where 
        T : Any + Send + Serialize,
        F : Any + Send + Serialize
    {
        let handler:Box<dyn Fn(Arc<Mutex<TcpStream>>, Arc<Mutex<Box<dyn Any + Send>>>) + Send> = Box::new(move |arc_stream, result| {
            let mut stream = arc_stream.lock().unwrap();
            let res = result.lock().unwrap();
            println!("result : {:#?}", res);
            let res_option = res.downcast_ref::<T>();
            //println!("res option :{:#?}", res_option);
                    
            match res_option {
                Some(res_obj) => {
                    //println!("res obj :{:#?}", res_obj);
                    let res_str = serde_json::to_string(res_obj).unwrap();
                    let mut headers = HashMap::new();
                    headers.insert("Content-Type".to_string(), "application/json".to_string());
                    let response = Response::new("HTTP/1.1".to_string(), 200, "OK".to_string(), Some(res_str), headers);
                    let response_str = response_string(&response);
                    match response_str {
                        Ok(str) => {
                            stream.write(str.as_bytes()).unwrap();
                            stream.flush().unwrap();
                        }, 
                        Err(e) => {
                            println!("error {:#?}", e)
                        }
                    }
                },
                None =>  {
                    let error_res = res.downcast_ref::<F>();
                    match error_res {
                        Some(res) => {
                            let res_str = serde_json::to_string(res).unwrap();
                            let mut headers = HashMap::new();
                            headers.insert("Content-Type".to_string(), "application/json".to_string());
                            let response = Response::new("HTTP/1.1".to_string(), 404, "NOT Found".to_string(), Some(res_str), headers);
                            let response_str = response_string(&response);
                            match response_str {
                                Ok(str) => {
                                    stream.write(str.as_bytes()).unwrap();
                                    stream.flush().unwrap();
                                }, 
                                Err(e) => {
                                    println!("error {:#?}", e)
                                }
                            }
                        }
                        None => println!("no value to retrieve")
                    }
                }
            }   
        });
        let controller = Controller::new(path, method, req_handler, handler);
        self.routes.push(controller);
    }
}



