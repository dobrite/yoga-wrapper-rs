use ffi::size::Size;

pub struct Measurer;

pub trait Measures {
    fn measure(&self, text: &str) -> Size {
        Size {
            width: text.len() as f32,
            height: 1.0,
        }
    }
}

impl Measures for Measurer {}
