use crate::{
    logic_gate::LogicGateMode,
    logic_unit::{IO, LogicUnit},
    sn,
    utils::{Id, SignalName},
};

/// tested
pub fn ticker() -> LogicUnit {
    let mut ticker = LogicUnit::new();

    let h = ticker.add_gate(LogicGateMode::AND, false);
    let o = ticker.add_gate_output(LogicGateMode::XOR, false, sn!('i'));
    ticker.io.create_input(sn!('i'), vec![o, h]);
    ticker.connect(h, o);

    ticker
}

// pub fn flip_flop() {

// }

/// I: (A, B, Cin) O: (S, Cout)
/// tested
pub fn full_adder() -> LogicUnit {
    let mut unit = LogicUnit::new();
    let xor1 = unit.add_gate(LogicGateMode::XOR, false);
    let and1 = unit.add_gate(LogicGateMode::AND, false);
    let and2 = unit.add_gate(LogicGateMode::AND, false);
    let xor2 = unit.add_gate_output(LogicGateMode::XOR, false, sn!('S'));
    let or = unit.add_gate_output(LogicGateMode::OR, false, sn!('C'));
    unit.connect(xor1, xor2);
    unit.connect(xor1, and1);
    unit.connect(and1, or);
    unit.connect(and2, or);
    unit.io.create_input(sn!('A'), vec![xor1, and2]);
    unit.io.create_input(sn!('B'), vec![xor1, and2]);
    unit.io.create_input(sn!('C'), vec![xor2, and1]);
    unit
}

/// I: (A{0-7}, B{0-7}, K), O: (O{0-7})
/// K is required!!!
/// tested
pub fn adder_substractor_8b() -> LogicUnit {
    let mut unit = LogicUnit::new();
    let mut cout_buf: Option<u32> = None;
    let mut k_ids: Vec<u32> = Vec::new();
    for idx in 0..8 {
        let io = unit.embed(full_adder());
        let cin = io.get_input(sn!('C'));
        let a = io.get_input(sn!('A'));
        let b = io.get_input(sn!('B'));
        let s = io.get_output(sn!('S')).id;
        let cout = io.get_output(sn!('C')).id;

        let xor = unit.add_gate(LogicGateMode::XOR, false);
        b.ids.iter().for_each(|&id| unit.connect(xor, id));
        unit.io.create_input(sn!('A', idx), a.ids.clone());
        unit.io.create_input(sn!('B', idx), vec![xor]);
        k_ids.push(xor);

        if let Some(v) = cout_buf {
            cin.ids.iter().for_each(|&id| unit.connect(v, id));
        } else {
            k_ids.extend(cin.ids.iter());
        }
        cout_buf = Some(cout);
        unit.io.create_output(sn!('O', idx), s);
    }
    unit.io.create_input(sn!('K'), k_ids);
    unit
}

/// I: (A, B, S) O: O
/// tested
pub fn mux_2_1() -> LogicUnit {
    let mut unit = LogicUnit::new();
    let or = unit.add_gate_output(LogicGateMode::OR, false, sn!('O'));
    let and0 = unit.add_gate(LogicGateMode::AND, false);
    let and1 = unit.add_gate(LogicGateMode::AND, false);
    let not = unit.add_gate(LogicGateMode::NAND, true);
    unit.connect(and0, or);
    unit.connect(and1, or);
    unit.connect(not, and0);
    unit.io.create_input(sn!('I', 0), vec![and0]);
    unit.io.create_input(sn!('I', 1), vec![and1]);
    unit.io.create_input(sn!('S'), vec![and1, not]);
    unit
}

pub fn sr_latch() -> LogicUnit {
    let mut unit = LogicUnit::new();
    let q = unit.add_gate_output(LogicGateMode::NOR, false, sn!('Q'));
    let nq = unit.add_gate(LogicGateMode::NOR, true);
    let h = unit.add_gate(LogicGateMode::AND, false);
    unit.connect(q, nq);
    unit.connect(nq, h);
    unit.connect(h, q);
    unit.io.create_input(sn!('S'), vec![nq]);
    unit.io.create_input(sn!('R'), vec![q]);
    unit
}

/// I: (I{0-3}, S{0-1}) O: O
/// tested
pub fn mux_4_1() -> LogicUnit {
    let mut unit = LogicUnit::new();
    let or = unit.add_gate_output(LogicGateMode::OR, false, sn!('O'));
    let and0 = unit.add_gate(LogicGateMode::AND, false);
    let and1 = unit.add_gate(LogicGateMode::AND, false);
    let and2 = unit.add_gate(LogicGateMode::AND, false);
    let and3 = unit.add_gate(LogicGateMode::AND, false);
    let not0 = unit.add_gate(LogicGateMode::NAND, true);
    let not1 = unit.add_gate(LogicGateMode::NAND, true);
    unit.connect(and0, or);
    unit.connect(and1, or);
    unit.connect(and2, or);
    unit.connect(and3, or);
    unit.connect(not0, and0);
    unit.connect(not0, and1);
    unit.connect(not1, and0);
    unit.connect(not1, and2);
    unit.io.create_input(sn!('I', 0), vec![and0]);
    unit.io.create_input(sn!('I', 1), vec![and1]);
    unit.io.create_input(sn!('I', 2), vec![and2]);
    unit.io.create_input(sn!('I', 3), vec![and3]);
    unit.io.create_input(sn!('S', 0), vec![not1, and1, and3]);
    unit.io.create_input(sn!('S', 1), vec![not0, and2, and3]);
    unit
}
pub fn mux_2n_1(n: usize) -> LogicUnit {
    let mut unit = LogicUnit::new();
    let num_inputs = 1 << n;

    // Output OR gate
    let or = unit.add_gate_output(LogicGateMode::OR, false, sn!('O'));

    // AND gates for each data input
    let mut and_ids = Vec::with_capacity(num_inputs);
    for _ in 0..num_inputs {
        let and = unit.add_gate(LogicGateMode::AND, false);
        and_ids.push(and);
        unit.connect(and, or);
    }

    // NOT gates (inverters) for each select line
    let mut not_ids = Vec::with_capacity(n);
    for _ in 0..n {
        let not = unit.add_gate(LogicGateMode::NAND, true); // inverter
        not_ids.push(not);
    }

    // For each select line, collect gates that need the non‑inverted signal
    let mut select_connections: Vec<Vec<Id>> = vec![Vec::new(); n];
    for j in 0..n {
        select_connections[j].push(not_ids[j]); // the NOT gate itself
    }

    // Connect each AND gate to the appropriate select signals
    for i in 0..num_inputs {
        let and = and_ids[i];
        for j in 0..n {
            if (i >> j) & 1 == 1 {
                // This AND needs the non‑inverted select line j
                select_connections[j].push(and);
            } else {
                // This AND needs the inverted select line j
                unit.connect(not_ids[j], and);
            }
        }
    }

    // Create the select inputs
    for j in 0..n {
        unit.io
            .create_input(sn!('S', j as u8), select_connections[j].clone());
    }

    // Create the data inputs
    for i in 0..num_inputs {
        unit.io.create_input(sn!('I', i as u8), vec![and_ids[i]]);
    }

    unit
}

pub fn decoder_1_2n(n: usize) -> LogicUnit {
    let mut unit = LogicUnit::new();
    let num_outputs = 1 << n;

    // NOT gates for each select line (inverters)
    let mut not_gates = Vec::with_capacity(n);
    for _ in 0..n {
        let not = unit.add_gate(LogicGateMode::NAND, true);
        not_gates.push(not);
    }

    // AND gates for each output – these are the outputs of the decoder
    let mut and_gates = Vec::with_capacity(num_outputs);
    for i in 0..num_outputs {
        let and = unit.add_gate_output(LogicGateMode::AND, false, sn!('O', i as u8));
        and_gates.push(and);
    }

    // For each select line, collect which AND gates need the direct (non‑inverted) signal
    let mut direct_connections: Vec<Vec<Id>> = vec![Vec::new(); n];

    // Connect each AND gate to the appropriate select signals
    for i in 0..num_outputs {
        let and = and_gates[i];
        for j in 0..n {
            if (i >> j) & 1 == 1 {
                // This AND needs the direct select signal
                direct_connections[j].push(and);
            } else {
                // This AND needs the inverted select signal
                unit.connect(not_gates[j], and);
            }
        }
    }

    // Create the select inputs
    for j in 0..n {
        let mut connections = direct_connections[j].clone();
        connections.push(not_gates[j]); // connect the select line to its inverter
        unit.io.create_input(sn!('S', j as u8), connections);
    }

    // Add a single enable input I0, connected to every output AND gate
    unit.io.create_input(sn!('I'), and_gates.clone());

    unit
}
/// modes:
/// 0 - addition
/// 1 - substraction
/// 2 - and
/// 3 - nand
pub fn alu_8b_4m() -> LogicUnit {
    let mut unit = LogicUnit::new();
    let n: usize = 8;
    let muxes: Vec<IO> = (0..n).map(|_| unit.embed(mux_4_1())).collect();
    let arithmetic = unit.embed(adder_substractor_8b());
    let xors: Vec<Id> = (0..n)
        .map(|_| unit.add_gate(LogicGateMode::XOR, false))
        .collect();
    let nands: Vec<Id> = (0..n)
        .map(|_| unit.add_gate(LogicGateMode::NAND, false))
        .collect();

    // opcode
    unit.io
        .create_input(sn!('S', 0), arithmetic.get_input(sn!('K')).ids.clone());
    unit.io.create_input(sn!('S', 1), vec![]);
    for i in 0..n {
        let mut a = arithmetic.get_input(sn!('A', i as u8)).ids.clone();
        a.extend(vec![nands[i], xors[i]]);
        let mut b = arithmetic.get_input(sn!('B', i as u8)).ids.clone();
        b.extend(vec![nands[i], xors[i]]);
        unit.io.create_input(sn!('A', i as u8), a);
        unit.io.create_input(sn!('B', i as u8), b);

        unit.connect_to_input(
            arithmetic.get_output(sn!('O', i as u8)).id,
            muxes[i].get_input(sn!('I', 0)),
        );
        unit.connect_to_input(
            arithmetic.get_output(sn!('O', i as u8)).id,
            muxes[i].get_input(sn!('I', 1)),
        );
        unit.connect_to_input(nands[i], muxes[i].get_input(sn!('I', 2)));
        unit.connect_to_input(xors[i], muxes[i].get_input(sn!('I', 3)));
        unit.io
            .create_output(sn!('O', i as u8), muxes[i].get_output(sn!('O')).id);
        for id in &muxes[i].get_input(sn!('S', 0)).ids {
            unit.io.add_input(sn!('S', 0), *id);
        }
        for id in &muxes[i].get_input(sn!('S', 1)).ids {
            unit.io.add_input(sn!('S', 1), *id);
        }
    }

    unit
}
/// tested
pub fn reg_1b() -> LogicUnit {
    let mut unit = LogicUnit::new();
    let nand1 = unit.add_gate_output(LogicGateMode::NAND, false, sn!('O'));
    let nand2 = unit.add_gate(LogicGateMode::NAND, true);
    let h = unit.add_gate(LogicGateMode::AND, true);
    let nand3 = unit.add_gate(LogicGateMode::NAND, true);
    let nand4 = unit.add_gate(LogicGateMode::NAND, true);
    let not = unit.add_gate(LogicGateMode::NAND, true);
    unit.io.create_input(sn!('D'), vec![nand4, not]);
    unit.io.create_input(sn!('C'), vec![nand3, nand4]);
    unit.connect(nand4, nand1);
    unit.connect(nand3, nand2);
    unit.connect(not, nand3);
    unit.connect(nand1, nand2);
    unit.connect(nand2, h);
    unit.connect(h, nand1);
    unit
}
pub fn reg_8b() -> LogicUnit {
    let mut unit = LogicUnit::new();
    let mut regs: Vec<IO> = (0..8).map(|_| unit.embed(reg_1b())).collect();
    unit.io.create_input(
        sn!('C'),
        regs.iter()
            .map(|r| r.get_input(sn!('C')).ids.clone())
            .flatten()
            .collect(),
    );
    for i in 0..8 {
        unit.io.create_input(
            sn!('D', i),
            regs[i as usize].get_input(sn!('D')).ids.clone(),
        );
        unit.io
            .create_output(sn!('O', i), regs[i as usize].get_output(sn!('O')).id);
    }
    unit
}
