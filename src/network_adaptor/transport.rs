use std::{borrow::{Borrow, BorrowMut}, cell::RefCell, io::{Error, Write}, net::{TcpListener, TcpStream}, rc::Rc, sync::{Arc, Mutex}};

use crate::{middlewares::controller::{self, ControllerResult}, parser::http::parse, pool::thread::ThreadPool};
use crate::middlewares::routing::Router;
use std::io::Read;

#[derive(Debug)]
pub struct TcpTransport { 
    host : String,
    port : i32,
    listener : Option<TcpListener>
}

#[derive(Debug)]
pub struct SockError { 
    pub msg : String
}

impl TcpTransport { 
    pub fn new(host: String, port : i32) -> Self {
        TcpTransport {host, port, listener: None}
    }

    pub fn listen(&mut self, pool : ThreadPool, router : Router) -> Result<Listener, SockError>{
        let pool = Arc::new(Mutex::new(pool));
        let router = Arc::new(Mutex::new(router));
        if let Some(ln) = self.listener.take() {
            println!("listening to {} : {}", self.host, self.port);
            
            Ok(Listener{ln: ln, pool, router})

            
        } else {
            let ln = TcpListener::bind(format!("{}:{}", self.host, self.port));
            match ln {
                Ok(listener) => {
                    self.listener = Some(listener.try_clone().unwrap());
                    println!("listening to {} : {}", self.host, self.port);
                    Ok(Listener { ln: listener , pool, router})
                },
                Err(e) => {
                    println!("socket binding error");
                    return Err(SockError{msg:"error occured".to_string()});
                }
            }
        }
    }
}

pub struct Listener {
    ln : TcpListener,
    pool : Arc<Mutex<ThreadPool>>,
    router : Arc<Mutex<Router>>
}

impl Listener {
    pub fn start(&self) {
        
        for stream in self.ln.incoming() {
            if let Ok(mut s) = stream { 
                self.handle_stream( s);
            } else if let Err(e) = stream {
                print!("error occured  : {:?}", e)
            }
        }
    }
    pub fn handle_stream(&self, mut stream : TcpStream) {
        let mut buff = [0;1024];
        let value = stream.read(&mut buff).unwrap();
        let request = parse(&String::from_utf8_lossy(&buff[..value]));
        //let request_clone = Arc::new(Mutex::new(Box::new(request)));

        print!("request : \t{:?}\n", request);
        let router = self.router.clone();
        let stream = Arc::new(Mutex::new(stream));
        let stream_clone = Arc::clone(&stream);
        self.pool.lock().unwrap().execute( move |tcpStream| {
            let router = router.lock();
            match router {
                Ok(routes) => {
            //let tcp_stream_clone = tcpStream.clone();
                    for controller in &routes.routes {
                        if request.path == controller.path && request.method == controller.method {
                            let req_handler = controller.req_handler.lock().unwrap();
                            let result = req_handler(Box::new(request.clone()));
                            let handler_func = controller.handler.lock().unwrap();
                            handler_func(tcpStream.clone(),  Arc::new(Mutex::new(result)));
                        } else if request.path.starts_with(&controller.path) && request.method == controller.method {
                            let req_handler = controller.req_handler.lock().unwrap();
                            let result = req_handler(Box::new(request.clone()));
                            let handler_func = controller.handler.lock().unwrap();
                            handler_func(tcpStream.clone(),  Arc::new(Mutex::new(result)));
                        }
                    }
                },
                Err(e) => println!("error at router : {:#?}", e)
            }
            // tcpStream.lock().unwrap().write("hello".as_bytes());
        },stream.clone() );
        
    } 
    
}

type handle_request = fn(TcpStream );

pub fn handle_stream(stream : TcpStream) {

}