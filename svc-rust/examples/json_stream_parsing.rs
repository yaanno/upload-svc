use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::Cursor;

#[derive(Debug, Serialize, Deserialize)]
struct Event {
    actor: Actor,
    event: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Actor {
    name: String,
    age: u32,
}

fn main() {
    // Simulate a JSON stream with partial objects
    let json_stream = r#"{"actor": {"name": "John", "age": 30}, "event": "login"}
{"actor": {"name": "Jane", "age": 25}, "event": "register"}
{"actor": {"name": "Bob", "age": 45}, "event": "logout"}"#;

    println!("Parsing stream with serde_json::Value:");
    let cursor = Cursor::new(json_stream);
    let stream = serde_json::Deserializer::from_reader(cursor)
        .into_iter::<Value>();

    for (index, record) in stream.enumerate() {
        match record {
            Ok(value) => {
                println!("Record {}: {:?}", index + 1, value);
                
                // Demonstrate extracting specific fields
                if let (Some(actor), Some(event)) = 
                    (value.get("actor"), value.get("event")) {
                    println!("  Actor Name: {}", 
                        actor.get("name").unwrap_or(&Value::Null));
                    println!("  Event: {}", 
                        event.as_str().unwrap_or("Unknown"));
                }
            }
            Err(e) => {
                eprintln!("Parsing error: {}", e);
            }
        }
    }

    println!("\nParsing stream with strongly-typed Event:");
    let cursor = Cursor::new(json_stream);
    let stream = serde_json::Deserializer::from_reader(cursor)
        .into_iter::<Event>();

    for (index, record) in stream.enumerate() {
        match record {
            Ok(event) => {
                println!("Record {}: {:?}", index + 1, event);
            }
            Err(e) => {
                eprintln!("Parsing error: {}", e);
            }
        }
    }
}
