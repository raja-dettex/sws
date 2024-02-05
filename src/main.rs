mod serializer_deserializer;
mod parser;
mod pool;
mod network_adaptor;
mod middlewares;

use std::collections::HashMap;
use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::io::prelude::Read;
use std::thread;
use middlewares::controller::Controller;
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
    for i in 0..4 {
        let my_handler = move || {
            println!("hello there index : {}", i );
        }; 
        let controller = Controller::new(format!("/index{}", i), "GET".to_string(), Box::new(my_handler));
        
        controllers.push(controller); 
    }
    let router = Router::init(controllers);
    //let listener = TcpListener::bind("0.0.0.0:8000");
    let t_pool = ThreadPool::new(5);
    let mut transport = TcpTransport::new("0.0.0.0".to_string(), 8000 as i32);
    let ln = transport.listen(); 
    match ln {
        Ok(listener) => {
            listener.start(t_pool, stream_handle, router);
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

fn stream_handle(mut stream: TcpStream, router : Router ) {
    print!("connection from : {:?}\n", stream);
    let mut buff = [0;1024];
    let value = stream.read(&mut buff).unwrap();
    print!("{:?} bytes read from \n", value);
    let request = parse(&String::from_utf8_lossy(&buff[..value]));

    let get = b"GET / HTTP/1.1\r\n";
    let post = b"POST /hello HTTP/1.1\r\n";
    print!("request : \t{:?}\n", request);
    for c in router.routes {
        if request.method == c.method && request.path == c.path { 
            let handler = c.handler.lock().unwrap();
        }
    }
} 


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