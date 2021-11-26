use std::iter::FromIterator;
use std::time::Duration;
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_inspector_egui::widgets::Plotted;
use bevy_inspector_egui::{Inspectable, InspectorPlugin};


#[derive(Inspectable)]
struct SomeData {
    a: f32,
    float_vec: Plotted<Vec<f64>>,
    int_vec: Plotted<Vec<i8>>,
    float_arr: Plotted<[f32; 5]>,
    xy_pairs: Plotted<Vec<(f64, f64)>>,
    ty_pairs: Plotted<Vec<(Duration, f64)>>,
}

impl Default for SomeData {
    fn default() -> Self {
        Self {
            a: 1.0,
            float_vec: Plotted(vec![5.0, 2.0, 3.0, 4.0, 1.0]),
            int_vec: Plotted(vec![9, 50, -71]),
            float_arr: Plotted([5.0, 2.0, 3.0, 4.0, 1.0]),
            xy_pairs: Plotted(vec![(-1.0, 0.3), (0.0, 0.2), (1.0, 0.1)]),
            ty_pairs: Plotted(vec![(Duration::from_secs_f64(15.0), 0.3), (Duration::from_secs_f64(20.4), 0.2), (Duration::from_secs_f64(23.6), 0.1)]),
        }
    }
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(InspectorPlugin::<SomeData>::new())
        .run();
}
