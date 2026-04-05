// logic gate id: 9f0f56e8-2c31-4d83-996c-d00a9b296c3f
// metal 2 block id: 1016cafc-9f6b-40c9-8713-9019d399783f
// switch id: 7cf717d7-d167-4f2d-a6e7-6b2c70aa3986

use crate::{logic_unit::LogicUnit, utils::SignalName};
use std::{fs::File, io::Write};

use crate::{blueprint::Blueprint, graph::run_graph, logic::*, pos::Pos};

mod blueprint;
mod color;
mod emulator;
mod graph;
mod logic;
mod logic_gate;
mod logic_unit;
mod pos;
mod table_parser;
mod utils;

fn save_blueprint(json: String) {
    let path = std::env::var("BLUEPRINT_PATH").unwrap_or("blueprint.json".to_string());
    println!("blueprint saved to {}", path);
    let mut f = File::create(path).unwrap();
    f.write_all(json.as_bytes()).unwrap();
}

fn bcd_test() -> Blueprint {
    let mut blueprint = Blueprint::new();

    let mut alu = alu_8b_4m();
    let mut bcd_conv = bin2bcd8b();
    let mut digit1 = digit_dispay();
    let mut digit2 = digit_dispay();
    let mut digit3 = digit_dispay();
    for i in 0..8 {
        alu.connect_to_input(
            alu.io.get_output(sn!('O', i)).id,
            bcd_conv.io.get_input(sn!('I', i)),
        );
    }
    for i in 0..4 {
        bcd_conv.connect_to_input(
            bcd_conv.io.get_output(sn!('O', i)).id,
            digit1.io.get_input(sn!('S', i)),
        );
        bcd_conv.connect_to_input(
            bcd_conv.io.get_output(sn!('T', i)).id,
            digit2.io.get_input(sn!('S', i)),
        );
    }
    bcd_conv.connect_to_input(
        bcd_conv.io.get_output(sn!('H', 0)).id,
        digit3.io.get_input(sn!('S', 0)),
    );
    bcd_conv.connect_to_input(
        bcd_conv.io.get_output(sn!('H', 1)).id,
        digit3.io.get_input(sn!('S', 1)),
    );

    blueprint.place(alu.assemble_io(Pos::default(), true));
    blueprint.place(bcd_conv.assemble_single(Pos::new(0, 1, 0)));
    blueprint.place(digit1.assemble_display(Pos::new(0, -1, 0), 3));
    blueprint.place(digit2.assemble_display(Pos::new(-4, -1, 0), 3));
    blueprint.place(digit3.assemble_display(Pos::new(-8, -1, 0), 3));
    blueprint
}

fn bp() {
    let mut blueprint = Blueprint::new();
    let mut unit = LogicUnit::from_table("/home/alexey/dev/learn/verilog/cla_adder_64b.table");
    blueprint.place(unit.assemble_io(Pos::new(0, 1, 0), true));
    let json = blueprint.to_json().to_string();
    // println!("{json}");
    save_blueprint(json);
}

fn main() {
    let unit = LogicUnit::from_table("/home/alexey/dev/learn/verilog/test.table");
    // emu(unit);

    bp();
}
