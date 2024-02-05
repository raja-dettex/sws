use std::{collections::HashMap, fmt};

#[derive(Debug,Clone, PartialEq)]
pub enum json_value {
    String(String),
    Number(f64),
    Object(HashMap<String, json_value>),
    Array(Vec<json_value>),
    Bool(bool),
    Null
}



impl fmt::Display for json_value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            json_value::String(s) => write!(f, "\"{}\"", s),
            json_value::Number(n) => write!(f, "{}", n),
            json_value::Object(obj) => {
                let inner = obj
                    .iter()
                    .map(|(key, val)| format!("\"{}\":{}", key, val))
                    .collect::<Vec<_>>()
                    .join(",");
                write!(f, "{{{}}}", inner)
            }
            json_value::Array(arr) => {
                let inner = arr.iter().map(|val| format!("{}", val)).collect::<Vec<_>>().join(",");
                write!(f, "[{}]", inner)
            }
            json_value::Bool(b) => write!(f, "{}", b),
            json_value::Null => write!(f, "null"),
        }
    }
}

pub fn serialize_json(value: &json_value) -> String {
    let mut result = String::new();
    serialize_json_recursive(value, &mut result);
    result
}

fn serialize_json_recursive(value: &json_value, result: &mut String) {
    match value {
        json_value::String(s) => result.push_str(&format!("\"{}\"", s)),
        json_value::Number(n) => result.push_str(&n.to_string()),
        json_value::Object(obj) => {
            result.push('{');
            for (key, val) in obj.iter() {
                result.push_str(&format!("\"{}\":", key));
                serialize_json_recursive(val, result);
                result.push(',');
            }
            if obj.len() > 0 {
                result.pop(); // Remove trailing comma
            }
            result.push('}');
        }
        json_value::Array(arr) => {
            result.push('[');
            for val in arr.iter() {
                serialize_json_recursive(val, result);
                result.push(',');
            }
            if arr.len() > 0 {
                result.pop(); // Remove trailing comma
            }
            result.push(']');
        }
        json_value::Bool(b) => result.push_str(&b.to_string()),
        json_value::Null => result.push_str("null"),
    }
}


pub fn deserialize_json(input : &str)  -> Result<json_value, &'static str> {
    let mut chars = input.chars();
    parse_value(&mut chars)
}


pub fn parse_value(chars: &mut std::str::Chars) -> Result<json_value, &'static str> {
    let c = chars.next();
    match c {
        Some('"') => parse_string(chars),
        Some('0'..='9') | Some('-') => parse_number(c.unwrap(), chars),
        Some('{') => parse_object(chars),
        Some('[') => parse_array(chars),
        Some('t' | 'f') => parse_Bool(c.unwrap(), chars),
        Some('n') => parse_null(chars),
        _ => Err("Unexpected character"),
    }
}

fn parse_string(chars: &mut std::str::Chars) -> Result<json_value, &'static str> {
    let mut result = String::new();
    while let Some(c) = chars.next() {
        match c {
            '"' => return Ok(json_value::String(result)),
            _ => result.push(c),
        }
    }
    Err("Unterminated string")
}

fn parse_number(first_char: char, chars: &mut std::str::Chars) -> Result<json_value, &'static str> {
    let mut result = String::new();
    result.push(first_char);
    while let Some(c) = chars.next() {
        match c {
            '0'..='9' | '.' | 'e' | 'E' | '+' | '-' => result.push(c),
            _ => {
                if let Ok(number) = result.parse() {
                    return Ok(json_value::Number(number));
                } else {
                    return Err("Invalid number format");
                }
            }
        }
    }
    Err("Unexpected end of input")
}

fn parse_object(chars: &mut std::str::Chars) -> Result<json_value, &'static str> {
    let mut object = HashMap::new();
    loop {
        match chars.next() {
            Some('"') => {
                let key = parse_string(chars)?;
                match chars.next() {
                    Some(':') => {
                        // while let Some(c) = chars.next() {
                        //     if c.is_whitespace() {
                        //         continue;
                        //     } else {
                        //         let value = parse_value(chars)?;
                        //         let key_without_quotes = match key {
                        //             json_value::String(s) => s,
                        //             _ => return Err("Key must be a string"),
                        //         };
                        //         object.insert(key_without_quotes, value);
                        //         break;
                        //     }
                        // }
                        // while let Some(c) = chars.clone().next() {
                        //     if c.is_whitespace() {
                        //         chars.next();
                        //     } else {
                        //         break;
                        //     }
                        // }
                        let value = parse_value(chars)?;
                        let key_without_quotes = match key {
                            json_value::String(s) => s,
                            _ => return Err("Key must be a string"),
                        };
                        object.insert(key_without_quotes, value);
                    }
                    _ => return Err("Expected colon after object key"),
                }
            }
            Some('}') => return Ok(json_value::Object(object)),
            Some(',') => continue,
            _ => return Err("Unexpected character in object"),
        }
    }
}



fn parse_array(chars: &mut std::str::Chars) -> Result<json_value, &'static str> {
    let mut array = Vec::new();
    loop {
        match chars.clone().next() {
            Some(']') => {
                chars.next(); // consume ']'
                return Ok(json_value::Array(array));
            }
            None => return Err("Unexpected end of input"),
            _ => {
                let value = parse_value(chars)?;
                array.push(value);
                match chars.next() {
                    Some(',') => continue,
                    Some(']') => return Ok(json_value::Array(array)),
                    _ => return Err("Unexpected character in array"),
                }
            }
        }
    }
}

fn parse_Bool(first_char: char, chars: &mut std::str::Chars) -> Result<json_value, &'static str> {
    let result = match first_char {
        't' => {
            let rest = chars.clone().take(3).collect::<String>();
            if rest == "rue" {
                chars.nth(2); // consume 'r', 'u', 'e'
                json_value::Bool(true)
            } else {
                return Err("Invalid Bool value");
            }
        }
        'f' => {
            let rest = chars.clone().take(4).collect::<String>();
            if rest == "alse" {
                chars.nth(3); // consume 'a', 'l', 's', 'e'
                json_value::Bool(false)
            } else {
                return Err("Invalid Bool value");
            }
        }
        _ => return Err("Unexpected character in Bool"),
    };
    Ok(result)
}

fn parse_null(chars: &mut std::str::Chars) -> Result<json_value, &'static str> {
    let rest = chars.clone().take(3).collect::<String>();
    if rest == "ull" {
        chars.nth(2); // consume 'u', 'l', 'l'
        Ok(json_value::Null)
    } else {
        Err("Invalid null value")
    }
}
