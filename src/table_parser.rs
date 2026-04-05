use std::{collections::HashMap, fs, io::Read};

use itertools::Itertools;

use crate::{
    logic::dff,
    logic_gate::{LogicGate, LogicGateMode},
    logic_unit::{IO, Input, LogicUnit, Output},
    sn,
    utils::{Id, SignalName},
};

fn parse_raw_name(map: &HashMap<String, SignalName>, s: &str) -> SignalName {
    let mut raw_name_iter = s.split(' ');
    let s = raw_name_iter.next().unwrap();
    let mut sn = map.get(s).unwrap().clone();
    if let Some(n_str) = raw_name_iter.next() {
        sn.1 = n_str
            .trim_matches(|c| c == '[' || c == ']')
            .parse()
            .unwrap();
    }
    sn
}

impl LogicUnit {
    /// parses single LogicUnit from yosys table format (write_table)
    pub fn from_table(path: &str) -> Self {
        let mut table = String::new();
        fs::File::open(path)
            .unwrap()
            .read_to_string(&mut table)
            .unwrap();

        let mut unit = Self {
            gates: Vec::new(),
            io: IO::new(),
        };

        let mut inputs: HashMap<String, SignalName> = HashMap::new();
        let mut outputs: HashMap<String, SignalName> = HashMap::new();

        let mut wires: HashMap<String, (Id, Vec<Id>)> = HashMap::new();
        for (k, section) in &table
            .split('\n')
            .filter_map(|r| {
                println!("{}", r);
                if r.len() == 0 {
                    None
                } else {
                    Some(r.split('\t').collect::<Vec<_>>())
                }
            })
            .chunk_by(|r| r[1].starts_with('$'))
        {
            if !k {
                // headers
                for header in section {
                    if header[5].starts_with('{') {
                        header[5]
                            .trim_matches(|c| c == '{' || c == '}')
                            .split_whitespace()
                            .rev()
                            .enumerate()
                            .for_each(|(n, name)| {
                                let c = name.split('.').last().unwrap().chars().nth(0).unwrap();
                                let sn = sn!(c, n as u8);
                                match header[4] {
                                    "pi" => inputs.insert(name.to_string(), sn),
                                    "po" => outputs.insert(name.to_string(), sn),
                                    _ => panic!("invalid table"),
                                };
                            });
                    } else {
                        match header[4] {
                            "pi" => {
                                inputs.insert(
                                    header[5].to_string(),
                                    sn!(header[5].chars().nth(1).unwrap()),
                                );
                            }
                            "po" => {
                                outputs.insert(
                                    header[5].to_string(),
                                    sn!(header[5].chars().nth(1).unwrap()),
                                );
                            }
                            _ => panic!("invalid table"),
                        }
                    }
                }
            } else {
                // body
                for (_, chunk) in &section.chunk_by(|r| r[1]) {
                    // record format:
                    // module_name   port_name   gate_type   pin_name   pin_type   connect_to
                    let records = chunk.collect::<Vec<Vec<_>>>();
                    let mut io: IO = IO::new();

                    for (i, record) in records.iter().enumerate() {
                        if i == 0 {
                            if record[2] == "$_DFF_P_" {
                                io = unit.embed(dff());
                            } else {
                                let mode = match record[2] {
                                    "$_AND_" => LogicGateMode::And,
                                    "$_OR_" => LogicGateMode::Or,
                                    "$_NAND_" | "$_NOT_" => LogicGateMode::Nand,
                                    "$_NOR_" => LogicGateMode::Nor,
                                    "$_XOR_" => LogicGateMode::Xor,
                                    "$_XNOR_" => LogicGateMode::Xnor,
                                    _ => panic!("incorrect gate mode: {}", record[2]),
                                };
                                io = unit.embed_gate(LogicGate::new(mode, false));
                            }
                        }

                        let pin = sn!(record[3].chars().nth(0).unwrap());

                        match record[4] {
                            "in" => {
                                let ids = &mut io.get_input(pin).ids.clone();
                                if !inputs
                                    .contains_key(record[5].split_whitespace().nth(0).unwrap())
                                {
                                    wires
                                        .entry(record[5].to_string())
                                        .or_default()
                                        .1
                                        .append(ids)
                                } else {
                                    let sn = parse_raw_name(&inputs, &record[5]);
                                    if let Some(i) =
                                        unit.io.inputs.iter_mut().find(|i| i.name == sn)
                                    {
                                        i.ids.append(&mut io.get_input(pin).ids.clone());
                                    } else {
                                        unit.io.inputs.push(Input {
                                            name: sn,
                                            ids: ids.clone(),
                                        });
                                    }
                                }
                            }
                            "out" => {
                                let id = io.get_output(pin).id;
                                if !outputs
                                    .contains_key(record[5].split_whitespace().nth(0).unwrap())
                                {
                                    wires.entry(record[5].to_string()).or_default().0 = id;
                                } else {
                                    let sn = parse_raw_name(&outputs, &record[5]);
                                    if let Some(o) =
                                        unit.io.outputs.iter_mut().find(|i| i.name == sn)
                                    {
                                        o.id = id;
                                    } else {
                                        unit.io.outputs.push(Output { name: sn, id: id });
                                    }
                                }
                            }
                            _ => panic!("incorrect pin mode"),
                        }
                    }
                }
            }
        }
        dbg!(&wires);
        for (name, wire) in wires {
            if let Some(sn) = outputs.get(name.split_whitespace().nth(0).unwrap()) {
                for to_id in &wire.1 {
                    unit.connect(unit.io.get_output(*sn).id, *to_id);
                }
            } else {
                for to_id in &wire.1 {
                    unit.connect(wire.0, *to_id);
                }
            }
        }
        unit
    }
}
