
mod parser;
mod pool;
mod network_adaptor;
mod middlewares;
pub mod types;
pub mod http;
mod storage;

use std::any::Any;
use std::collections::HashMap;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::io::prelude::Read;
use std::sync::{Arc, Mutex};
use std::{thread, time};
use middlewares::controller::{self, Controller, ControllerError};
use middlewares::init::Initializer;
use middlewares::routing::Router;
use serde::{Serialize, Deserialize};
use serde_json::from_str as deserialize;
use storage::store::Store;
use types::custom::Sample;

use crate::network_adaptor::transport::TcpTransport;
use crate::parser::http::{response_string, Request, Response, parse};
use crate::pool::thread::ThreadPool;
use std::{thread::sleep, time::Duration};
use regex::Regex;
use crate::http::server::{ServerOpts, Server};

#[derive(Debug, Deserialize, Serialize)]
pub struct CustomError {
    msg : String
}
#[derive(Clone)]
pub struct MemoryStore { 
    data : HashMap<String , Sample>
}

impl MemoryStore {
    pub fn new() -> Arc<Mutex<Self>> {
        Arc::new(Mutex::new(MemoryStore{data: HashMap::new()}))
    }
}

impl Store<Sample> for MemoryStore {
    fn getAll(&self) -> Vec<Sample> {
        let mut values = Vec::new();
        for (k, v) in self.data.clone() {
            values.push(v);
        }
        values
    }

    fn get(&self, key : String) -> Option<Sample> {
        let val = self.data.get(&key);
        match val {
            Some(r) => {
                let data = r.to_owned();
                Some(data)
            }
            None => None
        }
    }

    fn add(&mut self, key: String, val : Sample) -> Option<Sample> {
        let added = self.data.insert(key.clone(), val);
    // Return the added value, even if the key already existed
        added.or_else(|| self.data.get(&key).cloned())
    }

    fn update(&mut self, key: String, val : Sample) -> Option<Sample> {
        self.data.insert(key, val)

    }
    fn delete(&mut self, key: String) -> Option<Sample> {
        self.data.remove(&key)
    }
}
fn main() { 
        let m_store = MemoryStore::new();
       
        let opts = ServerOpts { host : "0.0.0.0".to_string(), port : 8000};
        let mut server = Server::new(opts, 5);
        let server_clone = Arc::clone(&server);
        let another_clone = Arc::clone(&server);
        let another_req_handler : Box<dyn Fn(Box<dyn Any + Send>) -> Box<dyn Any + Send> + Send> = Box::new(move |val| {
            let boxed_value = Box::new(Sample{id : 1 as u32, name: "raja".to_string()});
           // let val = server_clone.lock().unwrap().doSomething();
            //println!("{}", val);
            boxed_value
        });

        let string_handler : Box<dyn Fn(Box<dyn Any + Send>) -> Box<dyn Any + Send> + Send> = Box::new( move |val| {
            let boxed_value = Box::new("welcome home".to_string());
            //println!("{}", val);
            boxed_value
        });
        let m_store_clone = Arc::clone(&m_store);
        let get_sample_handler : Box<dyn Fn(Box <dyn Any + Send>) -> Box<dyn Any + Send> + Send> = Box::new(move |val| {
            let request = val.downcast_ref::<Request>();
            match request {
                Some(req) => {
                   let params:Vec<&str> =  req.path.split('/').collect();
                   let param = params.get(params.len() - 1);
                   match param {
                        Some(key) => {
                            println!("key is : {:#?}", key);
                            let found = m_store_clone.lock().unwrap().get(key.to_string());
                            match found {
                                Some(val) => {
                                    Box::new(val)
                                }
                                None => {
                                    let c_error = CustomError{msg: "error".to_string()};
                                    Box::new(c_error)
                                }
                            }
                        }
                        None => {
                            let c_error = CustomError{msg: "error".to_string()};
                            Box::new(c_error)
                        }
                   }
                   
                }
                None => {
                    let c_error = CustomError{msg: "error".to_string()};
                    Box::new(c_error)
                }
            }
        });
        let m_store_clone = Arc::clone(&m_store);
        let get_all_samples_handler : Box<dyn Fn(Box<dyn Any + Send>) -> Box<dyn Any + Send> + Send> = Box::new(move |val| {
            let samples = m_store_clone.lock().unwrap().getAll();
            Box::new(samples)
        });

        
        let m_store_clone = Arc::clone(&m_store);
        let post_req_handler : Box<dyn Fn(Box<dyn Any + Send>) -> Box<dyn Any + Send> + Send> = Box::new(move |val| {
            let req = val.downcast_ref::<Request>();
            if let Some(request) =  req {
                let body = &request.body;
                let json_body = serde_json::from_str::<Sample>(body);
                
                if let Ok(value) = json_body {
                    let another_valued = value.clone();
                    let second_valued = value.clone();
                    println!("value :{:#?}", another_valued);
                    if let Some(val) = m_store_clone.lock().unwrap().add(value.name, second_valued) {
                        println!("{:#?}", val);
                        let boxed_value = Box::new(val);  
                        boxed_value 
                    } else {
                        Box::new(CustomError{msg: "error in post controller".to_string()})
                    }
                } else if let Err(e) = json_body {
                    println!("error parsing the body : {:#?}", e);
                    let c_error = CustomError{msg: e.to_string()};
                    Box::new(c_error)
                } else {
                    let c_error = CustomError{msg: "error".to_string()};
                    Box::new(c_error)    
                }
            } else {
                let c_error = CustomError{msg: "error".to_string()};
                Box::new(c_error)
            }
        });
        

        

        let mut router = Router::init();
        router.add::<Sample, CustomError>("/custom".to_string(), "GET".to_string(), another_req_handler);
        router.add::<String, CustomError>("/home".to_string(), "GET".to_string(), string_handler);
        router.add::<Sample, CustomError>("/post".to_string(), "POST".to_string(), post_req_handler);
        router.add::<Sample, CustomError>("/sample".to_string(), "GET".to_string(), get_sample_handler);
        router.add::<Vec<Sample>, CustomError>("/all".to_string(), "GET".to_string(), get_all_samples_handler);
        
        let result = server.lock().unwrap().start(router);
        if let Err(err) = result  {
            println!("error : {}", err.error)
        }
    // match(ln) {.
    //     Ok(ln) => { 
    //         for stream in ln.incoming() {
    //             if let Ok(mut s) = stream {
    //                 t_pool.execute(|| {
    //                     stream_handle(s, router)
    //                 });
    //                 //handle_stream(s)
    //             } else if let Err(e) = stream {
    //                 print!("error occured  : {:?}", e)
    //             }
    //         }
    //     }
    //     Err(e) => {print!("err{:?}", e)}
    // }
}

// fn stream_handle(mut stream: TcpStream, router : Router ) {
//     print!("connection from : {:?}\n", stream);
//     let mut buff = [0;1024];
//     let value = stream.read(&mut buff).unwrap();
//     print!("{:?} bytes read from \n", value);
//     let request = parse(&String::from_utf8_lossy(&buff[..value]));

//     let get = b"GET / HTTP/1.1\r\n";
//     let post = b"POST /hello HTTP/1.1\r\n";
//     print!("request : \t{:?}\n", request);
//     for c in router.routes {
//         if request.method == c.method && request.path == c.path { 
//             //(c.handler)();
//         }
//     }
// } 


// fn handle_stream(mut stream: TcpStream) {
//     print!("connection from : {:?}\n", stream);
//     let mut buff = [0;1024];
//     let value = stream.read(&mut buff).unwrap();
//     print!("{:?} bytes read from \n", value);
//     let request = parse(&String::from_utf8_lossy(&buff[..value]));

//     let get = b"GET / HTTP/1.1\r\n";
//     let post = b"POST /hello HTTP/1.1\r\n";
//     print!("request : \t{:?}\n", request);
//     if request.method == "GET" && request.path == "/sleep" {
//         let mut headers = HashMap::new();
//         headers.insert("Content-Type".to_string(), "application/json".to_string());
//         let res = Response::new("HTTP/1.1".to_string(), 200 as u32 , "OK".to_string(), Some("hello from sleep".to_string()), headers);
//         let res_str = response_string(&res);
//         thread::sleep(Duration::from_secs(7));
//         stream.write(res_str.unwrap().as_bytes()).unwrap();
//         stream.flush().unwrap();
//     }
//     else if request.method == "GET" && request.path == "/" {
//         let mut headers = HashMap::new();
//         headers.insert("Content-Type".to_string(), "application/json".to_string());
//         let res = Response::new("HTTP/1.1".to_string(), 200 as u32 , "OK".to_string(), Some("hello".to_string()), headers);
//         let res_str = response_string(&res);                
//         stream.write(res_str.unwrap().as_bytes()).unwrap();
//         stream.flush().unwrap();
//     }else if request.method == "POST" {
//         let body = serde_json::from_str::<Sample>(&request.body);
//         println!("{:?}", body);
//             // }
//         match body {
//             Ok(sample) => {
//                 let body_str = serde_json::to_string::<Sample>(&sample);
//                 match body_str {
//                     Ok(str) => {
//                         let mut headers = HashMap::new();
//                         headers.insert("Content-Type".to_string(), "application/json".to_string());
//                         headers.insert("Content-Length".to_string(), str.len().to_string());
//                         let res = Response::new("HTTP/1.1".to_string(), 200 as u32 , "OK".to_string(), Some(str), headers);
//                         let res_str = response_string(&res);
//                         println!("response :{:#?}", res);
//                         stream.write(res_str.unwrap().as_bytes()).unwrap();
//                         stream.flush().unwrap();
//                     }
//                     Err(e) => println!("error : {}",e)
//                 } 
                
//             }       
//             Err(e) => println!("err {}", e)
//         }
//     } 
//     else { 
//         print!("invalid request")
//     }
    
// }