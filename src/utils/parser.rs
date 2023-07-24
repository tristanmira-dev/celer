pub fn parse_request<I: Iterator<Item = String>>(mut iter: I) -> Vec<String> {
    let mut request_vec: Vec<String> = Vec::new();

    let counter: usize = 0;

    loop {
        match iter.next() {
            Some(val) => {
                if val == "" {
                    break
                };

                request_vec.push(val);

                continue;
            },
            None => {
                break;
            }
        };
    
    }
    
    let req_method_vec = &request_vec[0];

    let req_method_vec: Vec<String> = req_method_vec.split(' ').map(|s| s.to_string()).collect();

    return req_method_vec;
    
}