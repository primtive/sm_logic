// logic gate id: 9f0f56e8-2c31-4d83-996c-d00a9b296c3f
// metal 2 block id: 1016cafc-9f6b-40c9-8713-9019d399783f
// switch id: 7cf717d7-d167-4f2d-a6e7-6b2c70aa3986

use std::{fs::File, io::Write};

use crate::{blueprint::Blueprint, graph::run_graph, logic::*, pos::Pos};

mod blueprint;
mod color;
mod graph;
mod logic;
mod logic_gate;
mod logic_unit;
mod pos;
mod utils;

fn save_blueprint(json: String) {
    let mut f = File::create(
        "/home/alexey/.steam/steam/steamapps/compatdata/387990/pfx/drive_c/users/steamuser/AppData/Roaming/Axolot Games/Scrap Mechanic/User/User_76561199049407465/Blueprints/8737ebc7-7ca3-4c28-aa60-e3e0a65b950e/blueprint.json",
    ).unwrap();
    f.write_all(json.as_bytes()).unwrap();
}

fn main() {
    // run_graph(alu_8b_4m());

    let mut blueprint = Blueprint::new();
    // blueprint.place(ticker().assemble_io(Pos::default(), false));
    blueprint.place(alu_8b_4m().assemble_io(Pos::new(0, 1, 0), true));
    // blueprint.place(adder_substractor_8b().assemble_io(Pos::new(0, 2, 0), true));
    let json = blueprint.to_json().to_string();
    // println!("{json}");
    save_blueprint(json);
    println!("file saved");
}
