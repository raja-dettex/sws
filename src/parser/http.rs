use std::collections::HashMap;


#[derive(Debug)]
pub struct ResponseEncodingErr {
    pub message : String 
}

#[derive(Debug,Clone)]
pub struct Request {
   pub method : String,
   pub version : String,
   pub path : String,
   pub headers : HashMap<String , String>,
   pub body : String
}




#[derive(Debug)]
pub struct Response {
    pub version : String,
    pub StatusCode : u32,
    pub statusMessage: String,
    pub headers : HashMap<String , String>,
    pub body : Option<String>
}
impl Request { 
    pub fn new(method: String , version:String, path: String , body : String, headers : HashMap<String , String>) -> Self {
        Request{
            method: method,
            path: path,
            version: version,
            headers: headers,
            body: body
        }
    }
}

impl Response { 
    pub fn new(version:String, StatusCode : u32, statusMessage: String,  body : Option<String>, headers : HashMap<String , String>) -> Self {
        Response{
            version: version,
            StatusCode : StatusCode,
            statusMessage: statusMessage,
            headers: headers,
            body: body
        }
    }
}



pub fn parse(request_str : &str) -> Request {
    let mut lines = request_str.lines();
    let first_line = lines.next().and_then(|line | Some(line) ).unwrap_or_default();
    let mut parts = first_line.split_whitespace();
    let method = parts.next().unwrap_or_default().to_string();
    let path = parts.next().unwrap_or_default().to_string();
    let version = parts.next().unwrap_or_default().to_string();
    let mut headers = HashMap::new();
    while let Some(line) = lines.next() {
        if line.is_empty() { 
            break;
        }
        let mut pairs = line.splitn(2, ": ");
        if let (Some(key), Some(val) ) = (pairs.next(), pairs.next()) {
            headers.insert(key.to_string(), val.to_string());
        }

    }
    let body = lines.collect::<String>();
    let request = Request::new(method, version, path, body, headers);
    request
}
pub fn response_string(response : &Response) -> Result<String, ResponseEncodingErr> {
    if response.version.is_empty() || response.statusMessage.is_empty() || response.headers.is_empty() {
        return Err(ResponseEncodingErr { message : "invalid response".to_string()});
    }
    let mut resp_str = format!("{} {} {}", response.version, response.StatusCode, response.statusMessage); 
    for (k , v) in response.headers.iter() {
        let mut header_str = format!("\r\n{}:{}", k.to_string(), v.to_string());
        resp_str.push_str(&header_str);
    }
    if let Some(resp_body ) =  response.body.as_deref(){

        let body_str = format!("\r\n\r\n{}", resp_body.to_string());
        resp_str.push_str(&body_str);
    }
    
    Ok(resp_str)
}