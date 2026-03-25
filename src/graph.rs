use std::collections::HashMap;

use petgraph::prelude::*;

use crate::logic_unit::LogicUnit;

impl LogicUnit {
    fn to_graph(self) -> StableGraph<(), ()> {
        let mut graph = StableGraph::new();
        let mut id_to_node = HashMap::new();

        // Add a node for every gate
        for gate in &self.gates {
            let node = graph.add_node(());
            id_to_node.insert(gate.id, node);
        }

        // Add edges from each gate to its children
        for gate in &self.gates {
            let from_node = id_to_node[&gate.id];
            for &child_id in &gate.children {
                if let Some(&to_node) = id_to_node.get(&child_id) {
                    graph.add_edge(from_node, to_node, ());
                } else {
                    // The child ID does not correspond to a known gate.
                    // This could be an input/output node, but without
                    // additional information it is skipped.
                    eprintln!("Warning: child ID {} not found in gates", child_id);
                }
            }
        }

        graph
    }
}

pub fn run_graph(unit: LogicUnit) {
    eframe::run_native(
        "egui_graphs_basic_demo",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(BasicApp::new(unit, cc)))),
    )
    .unwrap();
}

struct BasicApp {
    g: egui_graphs::Graph,
}

impl BasicApp {
    fn new(unit: LogicUnit, _: &eframe::CreationContext<'_>) -> Self {
        Self {
            g: egui_graphs::Graph::from(&unit.to_graph()),
        }
    }
}
impl eframe::App for BasicApp {
    fn update(&mut self, ctx: &egui::Context, _: &mut eframe::Frame) {
        type L = egui_graphs::LayoutHierarchical;
        type S = egui_graphs::LayoutStateHierarchical;
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(&mut egui_graphs::GraphView::<_, _, _, _, _, _, S, L>::new(
                &mut self.g,
            ));
        });
    }
}
