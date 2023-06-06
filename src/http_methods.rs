use super::handlers::route_handler::Route;

pub enum MethodNames {
    PUT,
    POST,
    DELETE,
    GET
}

pub struct Get {
    name: String,
    method: String,
    function: Box<dyn Fn()>,
}

impl Get {
    pub fn new(name: String, function: Box<dyn Fn()>) -> Self {
        Self {
            name,
            method: "GET".to_string(),
            function
        }
    }
}

impl Route for Get {
    fn return_method<'a> (self: &'a Self) -> &'a Box<dyn Fn()> {
        &self.function
    }
    fn ret_http_method(self: &Self) -> &str {
        &self.method
    }
}

pub struct Post {
    name: String,
    method: String,
    function: Box<dyn Fn()>,
}

impl Post {
    pub fn new(name: String, function: Box<dyn Fn()>) -> Self {
        Self {
            name,
            method: "POST".to_string(),
            function
        }
    }
}

impl Route for Post {
    fn return_method<'a> (self: &'a Self) -> &'a Box<dyn Fn()> {
        &self.function
    }
    fn ret_http_method(self: &Self) -> &str {
        &self.method
    }
}

pub struct Delete {
    name: String,
    method: String,
    function: Box<dyn Fn()>,
}

impl Delete {
    pub fn new(name: String, function: Box<dyn Fn()>) -> Self {
        Self {
            name,
            method: "DELETE".to_string(),
            function
        }
    }
}

impl Route for Delete {
    fn return_method<'a> (self: &'a Self) -> &'a Box<dyn Fn()> {
        &self.function
    }
    fn ret_http_method(self: &Self) -> &str {
        &self.method
    }
}

pub struct Put {
    name: String,
    method: String,
    function: Box<dyn Fn()>,
}

impl Put {
    pub fn new(name: String, function: Box<dyn Fn()>) -> Self {
        Self {
            name,
            method: "PUT".to_string(),
            function
        }
    }
}

impl Route for Put {
    fn return_method<'a> (self: &'a Self) -> &'a Box<dyn Fn()> {
        &self.function
    }
    fn ret_http_method(self: &Self) -> &str {
        &self.method
    }
}