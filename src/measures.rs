use ffi::size::Size;

pub trait Measures {
    fn measure(&self, text: &str) -> Size;
}
