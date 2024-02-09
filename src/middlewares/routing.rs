

use super::{controller::*, init::*};

pub struct Router { 
    pub routes : Vec<Controller>
}


impl  Router {
    pub fn init(controllers : Vec<Controller>) -> Self {
        Router{routes : controllers}
    }
}



