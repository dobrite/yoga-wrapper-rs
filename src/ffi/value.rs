use libc::c_float;
use ffi::unit::Unit;

#[repr(C)]
#[derive(Debug)]
pub struct Value {
    pub value: c_float,
    pub unit: Unit
}
