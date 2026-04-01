use serde_json::{Value, json};

use crate::color::Color;
use crate::pos::Pos;
use crate::utils::Id;

pub fn new_controller_id() -> Id {
    static mut ID: Id = 0;
    unsafe {
        // dbg!(ID);
        ID += 1;
        ID
    }
}
#[derive(Clone)]
pub struct LogicGate {
    pub id: Id,
    pub active: bool,
    pub mode: LogicGateMode,
    pub children: Vec<Id>,
}
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum LogicGateMode {
    AND,
    OR,
    XOR,
    NAND,
    NOR,
    XNOR,
}
impl LogicGate {
    const SHAPEID: &str = "9f0f56e8-2c31-4d83-996c-d00a9b296c3f";
    pub fn new(mode: LogicGateMode, active: bool) -> Self {
        Self {
            id: new_controller_id(),
            mode,
            active,
            children: Vec::new(),
        }
    }
    pub fn add_child(&mut self, id: Id) {
        self.children.push(id);
    }
    pub fn to_json(&self, pos: &Pos, color: &Color) -> Value {
        json!({
            "color": color.to_string(),
            "controller": {
              "active": self.active,
              "controllers": self.children.iter().map(|id| json!({"id": id})).collect::<Vec<Value>>(),
              "id": self.id,
              "joints": null,
              "mode": self.mode as u8,
            },
            "pos": pos.to_json(),
            "shapeId": Self::SHAPEID,
            "xaxis": 1,
            "zaxis": -2
        })
    }
    pub fn evaluate(&self, parents: &[bool]) -> bool {
        if parents.is_empty() {
            return false;
        }
        match self.mode {
            LogicGateMode::AND => parents.iter().all(|&x| x),
            LogicGateMode::OR => parents.iter().any(|&x| x),
            LogicGateMode::XOR => parents.iter().fold(false, |acc, &x| acc ^ x),
            LogicGateMode::NAND => !parents.iter().all(|&x| x),
            LogicGateMode::NOR => !parents.iter().any(|&x| x),
            LogicGateMode::XNOR => !parents.iter().fold(false, |acc, &x| acc ^ x),
        }
    }
}

pub struct Switch {
    active: bool,
    id: Id,
    children: Vec<Id>,
}

impl Switch {
    const SHAPEID: &str = "7cf717d7-d167-4f2d-a6e7-6b2c70aa3986";
    pub fn new(active: bool) -> Self {
        Self {
            id: new_controller_id(),
            active,
            children: Vec::new(),
        }
    }
    pub fn add_child(&mut self, id: Id) {
        self.children.push(id);
    }
    pub fn to_json(&self, pos: &Pos, color: &Color) -> Value {
        json!({
            "color": color.to_string(),
            "controller": {
              "active": self.active,
              "controllers": self.children.iter().map(|id| json!({"id": id})).collect::<Vec<Value>>(),
              "id": self.id,
              "joints": null,
            },
            "pos": pos.to_json(),
            "shapeId": Self::SHAPEID,
            "xaxis": 1,
            "zaxis": -2
        })
    }
}
