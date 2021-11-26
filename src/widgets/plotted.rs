use std::ops::{Deref, DerefMut};
use std::time::Duration;

use crate::{Context, Inspectable};
use bevy_egui::egui;
use bevy_egui::egui::plot::{Plot, Values, Value, Line, Points};
use crate::egui::{Color32, Vec2};
use crate::egui::plot::Polygon;

/// The attributes for [`PlotAttributes`]
#[allow(missing_docs)]
#[derive(Copy, Clone)]
pub struct PlottedAttributes {
    pub scatter: bool,
    pub histogram: bool,
    pub kde: bool,
    pub min_x: Option<f64>,
    pub max_x: Option<f64>,
    pub min_y: Option<f64>,
    pub max_y: Option<f64>,
}

impl Default for PlottedAttributes {
    fn default() -> Self {
        PlottedAttributes {
            scatter: false,
            histogram: false,
            kde: false,
            min_x: None,
            max_x: None,
            min_y: None,
            max_y: None,
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

pub trait FloatMapped {
    fn extract_f64(&self) -> f64;
}

macro_rules! impl_num {
    ($ty:ty) => {
        impl FloatMapped for $ty
        {
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

impl FloatMapped for Duration {
    fn extract_f64(&self) -> f64 {
        self.as_secs_f64()
    }
}

pub trait ValueMapped {
    fn extract_value(&self, i: usize) -> Value;
}

impl<Y: FloatMapped> ValueMapped for Y {
    fn extract_value(&self, i: usize) -> Value {
        Value::new(i as f64, self.extract_f64())
    }
}

impl<X: FloatMapped, Y: FloatMapped> ValueMapped for (X, Y) {
    fn extract_value(&self, _i: usize) -> Value {
        Value::new(self.0.extract_f64(), self.1.extract_f64())
    }
}

impl ValueMapped for bevy::math::Vec2 {
    fn extract_value(&self, _i: usize) -> Value {
        Value::new(self.x, self.y)
    }
}

impl<T, E> Inspectable for Plotted<T>
    where for<'a> &'a T: IntoIterator<Item=&'a E>,
          E: ValueMapped
{
    type Attributes = PlottedAttributes;

    fn ui(&mut self, ui: &mut egui::Ui, options: Self::Attributes, context: &Context) -> git gbool {
        let mut plot = if options.histogram {
            let values = self.into_iter().enumerate().map(|(i, e)| e.extract_value(i)).collect();
            //TODO: create histogram directly from (non-enumerated) iterator
            let bins = calc_histogram_bins(&values, 21);
            let mut plot = Plot::new(context.id);
            let shared_color = Color32::from_rgb(188, 77, 77);
            for bin in bins {
                if bin.2 == 0 {
                    continue;
                }
                let bar: Vec<Value> = vec![
                    Value::new(bin.0, 0.0),
                    Value::new(bin.0, bin.2),
                    Value::new(bin.1, bin.2),
                    Value::new(bin.1, 0.0),
                ];
                plot = plot.polygon(Polygon::new(Values::from_values(bar)).color(shared_color));
            }
            plot
        } else if options.kde {
            let values = self.into_iter().enumerate().map(|(i, e)| e.extract_value(i)).collect();
            //TODO: create kde directly from (non-enumerated) iterator
            let gaussian_kernel = |x: f64, range: f64| -> f64 {
                let bw = range / 20.0;
                f64::exp(-(x * x) / (bw * bw))
            };
            Plot::new(context.id)
                .line(Line::new(calc_kde(&values, 100, gaussian_kernel)))
        } else {
            let values = Values::from_values_iter(self.into_iter().enumerate().map(|(i, e)| e.extract_value(i)));
            if options.scatter {
                Plot::new(context.id)
                    .points(Points::new(values).radius(3.0))
            } else {
                Plot::new(context.id)
                    .line(Line::new(values))
            }
        };
        if let Some(min_x) = options.min_x {
            plot = plot.include_x(min_x);
        }
        if let Some(max_x) = options.max_x {
            plot = plot.include_x(max_x);
        }
        if let Some(min_y) = options.min_y {
            plot = plot.include_y(min_y);
        }
        if let Some(max_y) = options.max_y {
            plot = plot.include_y(max_y);
        }
        plot = plot.view_aspect(5.0).min_size(Vec2::new(300.0, 100.0)).allow_drag(false).allow_zoom(false);
        ui.add(plot);
        false
    }
}

fn get_minmax(values: &Vec<Value>) -> Option<(f64, f64)> {
    if let (Some(min), Some(max)) = (values.iter().map(|v| v.y).reduce(f64::min), values.iter().map(|v| v.y).reduce(f64::max)) {
        Some((min, max))
    } else {
        None
    }
}

fn calc_histogram_bins(values: &Vec<Value>, num_bins: usize) -> Vec<(f64, f64, u32)> {
    if num_bins > 0 {
        if let Some((min, max)) = get_minmax(values) {
            let step = (max - min) / (num_bins as f64);
            let mut bins: Vec<(f64, f64, u32)> = (0..num_bins).map(|i| (min + i as f64 * step, min + i as f64 * step + step, 0u32)).collect();
            for el in values.iter() {
                let i = (((el.y - min) / step) as usize).clamp(0, num_bins - 1);
                bins[i].2 += 1;
            }
            return bins;
        }
    }
    vec![]
}

fn calc_kde<K>(values: &Vec<Value>, num_bins: usize, kernel_func: K) -> Values
    where K: Fn(f64, f64) -> f64
{
    if num_bins > 0 {
        if let Some((min, max)) = get_minmax(values) {
            let step = (max - min) / (num_bins as f64);
            let halfstep = step / 2.0;
            let mut bins: Vec<Value> = (0..num_bins).map(|i| Value::new(min + i as f64 * step + halfstep, 0.0)).collect();
            for el in values.iter() {
                for bin in bins.iter_mut() {
                    bin.y += kernel_func(el.y - bin.x, max - min);
                }
            }
            return Values::from_values(bins);
        }
    }
    Values::from_values(vec![])
}