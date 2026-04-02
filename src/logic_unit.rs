use crate::color::Color;
use crate::logic_gate::{LogicGate, LogicGateMode, Switch};
use crate::pos::Pos;
use crate::utils::{Id, SignalName};
use serde_json::Value;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
pub struct Input {
    pub name: SignalName,
    pub ids: Vec<Id>,
}
#[derive(Clone)]
pub struct Output {
    pub name: SignalName,
    pub id: Id,
}
pub struct IO {
    pub inputs: Vec<Input>,
    pub outputs: Vec<Output>,
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
    pub fn add_inputs(&mut self, name: SignalName, ids: &[Id]) {
        ids.iter().for_each(|&id| {
            self.inputs
                .iter_mut()
                .find(|i| i.name == name)
                .unwrap()
                .ids
                .push(id);
        });
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
    pub fn assemble_display(&mut self, pos: Pos, w: usize) -> Vec<Value> {
        let mut items = Vec::with_capacity(self.gates.len() + self.io.inputs.len());
        println!("start assembling at pos: {pos}");
        let mut o_ids: Vec<u32> = Vec::new();

        self.io.outputs.sort_by_key(|o| o.name);
        self.io.outputs.iter().enumerate().for_each(|(idx, o)| {
            if o.name.0 == 'd' {
                let row = idx % w;
                let col = idx / w;
                println!("d r:{row} c:{col} name:{} id:{}", o.name, o.id);
                let gate = self.gates.iter().find(|g| g.id == o.id).unwrap();
                o_ids.push(o.id);
                items.push(gate.to_json(
                    &(pos + Pos::new(row as i32, -(col as i32), 0)),
                    &Color::DISPLAY,
                ));
            }
        });
        self.gates.iter().for_each(|gate| {
            if !o_ids.contains(&gate.id) {
                items.push(gate.to_json(&(pos + Pos::new(0, 0, -1)), &Color::SINGLE));
            }
        });

        items
    }
    pub fn assemble_io(&mut self, pos: Pos, switches: bool) -> Vec<Value> {
        let mut items = Vec::with_capacity(self.gates.len() + self.io.inputs.len());
        let mut hidden_cnt: u32 = 0;
        let mut chcolor = false;
        println!("start assembling at pos: {pos}");
        let mut o_ids: Vec<u32> = Vec::new();
        // self.io.outputs.iter().unique_by(|o| o.name).collect();

        self.io.outputs.sort_by_key(|o| o.name.0);
        self.io.outputs.iter().enumerate().for_each(|(idx, o)| {
            if idx != 0 && o.name.0 != self.io.outputs[idx - 1].name.0 {
                chcolor = !chcolor;
            }
            let color = match chcolor {
                true => &Color::OUTPUT1,
                false => &Color::OUTPUT2,
            };
            let gate = self.gates.iter().find(|g| g.id == o.id).unwrap();
            println!("o idx:{} name:{} id:{}", idx, o.name, gate.id);
            o_ids.push(o.id);
            items.push(gate.to_json(&pos.add_x(idx as i32 + 1), color));
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

    /// Собирает схему как "дерево" от `outputs` к `inputs`:
    /// - корни: гейты, которые объявлены как `io.outputs`
    /// - глубина: расстояние вверх по графу (через обратные связи)
    /// - входы: для каждого `io.inputs` создаётся фиктивный входной AND-гейт,
    ///   который управляет всеми гейтами из `input.ids`
    pub fn assemble_tree(&mut self, pos: Pos, switches: bool) -> Vec<Value> {
        // Быстрая индексация существующих гейтов.
        let mut id_to_gate: HashMap<Id, &LogicGate> = HashMap::new();
        for gate in &self.gates {
            id_to_gate.insert(gate.id, gate);
        }

        let output_ids: Vec<Id> = self.io.outputs.iter().map(|o| o.id).collect();
        let output_set: HashSet<Id> = output_ids.iter().copied().collect();

        // Обратные связи: parents[x] = все гейты p, которые подключены в x как child.
        let mut parents: HashMap<Id, Vec<Id>> = HashMap::new();
        for gate in &self.gates {
            for &child_id in &gate.children {
                parents.entry(child_id).or_default().push(gate.id);
            }
        }

        // BFS вверх от outputs по обратным связям.
        let mut depth: HashMap<Id, u32> = HashMap::new();
        let mut q: VecDeque<Id> = VecDeque::new();
        for &oid in &output_ids {
            depth.insert(oid, 0);
            q.push_back(oid);
        }

        while let Some(cur) = q.pop_front() {
            let d = *depth.get(&cur).unwrap_or(&0);
            if let Some(ps) = parents.get(&cur) {
                for &p in ps {
                    let nd = d + 1;
                    let should_enqueue = match depth.get(&p) {
                        Some(&old_d) => nd < old_d,
                        None => true,
                    };
                    if should_enqueue {
                        depth.insert(p, nd);
                        q.push_back(p);
                    }
                }
            }
        }

        let max_depth = depth.values().copied().max().unwrap_or(0);

        // Раскладываем по слоям.
        let mut by_depth: HashMap<u32, Vec<Id>> = HashMap::new();
        for (id, d) in &depth {
            by_depth.entry(*d).or_default().push(*id);
        }
        for v in by_depth.values_mut() {
            v.sort_unstable();
        }

        let mut items = Vec::with_capacity(self.gates.len() + self.io.inputs.len());

        // Выводим достижимые гейты.
        for d in 0..=max_depth {
            if let Some(ids) = by_depth.get(&d) {
                for (idx, &id) in ids.iter().enumerate() {
                    let gate = id_to_gate.get(&id).expect("gate id should exist");
                    let color = if output_set.contains(&id) {
                        &Color::OUTPUT1
                    } else {
                        &Color::SINGLE
                    };
                    let node_pos = Pos::new(pos.x + 1 - d as i32, pos.y + (idx as i32) * 2, pos.z);
                    items.push(gate.to_json(&node_pos, color));
                }
            }
        }

        // Любые "лишние" гейты (которые не влияют на outputs) выводим справа.
        let mut unreachable: Vec<Id> = Vec::new();
        for gate in &self.gates {
            if !depth.contains_key(&gate.id) {
                unreachable.push(gate.id);
            }
        }
        unreachable.sort_unstable();
        for (idx, id) in unreachable.iter().enumerate() {
            let gate = id_to_gate
                .get(id)
                .expect("unreachable gate id should exist");
            let node_pos = Pos::new(pos.x + 1 + 2, pos.y + (idx as i32) * 2, pos.z);
            items.push(gate.to_json(&node_pos, &Color::SINGLE));
        }

        // Входы: создаём фиктивные AND-гейты, которые контролируют ids входа.
        // Порядок входов делаем предсказуемым (как и в `assemble_io`).
        self.io.inputs.sort_by_key(|i| i.name.0);
        let input_depth = max_depth + 1;
        let input_x = pos.x + 1 - input_depth as i32;

        let mut chcolor = false;
        for i in 0..self.io.inputs.len() {
            let input = &self.io.inputs[i];
            if i != 0 && input.name.0 != self.io.inputs[i - 1].name.0 {
                chcolor = !chcolor;
            }
            let color = if chcolor {
                &Color::INPUT1
            } else {
                &Color::INPUT2
            };

            let mut in_gate = LogicGate::new(LogicGateMode::AND, false);
            for &target_id in &input.ids {
                in_gate.add_child(target_id);
            }

            let in_y = pos.y + (i as i32) * 2;
            let in_pos = Pos::new(input_x, in_y, pos.z);
            if switches {
                let mut sw = Switch::new(false);
                sw.add_child(in_gate.id);
                let sw_pos = Pos::new(input_x, in_y, pos.z + 1);
                items.push(sw.to_json(&sw_pos, color));
            }

            items.push(in_gate.to_json(&in_pos, color));
        }

        items
    }
}
