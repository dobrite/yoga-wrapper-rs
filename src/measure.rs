use Node;
use ffi;

pub extern "C" fn measure(
    node: *mut ffi::Node,
    width: f32,
    width_mode: ffi::MeasureMode,
    height: f32,
    height_mode: ffi::MeasureMode
) -> ffi::Size {
    let n = Node { _node: node };
    n.get_context().measure()
}
