use std::iter::FromIterator;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_inspector_egui::widgets::Plotted;
use bevy_inspector_egui::{Inspectable, InspectorPlugin};


#[derive(Inspectable)]
struct SomeData {
    a: f32,
    float_vec: Plotted<Vec<f64>>,
    float_arr: Plotted<[f64; 5]>,
    int_arr: Plotted<[i8; 3]>,
    // xy_pairs: Plotted<Vec<(f64, f64)>>,
    // xy_map: Plotted<HashMap<i32, f64>>,
}

impl Default for SomeData {
    fn default() -> Self {
        Self {
            a: 1.0,
            float_vec: Plotted(vec![5.0, 2.0, 3.0, 4.0, 1.0]),
            float_arr: Plotted([5.0, 2.0, 3.0, 4.0, 1.0]),
            int_arr: Plotted([9, 50, -71]),
            // xy_pairs: Plotted(vec![(-1.0, 0.3), (0.0, 0.2), (1.0, 0.1)]),
            // xy_map: Plotted(HashMap::from_iter(vec![(1000, 0.3), (1015, 0.2), (1040, 0.1)])),
        }
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(InspectorPlugin::<SomeData>::new())
        .run();
}
