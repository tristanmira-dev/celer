trait ToRes {
    fn res(content: String) -> Response;
}


struct Map {
    key: String,
    val: String
}

pub struct Response {
    message: String,
    code: i32,
    req_opt: Vec<Map>,
}

impl ToRes for Response {
    fn res (content: String) -> Response {
        return Response { message: String::new(), code: 0, req_opt: Vec::new()}
    }
}
