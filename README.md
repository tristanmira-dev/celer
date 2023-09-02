use std::{any::TypeId, collections::HashMap};

use celer::{Application, resource_sys::system::{Req, Res, IntoSystem, Global, ArgsCollection}, handlers::route_handler::{RouteHandler, RouteBuilder}};

#[derive(Clone, Copy)]
struct CustomObject {
    client: i32
}


fn test_handler (req: Global<CustomObject>) {

    dbg!("{}", req.inner.client);
}

fn route (req: Global<CustomObject>) {

    dbg!("{}", req.inner.client);
}


fn main() {

    let mut test = ArgsCollection::init();


    let mut app = Application::new("127.0.0.1",
    "3000", 
    RouteBuilder::new().get("/test".to_string(), test_handler.into_system()).with_global(Global {
        inner: CustomObject {
            client: 300
        }
    }).get("/route".to_string(), route.into_system()).with_global(Global {
        inner: CustomObject {
            client: 400
        }
    })
    );

    app.run(|| {
        println!("CONNECTTION SUCCESS")
    });

}

