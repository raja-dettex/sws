mod serializer_deserializer;
mod parser;
mod pool;
mod network_adaptor;
mod middlewares;
pub mod types;

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
use serializer_deserializer::lib::{serialize_json, deserialize_json};

use crate::network_adaptor::transport::TcpTransport;
use crate::parser::http::{response_string, Request, Response, parse};
use crate::pool::thread::ThreadPool;
use crate::serializer_deserializer::lib::json_value;
use std::{thread::sleep, time::Duration};
use regex::Regex;


#[derive(Debug, Deserialize, Serialize)]
struct Sample {
    id : i32,
    name: String,
}

fn main() { 
    let mut controllers = Vec::new();
        
        let req_handler : Box<dyn Fn() -> Result<Box<dyn Any + Send>, ControllerError> + Send> = Box::new(|| Ok(Box::new("Hello special") as Box<dyn Any + Send>));
        let handler:Box<dyn Fn(Arc<Mutex<TcpStream>>, Arc<Mutex<Box<dyn Any + Send>>>) + Send> = Box::new(move |arc_stream, result| {
            println!("controller works , {}", 5);
            let mut stream = arc_stream.lock().unwrap();
            let res = result.lock().unwrap();
            let res_str = res.downcast_ref::<&str>().unwrap().to_string();

            let mut headers = HashMap::new();
            headers.insert("Content-Type".to_string(), "text/plain".to_string());
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
            
        });
        let co = Controller::custom_controller(format!("/string"), "GET".to_string(), req_handler, handler);
        
        let another_req_handler : Box<dyn Fn() -> Result<Box<dyn Any + Send>, ControllerError> + Send> = Box::new(|| {
            let boxed_value = Box::new(Sample{id : 1, name: "raja".to_string()});
            Ok(boxed_value as Box<dyn Any + Send>)
        });
        let another_handler:Box<dyn Fn(Arc<Mutex<TcpStream>>, Arc<Mutex<Box<dyn Any + Send>>>) + Send> = Box::new(move |arc_stream, result| {
            println!("controller works , {}", 5);
            let mut stream = arc_stream.lock().unwrap();
            let res = result.lock().unwrap();
            let res_option = res.downcast_ref::<Sample>();
            match res_option {
                Some(res_obj) => {
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
                None => println!("no value to retrieve")
            }
            
            
        });
        let controller = Controller::custom_controller(format!("/custom"), "GET".to_string(), another_req_handler, another_handler);
        
        let post_req_handler : Box<dyn Fn(controller::boxedAnyType) -> Result<Box<dyn Any + Send>, ControllerError> + Send> = Box::new(|val| {
            let req = val.downcast_ref::<Request>();
            println!("here");
            if let Some(request) =  req {
                println!("right here");
                println!("request is {:#?}", request);
                let body = &request.body;
                println!("called from post_req_handler , {:#?}", body.to_string());
                let json_body = serde_json::from_str::<Sample>(body);
                
                if let Ok(value) = json_body {
                    let boxed_value = Box::new(value);
                    Ok(boxed_value as Box<dyn Any + Send>)    
                } else if let Err(e) = json_body {
                    println!("error parsing the body : {:#?}", e);
                    Err(ControllerError {  })
                } else {
                    Err(ControllerError{})
                }
                
            } else {
                Err(ControllerError {  })
            }
            
        });
        let post_handler:Box<dyn Fn(Arc<Mutex<TcpStream>>, Arc<Mutex<Box<dyn Any + Send>>>) + Send> = Box::new(move |arc_stream, result| {
            println!("controller works , {}", 5);
            let mut stream = arc_stream.lock().unwrap();
            let res = result.lock().unwrap();
            let res_option = res.downcast_ref::<Sample>();
            match res_option {
                Some(res_obj) => {
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
                None => println!("no value to retrieve")
            }
            
            
        });
        let post_controller = Controller::custom_post_controller(format!("/post"), "POST".to_string(), post_req_handler, post_handler);
        


        controllers.push(co);
        
        controllers.push(controller);
        controllers.push(post_controller);
        let router = Router::init(controllers);
    //let listener = TcpListener::bind("0.0.0.0:8000");
    let t_pool = ThreadPool::new(5);
    let mut transport = TcpTransport::new("0.0.0.0".to_string(), 8000 as i32);
    let ln = transport.listen(t_pool, router); 
    match ln {
        Ok(listener) => {
            listener.start();
        }, 
        Err(e) => {
            println!("error {:?}", e);
        }
    }
    // match(ln) {
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


fn handle_stream(mut stream: TcpStream) {
    print!("connection from : {:?}\n", stream);
    let mut buff = [0;1024];
    let value = stream.read(&mut buff).unwrap();
    print!("{:?} bytes read from \n", value);
    let request = parse(&String::from_utf8_lossy(&buff[..value]));

    let get = b"GET / HTTP/1.1\r\n";
    let post = b"POST /hello HTTP/1.1\r\n";
    print!("request : \t{:?}\n", request);
    if request.method == "GET" && request.path == "/sleep" {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        let res = Response::new("HTTP/1.1".to_string(), 200 as u32 , "OK".to_string(), Some("hello from sleep".to_string()), headers);
        let res_str = response_string(&res);
        thread::sleep(Duration::from_secs(7));
        stream.write(res_str.unwrap().as_bytes()).unwrap();
        stream.flush().unwrap();
    }
    else if request.method == "GET" && request.path == "/" {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        let res = Response::new("HTTP/1.1".to_string(), 200 as u32 , "OK".to_string(), Some("hello".to_string()), headers);
        let res_str = response_string(&res);                
        stream.write(res_str.unwrap().as_bytes()).unwrap();
        stream.flush().unwrap();
    }else if request.method == "POST" {
        let body = serde_json::from_str::<Sample>(&request.body);
        println!("{:?}", body);
            // }
        match body {
            Ok(sample) => {
                let body_str = serde_json::to_string::<Sample>(&sample);
                match body_str {
                    Ok(str) => {
                        let mut headers = HashMap::new();
                        headers.insert("Content-Type".to_string(), "application/json".to_string());
                        headers.insert("Content-Length".to_string(), str.len().to_string());
                        let res = Response::new("HTTP/1.1".to_string(), 200 as u32 , "OK".to_string(), Some(str), headers);
                        let res_str = response_string(&res);
                        println!("response :{:#?}", res);
                        stream.write(res_str.unwrap().as_bytes()).unwrap();
                        stream.flush().unwrap();
                    }
                    Err(e) => println!("error : {}",e)
                } 
                
            }       
            Err(e) => println!("err {}", e)
        }
    } 
    else { 
        print!("invalid request")
    }
    
}