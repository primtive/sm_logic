use std::{collections::HashMap, io::stdout, process::exit};

use crossterm::{
    cursor::MoveTo,
    event::{EnableMouseCapture, Event, KeyCode, MouseEventKind, read},
    execute,
    terminal::{Clear, enable_raw_mode},
};
use itertools::Itertools;

use crate::{
    logic_unit::{LogicUnit, Output},
    sn,
    utils::{Id, SignalName},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum ParentRef {
    Input(SignalName),
    Gate(Id),
}
pub struct Emulator {
    unit: LogicUnit,
    inputs: Vec<(SignalName, bool, Vec<Id>)>,
    parents: HashMap<Id, Vec<ParentRef>>,
    tick: u64,
}

impl Emulator {
    pub fn new(unit: LogicUnit) -> Self {
        let mut parents: HashMap<Id, Vec<ParentRef>> = HashMap::new();
        for input in &unit.io.inputs {
            for &child in &input.ids {
                parents
                    .entry(child)
                    .or_default()
                    .push(ParentRef::Input(input.name));
            }
        }
        for gate in &unit.gates {
            for &child in &gate.children {
                parents
                    .entry(child)
                    .or_default()
                    .push(ParentRef::Gate(gate.id));
            }
        }
        Self {
            inputs: unit
                .io
                .inputs
                .iter()
                .sorted_by_key(|i| i.name)
                .map(|i| (i.name, false, i.ids.clone()))
                .collect(),
            unit,
            parents,
            tick: 0,
        }
    }
    pub fn tick(&mut self) {
        let mut gates_buf = self.unit.gates.clone();
        // very slow
        // TODO
        (0..gates_buf.len()).for_each(|i| {
            let g = &self.unit.gates[i];
            let v = g.evaluate(
                &self
                    .parents
                    .get(&g.id)
                    .unwrap()
                    .iter()
                    .map(|&r| match r {
                        ParentRef::Gate(id) => {
                            self.unit.gates.iter().find(|&g| g.id == id).unwrap().active
                        }
                        ParentRef::Input(sn) => self.inputs.iter().find(|i| i.0 == sn).unwrap().1,
                    })
                    .collect::<Vec<bool>>(),
            );
            gates_buf[i].active = v;
        });
        self.unit.gates = gates_buf;
        self.tick += 1;
        self.display();
    }
    pub fn display(&self) {
        // disable_raw_mode().unwrap();
        execute!(
            stdout(),
            Clear(crossterm::terminal::ClearType::All),
            MoveTo(0, 0)
        )
        .unwrap();
        print!("Inputs:");
        self.inputs
            .iter()
            .enumerate()
            .for_each(|(i, (sn, active, _))| {
                execute!(stdout(), MoveTo(i as u16 * 3, 1)).unwrap();
                print!("{} ", if *active { "██" } else { "░░" });
                execute!(stdout(), MoveTo(i as u16 * 3, 2)).unwrap();
                print!("{} ", sn);
            });

        execute!(stdout(), MoveTo(0, 3)).unwrap();
        print!("Outputs:");
        self.unit
            .io
            .outputs
            .iter()
            .sorted_by_key(|o| o.name)
            .enumerate()
            .for_each(|(i, o)| {
                execute!(stdout(), MoveTo(i as u16 * 3, 4)).unwrap();
                print!(
                    "{} ",
                    if self
                        .unit
                        .gates
                        .iter()
                        .find(|&g| g.id == o.id)
                        .unwrap()
                        .active
                    {
                        "██"
                    } else {
                        "░░"
                    }
                );
                execute!(stdout(), MoveTo(i as u16 * 3, 5)).unwrap();
                print!("{} ", o.name);
            });
        execute!(stdout(), MoveTo(0, 6)).unwrap();
        println!("tick: {}", self.tick);
        for (i, g) in self.unit.gates.iter().enumerate() {
            execute!(stdout(), MoveTo(0, i as u16 + 7)).unwrap();
            print!(
                "{} {} {}",
                g.id,
                g.active,
                self.unit
                    .io
                    .outputs
                    .iter()
                    .find(|o| o.id == g.id)
                    .unwrap_or(&Output {
                        id: 0,
                        name: sn!('0')
                    })
                    .name
            );
        }
    }
    pub fn enable_mouse_mode() {
        execute!(stdout(), EnableMouseCapture).unwrap();
    }
    pub fn handle_events(&mut self) {
        enable_raw_mode().unwrap();
        if let Event::Mouse(me) = read().unwrap() {
            if let MouseEventKind::Down(_) = me.kind {
                let x = me.column;
                let y = me.row;

                if y == 1 && x / 3 <= self.inputs.len() as u16 {
                    // self.recompute(); // если есть логика
                    self.inputs[x as usize / 3].1 = !self.inputs[x as usize / 3].1;
                    self.display();
                }
            }
        }
        if let Event::Key(key) = read().unwrap() {
            if key.code == KeyCode::Char('q') {
                exit(0);
            } else if key.code == KeyCode::Char('t') {
                self.tick();
            }
        }
    }
}
