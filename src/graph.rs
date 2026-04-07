use std::io::Write;

use crate::logic_unit::LogicUnit;

impl LogicUnit {
    pub fn save_dot(&self, path: &str) {
        let mut out = String::new();

        out.push_str("digraph LogicUnit {\n");
        out.push_str("    rankdir=LR;\n");

        // --- Inputs ---
        for input in &self.io.inputs {
            out.push_str(&format!(
                "    in_{} [label=\"{}\", shape=box, style=filled, fillcolor=lightblue];\n",
                input.name, input.name
            ));

            for &target in &input.ids {
                out.push_str(&format!("    in_{} -> node_{};\n", input.name, target));
            }
        }

        // --- Gates ---
        for gate in &self.gates {
            let label = format!("{}_{}", gate.mode, gate.id);

            // цвет по active
            let color = if gate.active {
                "lightgreen"
            } else {
                "lightgray"
            };

            out.push_str(&format!(
                "    node_{} [label=\"{}\", shape=circle, style=filled, fillcolor={}];\n",
                gate.id, label, color
            ));

            // связи через children
            for &child in &gate.children {
                out.push_str(&format!("    node_{} -> node_{};\n", gate.id, child));
            }
        }

        // --- Outputs ---
        for output in &self.io.outputs {
            out.push_str(&format!(
                "    out_{} [label=\"{}\", shape=box, style=filled, fillcolor=lightyellow];\n",
                output.name, output.name
            ));

            out.push_str(&format!("    node_{} -> out_{};\n", output.id, output.name));
        }

        out.push_str("}\n");

        std::fs::File::create(path)
            .unwrap()
            .write_all(out.as_bytes())
            .unwrap();
        println!("graph saved to {path}");
    }
}
