use serde_json::{Value, json};

pub struct Blueprint {
    bodies: Vec<Value>,
}

impl Blueprint {
    pub fn new() -> Self {
        Self { bodies: Vec::new() }
    }
    pub fn place(&mut self, mut bodies: Vec<Value>) {
        self.bodies.append(&mut bodies);
    }
    pub fn to_json(&self) -> Value {
        println!("converting blueprint to json, {} bodies", self.bodies.len());
        json!({
            "version": 4,
            "bodies": [
                {
                    "childs": self.bodies
                }
            ]
        })
    }
}
