use std::collections::HashMap;

use super::lib::json_value;

#[derive(Debug, Clone)]
struct Sample {
    id: u32,
    name: String,
}

struct SampleHashMap(HashMap<String, json_value>);


impl From<Sample> for HashMap<String, json_value> {
    fn from(sample: Sample) -> Self {
        let mut map = HashMap::new();
        map.insert("id".to_string(), json_value::Number(sample.id as f64));
        map.insert("name".to_string(), json_value::String(sample.name));
        map
    }
}

impl From<SampleHashMap> for Sample {
    fn from(wrapper : SampleHashMap) -> Self {
        let map = wrapper.0;
        let id = map.get("id")
            .and_then(|value| {
                match value {
                    json_value::Number(n) => Some(*n as u32),
                    _ =>  None
                }
            }).unwrap_or_default();

        let name = map.get("name").and_then(|value| {
            match value {
                json_value::String(val) => Some(val.to_owned()),
                _ => None 
            }
        }).unwrap_or_default();
        
        Sample{id, name}
    }
}



#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::serializer_deserializer::lib::*;
    use self::thread::ThreadPool;

    use super::*;
    use crate::pool::*;
    use std::{thread::sleep, time::Duration};
    // #[test]
    // fn test_with_sample_data() {
    //     println!("into the  tests");
    //     let data = Sample{id: 1, name: "raja".to_string()};
    //     println!("sample created : {:?}", data);
    //     let data_value = HashMap::from(data.clone());
    //     println!("data_value: {:?}", data_value.clone());
    //     let json_obj = json_value::Object(data_value.clone());
    //     println!("json_obj  : {:?}", json_obj);
    //     let json_string  = serialize_json(&json_obj);
    //     print!("string is {:?}\n", json_string);
    //     if let Ok(retrieved_json) = deserialize_json(&json_string) {
    //         print!("json  : {:?}\n", retrieved_json.clone());
    //         match retrieved_json {
    //             json_value::Object(r_map) => {
    //                 let map_wrapper = SampleHashMap(r_map);
    //                 let s = Sample::from(map_wrapper);
    //                 print!("sample : {:?}\n", s)
    //             }
    //             _ => print!("value not found")
    //         }
    //     } else if let Err(e) = deserialize_json(&json_string) {
    //         print!("error : {:?}\n", e)
    //     }
        
    //     assert!(!json_string.is_empty())
    // }
    // #[test]
    // fn test_thread_pool() {
    //     let t_pool = ThreadPool::new(4);
    //     for _ in 0..5 {
    //         println!("hello there, none can stop me");
    //         t_pool.execute(|| {
    //             sleep(Duration::from_secs(1));
    //             println!("i am here")
    //         })
    //     }      
    // }
} 