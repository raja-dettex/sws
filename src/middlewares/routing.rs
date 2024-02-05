

use super::{controller::*, init::*};

#[derive(Clone)]
pub struct Router { 
    pub routes : Vec<Controller>
}


impl Initializer<Vec<Controller>> for Router {
    fn init(controllers : Vec<Controller>) -> Self {
        Router { routes : controllers }
    } 
}




