use crate::{
    logic_gate::LogicGateMode,
    logic_unit::{IO, LogicUnit},
    sn,
    utils::{Id, SignalName},
};

/// tested
pub fn ticker() -> LogicUnit {
    let mut ticker = LogicUnit::new();

    let h = ticker.add_gate(LogicGateMode::And, false);
    let o = ticker.add_gate_output(LogicGateMode::Xor, false, sn!('i'));
    ticker.io.create_input(sn!('i'), vec![o, h]);
    ticker.connect(h, o);

    ticker
}

/// D trigger
pub fn dff() -> LogicUnit {
    let mut unit = LogicUnit::new();
    let nand1 = unit.add_gate_output(LogicGateMode::Nand, false, sn!('Q'));
    let nand2 = unit.add_gate(LogicGateMode::Nand, false);
    let nand3 = unit.add_gate(LogicGateMode::Nand, false);
    let nand4 = unit.add_gate(LogicGateMode::Nand, false);
    let h = unit.add_gate(LogicGateMode::And, false);
    let not = unit.add_gate(LogicGateMode::Nand, true);
    unit.connect(nand1, h);
    unit.connect(h, nand2);
    unit.connect(nand2, nand1);
    unit.connect(nand3, nand1);
    unit.connect(nand4, nand2);
    unit.connect(not, nand4);
    unit.io.create_input(sn!('D'), vec![nand3, not]);
    unit.io.create_input(sn!('C'), vec![nand3, nand4]);
    unit
}
/// D trigger, 1 tick CLK (for SM)
/// F for flip (1 tick only)
pub fn dff_1tick() -> LogicUnit {
    let mut unit = LogicUnit::new();
    let bit = unit.add_gate_output(LogicGateMode::Xor, false, sn!('Q'));
    let xor = unit.add_gate(LogicGateMode::Xor, false);
    let and = unit.add_gate(LogicGateMode::And, false);
    unit.connect(bit, bit);
    unit.connect(bit, xor);
    unit.connect(xor, and);
    unit.connect(and, bit);
    unit.io.create_input(sn!('D'), vec![xor]);
    unit.io.create_input(sn!('C'), vec![and]);
    unit.io.create_input(sn!('F'), vec![bit]);
    unit
}

/// I: (A, B, Cin) O: (S, Cout)
/// tested
pub fn full_adder() -> LogicUnit {
    let mut unit = LogicUnit::new();
    let xor1 = unit.add_gate(LogicGateMode::Xor, false);
    let and1 = unit.add_gate(LogicGateMode::And, false);
    let and2 = unit.add_gate(LogicGateMode::And, false);
    let xor2 = unit.add_gate_output(LogicGateMode::Xor, false, sn!('S'));
    let or = unit.add_gate_output(LogicGateMode::Or, false, sn!('C'));
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

        let xor = unit.add_gate(LogicGateMode::Xor, false);
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
    let or = unit.add_gate_output(LogicGateMode::Or, false, sn!('O'));
    let and0 = unit.add_gate(LogicGateMode::And, false);
    let and1 = unit.add_gate(LogicGateMode::And, false);
    let not = unit.add_gate(LogicGateMode::Nand, true);
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
    let q = unit.add_gate_output(LogicGateMode::Nor, false, sn!('Q'));
    let nq = unit.add_gate(LogicGateMode::Nor, true);
    let h = unit.add_gate(LogicGateMode::And, false);
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
    let or = unit.add_gate_output(LogicGateMode::Or, false, sn!('O'));
    let and0 = unit.add_gate(LogicGateMode::And, false);
    let and1 = unit.add_gate(LogicGateMode::And, false);
    let and2 = unit.add_gate(LogicGateMode::And, false);
    let and3 = unit.add_gate(LogicGateMode::And, false);
    let not0 = unit.add_gate(LogicGateMode::Nand, true);
    let not1 = unit.add_gate(LogicGateMode::Nand, true);
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
    let or = unit.add_gate_output(LogicGateMode::Or, false, sn!('O'));

    // AND gates for each data input
    let mut and_ids = Vec::with_capacity(num_inputs);
    for _ in 0..num_inputs {
        let and = unit.add_gate(LogicGateMode::And, false);
        and_ids.push(and);
        unit.connect(and, or);
    }

    // NOT gates (inverters) for each select line
    let mut not_ids = Vec::with_capacity(n);
    for _ in 0..n {
        let not = unit.add_gate(LogicGateMode::Nand, true); // inverter
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
        let not = unit.add_gate(LogicGateMode::Nand, true);
        not_gates.push(not);
    }

    // AND gates for each output – these are the outputs of the decoder
    let mut and_gates = Vec::with_capacity(num_outputs);
    for i in 0..num_outputs {
        let and = unit.add_gate_output(LogicGateMode::And, false, sn!('O', i as u8));
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
        .map(|_| unit.add_gate(LogicGateMode::Xor, false))
        .collect();
    let nands: Vec<Id> = (0..n)
        .map(|_| unit.add_gate(LogicGateMode::Nand, false))
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
        unit.io
            .add_inputs(sn!('S', 0), &muxes[i].get_input(sn!('S', 0)).ids);
        unit.io
            .add_inputs(sn!('S', 1), &muxes[i].get_input(sn!('S', 1)).ids);
    }

    unit
}
/// tested
/// rs trigger
pub fn reg_1b() -> LogicUnit {
    let mut unit = LogicUnit::new();
    let nand1 = unit.add_gate_output(LogicGateMode::Nand, false, sn!('O'));
    let nand2 = unit.add_gate(LogicGateMode::Nand, true);
    let h1 = unit.add_gate(LogicGateMode::And, true);
    let h2 = unit.add_gate(LogicGateMode::And, true);
    let nand3 = unit.add_gate(LogicGateMode::Nand, true);
    let nand4 = unit.add_gate(LogicGateMode::Nand, true);
    let not = unit.add_gate(LogicGateMode::Nand, true);
    unit.io.create_input(sn!('I'), vec![nand4, not]);
    unit.io.create_input(sn!('W'), vec![nand3, nand4]);
    unit.connect(nand4, nand1);
    unit.connect(nand3, nand2);
    unit.connect(not, nand3);
    unit.connect(nand2, h1);
    unit.connect(h1, nand1);
    unit.connect(nand1, h2);
    unit.connect(h2, nand2);
    unit
}
pub fn reg_8b() -> LogicUnit {
    let mut unit = LogicUnit::new();
    let regs: Vec<IO> = (0..8).map(|_| unit.embed(reg_1b())).collect();
    unit.io.create_input(
        sn!('W'),
        regs.iter()
            .map(|r| r.get_input(sn!('W')).ids.clone())
            .flatten()
            .collect(),
    );
    for i in 0..8 {
        unit.io.create_input(
            sn!('I', i),
            regs[i as usize].get_input(sn!('I')).ids.clone(),
        );
        unit.io
            .create_output(sn!('O', i), regs[i as usize].get_output(sn!('O')).id);
    }
    unit
}

/// TODO: R signal
/// tested
pub fn reg_bank_8b_8x() -> LogicUnit {
    let mut unit = LogicUnit::new();
    let muxes: Vec<IO> = (0..8).map(|_| unit.embed(mux_2n_1(3))).collect();
    println!("muxes");
    let dec = unit.embed(decoder_1_2n(3));
    let regs: Vec<IO> = (0..8).map(|_| unit.embed(reg_8b())).collect();

    // W signal
    unit.io
        .create_input(sn!('W'), dec.get_input(sn!('I')).ids.clone());
    // WriteReg
    unit.io
        .create_input(sn!('S', 0), dec.get_input(sn!('S', 0)).ids.clone());
    unit.io
        .create_input(sn!('S', 1), dec.get_input(sn!('S', 1)).ids.clone());
    unit.io
        .create_input(sn!('S', 2), dec.get_input(sn!('S', 2)).ids.clone());
    // ReadReg
    unit.io.create_input(sn!('s', 0), vec![]);
    unit.io.create_input(sn!('s', 1), vec![]);
    unit.io.create_input(sn!('s', 2), vec![]);
    // CLK
    // unit.io.create_input(sn!('C'), vec![]);

    for i in 0..8 {
        unit.io.create_input(sn!('I', i), vec![]);

        // ReadReg
        unit.io
            .add_inputs(sn!('s', 0), &muxes[i as usize].get_input(sn!('S', 0)).ids);
        unit.io
            .add_inputs(sn!('s', 1), &muxes[i as usize].get_input(sn!('S', 1)).ids);
        unit.io
            .add_inputs(sn!('s', 2), &muxes[i as usize].get_input(sn!('S', 2)).ids);
        // Output
        unit.io
            .create_output(sn!('O', i), muxes[i as usize].get_output(sn!('O')).id);
    }
    for reg_idx in 0..8 {
        for i in 0..8 {
            // input to regs
            unit.io
                .add_inputs(sn!('I', i), &regs[reg_idx].get_input(sn!('I', i)).ids);
            // regs to mux
            unit.connect_to_input(
                regs[reg_idx].get_output(sn!('O', i)).id,
                muxes[i as usize].get_input(sn!('I', reg_idx as u8)),
            );
        }
        // CLK
        // unit.io
        //     .add_inputs(sn!('C'), &regs[reg_idx].get_input(sn!('C')).ids);

        // dec to regs
        unit.connect_to_input(
            dec.get_output(sn!('O', reg_idx as u8)).id,
            regs[reg_idx].get_input(sn!('W')),
        );
    }
    unit
}

pub fn rom_4kb(data: &[u32]) -> LogicUnit {
    let mut unit = LogicUnit::new();
    let col_dec = unit.embed(decoder_1_2n(5));
    let row_dec = unit.embed(decoder_1_2n(5));
    for i in 2..12 {
        // addr
        if i > 6 {
            unit.io
                .create_input(sn!('A', i), col_dec.get_input(sn!('S', i - 7)).ids.clone());
        } else {
            unit.io
                .create_input(sn!('A', i), row_dec.get_input(sn!('S', i - 2)).ids.clone());
        }
    }
    unit.io
        .create_input(sn!('R'), col_dec.get_input(sn!('I')).ids.clone());
    unit.io
        .add_inputs(sn!('R'), &row_dec.get_input(sn!('I')).ids.clone());
    let output_ids: Vec<Id> = (0..32)
        .map(|i| unit.add_gate_output(LogicGateMode::Or, false, sn!('O', i)))
        .collect();
    for i in 0..1024 {
        let col = i / 32;
        let row = i % 32;
        let value = data.get(i).unwrap_or(&0);
        let and = unit.add_gate(LogicGateMode::And, false);
        unit.connect(col_dec.get_output(sn!('O', col as u8)).id, and);
        unit.connect(row_dec.get_output(sn!('O', row as u8)).id, and);
        (0..32).for_each(|i| {
            let bit = (value >> i) & 1 == 1;
            if bit {
                unit.connect(and, output_ids[i]);
            }
        });
    }
    unit
}

pub fn digit_dispay() -> LogicUnit {
    let mut unit = LogicUnit::new();
    #[rustfmt::skip]
    const DIGITS: [[u8; 15]; 10] = [
      [
          1, 1, 1,
          1, 0, 1,
          1, 0, 1,
          1, 0, 1,
          1, 1, 1
      ],
      [
          0, 1, 0,
          1, 1, 0,
          0, 1, 0,
          0, 1, 0,
          0, 1, 0
      ],
      [
          0, 1, 0,
          1, 0, 1,
          0, 0, 1,
          0, 1, 0,
          1, 1, 1
      ],
      [
          1, 1, 0,
          0, 0, 1,
          0, 1, 0,
          0, 0, 1,
          1, 1, 0,
      ],
      [
          1, 0, 1,
          1, 0, 1,
          1, 1, 1,
          0, 0, 1,
          0, 0, 1
      ],
      [
          1, 1, 1,
          1, 0, 0,
          1, 1, 1,
          0, 0, 1,
          1, 1, 0
      ],
      [
          1, 1, 1,
          1, 0, 0,
          1, 1, 1,
          1, 0, 1,
          1, 1, 1
      ],
      [
          1, 1, 1,
          0, 0, 1,
          0, 1, 0,
          0, 1, 0,
          0, 1, 0
      ],
      [
          1, 1, 1,
          1, 0, 1,
          0, 1, 2,
          1, 0, 1,
          1, 1, 1
      ],
      [
          1, 1, 1,
          1, 0, 1,
          1, 1, 1,
          0, 0, 1,
          1, 1, 1
      ],
    ];
    let output_ids: Vec<Id> = (0..15)
        .map(|i| unit.add_gate_output(LogicGateMode::Or, false, sn!('d', i)))
        .collect();
    let dec = unit.embed(decoder_1_2n(4));
    unit.io
        .create_input(sn!('I'), dec.get_input(sn!('I')).ids.clone());
    for i in 0..4 {
        let t = unit.add_gate(LogicGateMode::And, false);
        unit.io.create_input(sn!('S', i), vec![t]);
        unit.connect_to_input(t, dec.get_input(sn!('S', i)));
    }
    for (n, d) in DIGITS.iter().enumerate() {
        for i in 0..15 {
            if d[i] == 1 {
                unit.connect(dec.get_output(sn!('O', n as u8)).id, output_ids[i]);
            }
        }
    }
    unit
}

/// tested
fn cond_plus_3() -> LogicUnit {
    let mut unit = LogicUnit::new();
    let or1 = unit.add_gate(LogicGateMode::Or, false);
    let and1 = unit.add_gate(LogicGateMode::And, false);
    let or2 = unit.add_gate(LogicGateMode::Or, false);
    let xor1 = unit.add_gate_output(LogicGateMode::Xor, false, sn!('Q', 0));
    let and2 = unit.add_gate(LogicGateMode::And, false);
    let not = unit.add_gate(LogicGateMode::Nand, true);
    let and3 = unit.add_gate(LogicGateMode::And, false);
    let xor2 = unit.add_gate(LogicGateMode::Xor, false);
    let ha_c = unit.add_gate(LogicGateMode::And, false);
    let ha_s = unit.add_gate_output(LogicGateMode::Xor, false, sn!('Q', 2));
    let xor3 = unit.add_gate_output(LogicGateMode::Xor, false, sn!('Q', 1));
    let xor4 = unit.add_gate_output(LogicGateMode::Xor, false, sn!('Q', 3));
    unit.io.create_input(sn!('A', 0), vec![or1, xor1]);
    unit.io.create_input(sn!('A', 1), vec![or1, not, xor3]);
    unit.io.create_input(sn!('A', 2), vec![and1, ha_c, ha_s]);
    unit.io.create_input(sn!('A', 3), vec![or2, xor4]);
    unit.connect(or1, and1);
    unit.connect(and1, or2);
    unit.connect(or2, xor1);
    unit.connect(or2, xor2);
    unit.connect(or2, and2);
    unit.connect(xor1, and2);
    unit.connect(and2, xor3);
    unit.connect(and2, and3);
    unit.connect(not, and3);
    unit.connect(and3, xor2);
    unit.connect(xor2, ha_c);
    unit.connect(xor2, ha_s);
    unit.connect(ha_c, xor4);
    unit
}

/// double-dabble implementation
pub fn bin2bcd8b() -> LogicUnit {
    let mut unit = LogicUnit::new();
    let pluses: Vec<IO> = (0..7).map(|_| unit.embed(cond_plus_3())).collect();
    let o0 = unit.add_gate_output(LogicGateMode::And, false, sn!('O', 0));
    unit.io.create_input(sn!('I', 0), vec![o0]);
    unit.io
        .create_input(sn!('I', 1), pluses[4].get_input(sn!('A', 0)).ids.clone());
    unit.io
        .create_input(sn!('I', 2), pluses[3].get_input(sn!('A', 0)).ids.clone());
    unit.io
        .create_input(sn!('I', 3), pluses[2].get_input(sn!('A', 0)).ids.clone());
    unit.io
        .create_input(sn!('I', 4), pluses[1].get_input(sn!('A', 0)).ids.clone());
    unit.io
        .create_input(sn!('I', 5), pluses[0].get_input(sn!('A', 0)).ids.clone());
    unit.io
        .create_input(sn!('I', 6), pluses[0].get_input(sn!('A', 1)).ids.clone());
    unit.io
        .create_input(sn!('I', 7), pluses[0].get_input(sn!('A', 2)).ids.clone());

    for i in 0..4 {
        unit.connect_to_input(
            pluses[i].get_output(sn!('Q', 0)).id,
            pluses[i + 1].get_input(sn!('A', 1)),
        );
        unit.connect_to_input(
            pluses[i].get_output(sn!('Q', 1)).id,
            pluses[i + 1].get_input(sn!('A', 2)),
        );
        unit.connect_to_input(
            pluses[i].get_output(sn!('Q', 2)).id,
            pluses[i + 1].get_input(sn!('A', 3)),
        );
    }
    unit.connect_to_input(
        pluses[0].get_output(sn!('Q', 3)).id,
        pluses[5].get_input(sn!('A', 2)),
    );
    unit.connect_to_input(
        pluses[1].get_output(sn!('Q', 3)).id,
        pluses[5].get_input(sn!('A', 1)),
    );
    unit.connect_to_input(
        pluses[2].get_output(sn!('Q', 3)).id,
        pluses[5].get_input(sn!('A', 0)),
    );
    unit.connect_to_input(
        pluses[3].get_output(sn!('Q', 3)).id,
        pluses[6].get_input(sn!('A', 0)),
    );

    unit.connect_to_input(
        pluses[5].get_output(sn!('Q', 2)).id,
        pluses[6].get_input(sn!('A', 3)),
    );
    unit.connect_to_input(
        pluses[5].get_output(sn!('Q', 1)).id,
        pluses[6].get_input(sn!('A', 2)),
    );
    unit.connect_to_input(
        pluses[5].get_output(sn!('Q', 0)).id,
        pluses[6].get_input(sn!('A', 1)),
    );
    unit.io
        .create_output(sn!('O', 1), pluses[4].get_output(sn!('Q', 0)).id);
    unit.io
        .create_output(sn!('O', 2), pluses[4].get_output(sn!('Q', 1)).id);
    unit.io
        .create_output(sn!('O', 3), pluses[4].get_output(sn!('Q', 2)).id);
    unit.io
        .create_output(sn!('T', 0), pluses[4].get_output(sn!('Q', 3)).id);
    unit.io
        .create_output(sn!('T', 1), pluses[6].get_output(sn!('Q', 0)).id);
    unit.io
        .create_output(sn!('T', 2), pluses[6].get_output(sn!('Q', 1)).id);
    unit.io
        .create_output(sn!('T', 3), pluses[6].get_output(sn!('Q', 2)).id);
    unit.io
        .create_output(sn!('H', 0), pluses[6].get_output(sn!('Q', 3)).id);
    unit.io
        .create_output(sn!('H', 1), pluses[5].get_output(sn!('Q', 3)).id);

    unit
}

/// inc - I (1 tick)
/// set - S (1 tick)
/// data - D
/// output - O
pub fn counter_8b() -> LogicUnit {
    let mut unit = LogicUnit::new();
    let ands: Vec<Id> = (0..8)
        .map(|_| unit.add_gate(LogicGateMode::And, false))
        .collect();
    unit.io.create_input(sn!('S'), vec![]);
    let bits: Vec<IO> = (0..8)
        .map(|i| {
            let io = unit.embed(dff_1tick());
            unit.io.add_inputs(sn!('S'), &io.get_input(sn!('C')).ids);
            unit.io
                .create_input(sn!('D', i), io.get_input(sn!('D')).ids.clone());
            unit.io
                .create_output(sn!('O', i), io.get_output(sn!('Q')).id);
            io
        })
        .collect();
    unit.io.create_input(sn!('I'), ands.clone());

    let mut buf = Vec::new();
    for (i, &and) in ands.iter().enumerate() {
        unit.connect_to_input(and, bits[i].get_input(sn!('F')));
        for &id in &buf {
            unit.connect(id, and);
        }
        buf.push(bits[i].get_output(sn!('Q')).id);
    }

    unit
}
