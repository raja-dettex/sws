


#[cfg(test)]
mod tests {
    use std::borrow::Borrow;
    use std::sync::{mpsc, Arc, Mutex};
    use std::thread::{self, Thread};
    use std::time::{self, Duration};

    use crate::middlewares::controller::*;
    use crate::middlewares::init::Initializer;
    use crate::middlewares::routing::*;
    use crate::pool::thread::ThreadPool;



//     #[test]
//     fn test() {
//         let mut controllers = Vec::new();
//         for i in 0..5 {
//             let handler:Box<dyn Fn() + Send>;
//             if i == 1 {
//                 handler = Box::new(move || {
//                     thread::sleep(time::Duration::from_secs(3));
//                     println!("controller works , {}", i);
//                 });    
//             } else {
//                 handler = Box::new(move || {
//                     println!("controller works , {}", i);
//                 });
//             }
//             let co = Controller::new(format!("/index{}", i), "GET".to_string(), Arc::new(Mutex::new(handler)));
//             controllers.push(co);
//         }
//         let router = Router::init(controllers);
//         let t_pool = ThreadPool::new(4);
//         let (tx, rx) = mpsc::channel();
//         let rx = Arc::new(Mutex::new(rx));
//         let mut i = 1;
//         for co in router.routes {
//             println!("doing something before scheduling thread : {}", i);
//             let tx_clone = tx.clone();
//             let handler_func = co.borrow().handler.clone();
//             t_pool.execute(move || {
//                 let handler  = handler_func.lock().unwrap();
//                 tx_clone.send(()).unwrap();
//                 (handler)();
//             });
//             i += 1;
//         }
//         thread::sleep(time::Duration::from_secs(5));
//         for _ in 0..5 {
//             rx.lock().unwrap().recv().unwrap();
//         }
//     }
}