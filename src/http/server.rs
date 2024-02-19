use std::sync::{Arc, Mutex};

use crate::{middlewares::routing::Router, network_adaptor::transport::TcpTransport, pool::thread::ThreadPool};


pub struct InternalServerError{
    pub error : String
}
pub struct ServerOpts {
    pub host: String,
    pub port : i32
}
pub struct Server {
    thread_count : i32,
    transport : TcpTransport
}

impl Server {
    pub fn new(opts : ServerOpts, thread_count : i32) -> Arc<Mutex<Self>> {
        let transport = TcpTransport::new(opts.host, opts.port);
        let server = Server {thread_count, transport};
        Arc::new(Mutex::new(server))
    }

    pub fn start(&mut self, router : Router) -> Result<(), InternalServerError> {
        let pool = ThreadPool::new(self.thread_count as usize);
        let res = self.transport.listen(pool, router);
        match res {
            Ok(listener) => {
                listener.start();
                Ok(())
            }
            Err(e) => {
                Err(InternalServerError{error : e.msg})
            }
        }
    } 
    pub fn doSomething(&self) -> String{
        println!("doing things");
        "hello".to_string()
    }
}