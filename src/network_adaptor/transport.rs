use std::{borrow::{Borrow, BorrowMut}, cell::RefCell, io::Error, net::{TcpListener, TcpStream}, rc::Rc, sync::{Arc, Mutex}};

use crate::pool::thread::ThreadPool;
use crate::middlewares::routing::Router;

#[derive(Debug)]
pub struct TcpTransport { 
    host : String,
    port : i32,
    listener : Option<TcpListener>
}

#[derive(Debug)]
pub struct SockError { 
    msg : String
}

impl TcpTransport { 
    pub fn new(host: String, port : i32) -> Self {
        TcpTransport {host, port, listener: None}
    }

    pub fn listen(&mut self) -> Result<Listener, SockError>{
        if let Some(ln) = self.listener.take() {
            println!("listening to {} : {}", self.host, self.port);
            Ok(Listener{ln: ln})

            
        } else {
            let ln = TcpListener::bind(format!("{}:{}", self.host, self.port));
            match ln {
                Ok(listener) => {
                    self.listener = Some(listener.try_clone().unwrap());
                    println!("listening to {} : {}", self.host, self.port);
                    Ok(Listener { ln: listener })
                },
                Err(e) => {
                    println!("socket binding error");
                    return Err(SockError{msg:"error occured".to_string()});
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct Listener {
    ln : TcpListener
}

impl Listener {
    pub fn start(&self, mut t_pool : ThreadPool, handler : stream_handler, router : Router) {
        
        for stream in self.ln.incoming() {
            if let Ok(mut s) = stream {
                
                t_pool.execute( move || { 
                    handler(s, router);
                });
            } else if let Err(e) = stream {
                print!("error occured  : {:?}", e)
            }
        }
    }
    
}

type stream_handler = fn(TcpStream, Router);

pub fn handle_stream(stream : TcpStream) {

}