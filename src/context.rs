use ffi::size::Size;
use measures::Measures;

pub struct Context<'a, 'm> {
    text: &'a str,
    measurer: &'m Measures,
}

impl<'a, 'm> Context<'a, 'm> {
    pub fn new<M: Measures>(text: &'a str, measures: &'m M) -> Context<'a, 'm> {
        Context {
            text: text,
            measurer: measures,
        }
    }

    pub fn measure(&self) -> Size {
        self.measurer.measure(self.text)
    }
}
