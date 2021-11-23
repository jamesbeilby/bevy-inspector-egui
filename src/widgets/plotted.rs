use std::ops::{Deref, DerefMut};

use crate::{Context, Inspectable};
use bevy_egui::egui::{self, Id, Vec2};
use bevy_egui::egui::plot::{Plot, Values, Value, Line, Points};

#[derive(Copy, Clone)]
pub enum PlotType {
    Line,
    Scatter,
    Histogram,
}

/// The attributes for [`PlotAttributes`]
#[allow(missing_docs)]
#[derive(Copy, Clone)]
pub struct PlottedAttributes {
    pub plot_type: PlotType,
}

impl Default for PlottedAttributes {
    fn default() -> Self {
        PlottedAttributes {
            plot_type: PlotType::Scatter,
        }
    }
}

#[derive(Default)]
/// Wrapper type which plots the inner 1D or 2D collection.
///
/// Can be configured using [`PlottedAttributes`].
pub struct Plotted<T>(pub T);

impl<T> Deref for Plotted<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Plotted<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T, E> Inspectable for Plotted<T>
    where for<'a> &'a T: IntoIterator<Item=&'a E>,
          E: Copy + Into<f64>
{
    type Attributes = PlottedAttributes;

    fn ui(&mut self, ui: &mut egui::Ui, options: Self::Attributes, context: &Context) -> bool {
        let x = self.0.into_iter().enumerate().map(|(i, &y)| Value::new(i as f64, y));
        let values = Values::from_values_iter(x);
        let mut plot = match options.plot_type {
            PlotType::Line => {
                Plot::new("lines_demo")
                    .line(Line::new(values))
            }
            PlotType::Scatter => {
                Plot::new("lines_demo")
                    .points(Points::new(values).radius(3.0))
            },
            PlotType::Histogram => {
                unimplemented!();
            }
        };
        plot = plot.view_aspect(5.0).min_size(Vec2::new(300.0, 100.0)).allow_drag(false).allow_zoom(false);
        ui.add(plot);
        false
    }
}

// TODO: separate values extractor incl for (K, V)
// https://stackoverflow.com/questions/40392524/conflicting-trait-implementations-even-though-associated-types-differ
