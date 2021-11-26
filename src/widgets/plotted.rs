use std::ops::{Deref, DerefMut};
use std::time::Duration;

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

pub trait ValueMapped {
    fn extract_value(&self, i: usize) -> Value;
    fn extract_f64(&self) -> f64;
}

macro_rules! impl_num {
    ($ty:ty) => {
        impl ValueMapped for $ty
        {
            fn extract_value(&self, i: usize) -> Value {
                Value::new(i as f64, self.extract_f64())
            }

            fn extract_f64(&self) -> f64 {
                *self as f64
            }
        }
    }
}

impl_num!(f32);
impl_num!(f64);
impl_num!(i8);
impl_num!(i16);
impl_num!(i32);
impl_num!(i64);     // truncates
impl_num!(isize);   // truncates
impl_num!(u8);
impl_num!(u16);
impl_num!(u32);
impl_num!(u64);     // truncates
impl_num!(usize);   // truncates

impl ValueMapped for Duration {
    fn extract_value(&self, i: usize) -> Value {
        Value::new(i as f64, self.extract_f64())
    }

    fn extract_f64(&self) -> f64 {
        self.as_secs_f64()
    }
}

impl<X: ValueMapped, Y: ValueMapped> ValueMapped for (X, Y) {
    fn extract_value(&self, i: usize) -> Value {
        Value::new(self.0.extract_f64(), self.1.extract_f64())
    }

    fn extract_f64(&self) -> f64 {
        unimplemented!()
    }
}

impl<T, E> Inspectable for Plotted<T>
    where for<'a> &'a T: IntoIterator<Item=&'a E>,
          E: ValueMapped
{
    type Attributes = PlottedAttributes;

    fn ui(&mut self, ui: &mut egui::Ui, options: Self::Attributes, context: &Context) -> bool {
        let values = Values::from_values_iter(self.into_iter().enumerate().map(|(i, e)| e.extract_value(i)));
        let mut plot = match options.plot_type {
            PlotType::Line => {
                Plot::new("lines_demo")
                    .line(Line::new(values))
            }
            PlotType::Scatter => {
                Plot::new("lines_demo")
                    .points(Points::new(values).radius(3.0))
            }
            PlotType::Histogram => {
                unimplemented!();
            }
        };
        plot = plot.view_aspect(5.0).min_size(Vec2::new(300.0, 100.0)).allow_drag(false).allow_zoom(false);
        ui.add(plot);
        false
    }
}