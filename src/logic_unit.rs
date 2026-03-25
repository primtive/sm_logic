use crate::color::Color;
use crate::logic_gate::{LogicGate, LogicGateMode, Switch};
use crate::pos::Pos;
use crate::utils::{Id, SignalName};
use serde_json::Value;

#[derive(Debug)]
pub struct Input {
    pub name: SignalName,
    pub ids: Vec<Id>,
}
pub struct Output {
    pub name: SignalName,
    pub id: Id,
}
pub struct IO {
    inputs: Vec<Input>,
    outputs: Vec<Output>,
}
pub struct LogicUnit {
    pub(crate) gates: Vec<LogicGate>,
    pub io: IO,
}

impl IO {
    fn new() -> Self {
        Self {
            inputs: Vec::new(),
            outputs: Vec::new(),
        }
    }
    pub fn create_input(&mut self, name: SignalName, ids: Vec<Id>) {
        self.inputs.push(Input { name, ids });
    }
    pub fn add_input(&mut self, name: SignalName, id: Id) {
        self.inputs
            .iter_mut()
            .find(|i| i.name == name)
            .unwrap()
            .ids
            .push(id);
    }
    pub fn get_input(&self, name: SignalName) -> &Input {
        self.inputs.iter().find(|i| i.name == name).unwrap()
    }
    pub fn create_output(&mut self, name: SignalName, id: Id) {
        self.outputs.push(Output { name, id });
    }
    pub fn get_output(&self, name: SignalName) -> &Output {
        self.outputs.iter().find(|i| i.name == name).unwrap()
    }
}

impl LogicUnit {
    pub fn new() -> Self {
        Self {
            gates: Vec::new(),
            io: IO::new(),
        }
    }
    pub fn add_gate(&mut self, mode: LogicGateMode, active: bool) -> Id {
        let gate = LogicGate::new(mode, active);
        let id = gate.id;
        self.gates.push(gate);
        id
    }
    pub fn add_gate_output(&mut self, mode: LogicGateMode, active: bool, name: SignalName) -> Id {
        let gate = LogicGate::new(mode, active);
        let id = gate.id;
        self.io.create_output(name, id);
        self.gates.push(gate);
        id
    }
    pub fn connect(&mut self, from_id: Id, to_id: Id) {
        self.gates
            .iter_mut()
            .find(|g| g.id == from_id)
            .unwrap()
            .add_child(to_id);
    }
    pub fn connect_to_input(&mut self, from_id: Id, to: &Input) {
        to.ids.iter().for_each(|&id| {
            self.connect(from_id, id);
        });
    }
    // pub fn connect_io(&mut self, input: &mut Input, output: &Output) {
    //     assert_eq!(input.len(), output.bits.len());
    //     (0..input.bits.len()).for_each(|i| {
    //         input.bits[i]
    //             .iter()
    //             .for_each(|&id| self.connect_gates(id, output.bits[i]));
    //     });
    // }
    pub fn embed(&mut self, mut unit: LogicUnit) -> IO {
        self.gates.append(&mut unit.gates);
        unit.io
    }
    fn get(&mut self, id: u32) -> Option<&LogicGate> {
        self.gates.iter().find(|g| g.id == id)
    }
    fn assemble_single(&self, pos: Pos) -> Vec<Value> {
        self.gates
            .iter()
            .map(|g| g.to_json(&pos, &Color::SINGLE))
            .collect()
    }
    pub fn assemble_io(&mut self, pos: Pos, switches: bool) -> Vec<Value> {
        let mut items = Vec::with_capacity(self.gates.len() + self.io.inputs.len());
        let mut hidden_cnt: u32 = 0;
        let mut chcolor = false;
        println!("start assembling at pos: {pos}");
        let mut o_ids: Vec<u32> = Vec::new();
        // self.io.outputs.iter().unique_by(|o| o.name).collect();

        self.io.outputs.iter().enumerate().for_each(|(idx, o)| {
            // let color = match chcolor {
            //     true => &Color::OUTPUT1,
            //     false => &Color::OUTPUT2,
            // };
            let gate = self.gates.iter().find(|g| g.id == o.id).unwrap();
            println!("o idx:{} name:{} id:{}", idx, o.name, gate.id);
            o_ids.push(o.id);
            items.push(gate.to_json(&pos.add_x(idx as i32 + 1), &Color::OUTPUT1));
            // chcolor = !chcolor;
        });
        chcolor = false;
        self.gates.iter().for_each(|gate| {
            if !o_ids.contains(&gate.id) {
                items.push(gate.to_json(&pos, &Color::SINGLE));
                hidden_cnt += 1;
            }
        });

        // let mut groups: HashMap<char, Vec<Input>> = HashMap::new();
        // for input in &self.io.inputs {
        //     groups
        //         .entry(input.name.0)
        //         .or_insert_with(Vec::new)
        //         .push(*input.clone());
        // }
        // groups.iter().for_each(|group| {
        //     group.1.iter().for_each(|input| {
        //         let gate = LogicGate::new(LogicGateMode::AND, false);
        //         let color = match chcolor {
        //             true => &Color::INPUT1,
        //             false => &Color::INPUT2,
        //         };
        //         if switches {
        //             let mut switch = Switch::new(false);
        //             switch.add_child(gate.id);
        //             items.push(
        //                 switch.to_json(&(Pos::new(-(i as i32 + 1), 0, 1) + pos), &Color::INPUT1),
        //             );
        //         }
        //         println!("i idx:{} name:{} id:{}", i, input.name, gate.id);
        //         let gate_id = gate.id;
        //         self.gates.push(gate);
        //         let ids = input.ids.clone();
        //         ids.iter().for_each(|&id| {
        //             self.connect(gate_id, id);
        //         });
        //         items.push(
        //             self.get(gate_id)
        //                 .unwrap()
        //                 .to_json(&pos.add_x(-(i as i32 + 1)), &Color::INPUT1),
        //         );
        //     });
        // });
        self.io.inputs.sort_by_key(|i| i.name.0);
        for i in 0..self.io.inputs.len() {
            let input = &self.io.inputs[i];
            if i != 0 && input.name.0 != self.io.inputs[i - 1].name.0 {
                chcolor = !chcolor;
            }
            let gate = LogicGate::new(LogicGateMode::AND, false);
            let color = match chcolor {
                true => &Color::INPUT1,
                false => &Color::INPUT2,
            };
            if switches {
                let mut switch = Switch::new(false);
                switch.add_child(gate.id);
                items.push(switch.to_json(&(Pos::new(-(i as i32 + 1), 0, 1) + pos), color));
            }
            println!("i idx:{} name:{} id:{}", i, input.name, gate.id);
            let gate_id = gate.id;
            self.gates.push(gate);
            let ids = input.ids.clone();
            ids.iter().for_each(|&id| {
                self.connect(gate_id, id);
            });
            items.push(
                self.get(gate_id)
                    .unwrap()
                    .to_json(&pos.add_x(-(i as i32 + 1)), color),
            );
        }
        println!("hidden count: {hidden_cnt}");
        items
    }
    // pub fn assemble(&mut self, mode: LogicUnitAssembleMode, pos: Pos) -> Vec<Value> {
    //     match mode {
    //         LogicUnitAssembleMode::Single => self.assemble_single(pos),
    //         LogicUnitAssembleMode::Io => self.assemble_io(pos, true),
    //         LogicUnitAssembleMode::Tree => unimplemented!(),
    //     }
    // }
}
