extern crate libc;

mod ffi;

pub use self::ffi::Align;
pub use self::ffi::Dimensions;
pub use self::ffi::Direction;
pub use self::ffi::Edge;
pub use self::ffi::ExperimentalFeature;
pub use self::ffi::FlexDirection;
pub use self::ffi::FlexWrap;
pub use self::ffi::JustifyContent;
pub use self::ffi::LogLevel;
pub use self::ffi::MeasureMode;
pub use self::ffi::Overflow;
pub use self::ffi::PositionType;
pub use self::ffi::PrintOptions;
pub use self::ffi::Size;

pub extern "C" fn measure(mut node: ffi::Node,
                          width: f32,
                          width_mode: MeasureMode,
                          height: f32,
                          height_mode: MeasureMode)
                          -> Size {
    Size {
        width: 3.0,
        height: 1.0,
    }
}

#[derive(Debug)]
pub struct Node {
    _node: *mut ffi::Node,
}

impl Node {
    pub fn new() -> Node {
        Node { _node: unsafe { ffi::YGNodeNew() } }
    }

    pub fn get_instance_count() -> i32 {
        unsafe { ffi::YGNodeGetInstanceCount() }
    }

    pub fn reset(&self) {
        unsafe { ffi::YGNodeReset(self._node) }
    }

    pub fn free(&self) {
        unsafe { ffi::YGNodeFree(self._node) }
    }

    pub fn free_recursive(&self) {
        unsafe { ffi::YGNodeFreeRecursive(self._node) }
    }

    pub fn set_direction(&self, value: ffi::Direction) {
        unsafe { ffi::YGNodeStyleSetDirection(self._node, value) }
    }

    pub fn get_direction(&self) -> ffi::Direction {
        unsafe { ffi::YGNodeStyleGetDirection(self._node) }
    }

    pub fn set_flex_direction(&self, value: ffi::FlexDirection) {
        unsafe { ffi::YGNodeStyleSetFlexDirection(self._node, value) }
    }

    pub fn get_flex_direction(&self) -> ffi::FlexDirection {
        unsafe { ffi::YGNodeStyleGetFlexDirection(self._node) }
    }

    pub fn set_width(&self, width: f32) {
        unsafe { ffi::YGNodeStyleSetWidth(self._node, width) }
    }

    pub fn get_width(&self) -> f32 {
        unsafe { ffi::YGNodeStyleGetWidth(self._node) }
    }

    pub fn set_height(&self, height: f32) {
        unsafe { ffi::YGNodeStyleSetHeight(self._node, height) }
    }

    pub fn get_height(&self) -> f32 {
        unsafe { ffi::YGNodeStyleGetHeight(self._node) }
    }

    pub fn calculate_layout(&self) {
        unsafe {
            ffi::YGNodeCalculateLayout(self._node, std::f32::NAN, std::f32::NAN, Direction::LTR)
        }
    }

    pub fn get_layout_width(&self) -> f32 {
        unsafe { ffi::YGNodeLayoutGetWidth(self._node) }
    }

    pub fn get_layout_height(&self) -> f32 {
        unsafe { ffi::YGNodeLayoutGetHeight(self._node) }
    }

    pub fn get_layout_left(&self) -> f32 {
        unsafe { ffi::YGNodeLayoutGetLeft(self._node) }
    }

    pub fn get_layout_top(&self) -> f32 {
        unsafe { ffi::YGNodeLayoutGetTop(self._node) }
    }

    pub fn get_layout_direction(&self) -> ffi::Direction {
        unsafe { ffi::YGNodeLayoutGetDirection(self._node) }
    }

    pub fn insert_child(&self, child: &Node, index: u32) {
        unsafe { ffi::YGNodeInsertChild(self._node, child._node, index) }
    }

    pub fn remove_child(&self, child: &Node) {
        unsafe { ffi::YGNodeRemoveChild(self._node, child._node) }
    }

    pub fn get_child(&self, index: u32) -> Node {
        unsafe {
            let mut node = ffi::YGNodeGetChild(self._node, index);
            Node { _node: &mut node } // this is wrong
        }
    }

    pub fn get_child_count(&self) -> u32 {
        unsafe { ffi::YGNodeGetChildCount(self._node) }
    }

    // TODO option
    pub fn get_parent(&self) -> Node {
        unsafe {
            let mut node = ffi::YGNodeGetParent(self._node);
            Node { _node: &mut node } // this is wrong
        }
    }

    pub fn set_justify_content(&self, value: ffi::JustifyContent) {
        unsafe { ffi::YGNodeStyleSetJustifyContent(self._node, value) }
    }

    pub fn get_justify_content(&self) -> ffi::JustifyContent {
        unsafe { ffi::YGNodeStyleGetJustifyContent(self._node) }
    }

    pub fn get_align_items(&self) -> ffi::Align {
        unsafe { ffi::YGNodeStyleGetAlignItems(self._node) }
    }

    pub fn set_align_items(&self, value: ffi::Align) {
        unsafe { ffi::YGNodeStyleSetAlignItems(self._node, value) }
    }

    pub fn get_align_self(&self) -> ffi::Align {
        unsafe { ffi::YGNodeStyleGetAlignSelf(self._node) }
    }

    pub fn set_align_self(&self, value: ffi::Align) {
        unsafe { ffi::YGNodeStyleSetAlignSelf(self._node, value) }
    }

    pub fn get_align_content(&self) -> ffi::Align {
        unsafe { ffi::YGNodeStyleGetAlignContent(self._node) }
    }

    pub fn set_align_content(&self, value: ffi::Align) {
        unsafe { ffi::YGNodeStyleSetAlignContent(self._node, value) }
    }

    pub fn get_position_type(&self) -> ffi::PositionType {
        unsafe { ffi::YGNodeStyleGetPositionType(self._node) }
    }

    pub fn set_position_type(&self, value: ffi::PositionType) {
        unsafe { ffi::YGNodeStyleSetPositionType(self._node, value) }
    }

    pub fn get_flex_wrap(&self) -> ffi::FlexWrap {
        unsafe { ffi::YGNodeStyleGetFlexWrap(self._node) }
    }

    pub fn set_flex_wrap(&self, value: ffi::FlexWrap) {
        unsafe { ffi::YGNodeStyleSetFlexWrap(self._node, value) }
    }

    pub fn get_overflow(&self) -> ffi::Overflow {
        unsafe { ffi::YGNodeStyleGetOverflow(self._node) }
    }

    pub fn set_overflow(&self, value: ffi::Overflow) {
        unsafe { ffi::YGNodeStyleSetOverflow(self._node, value) }
    }

    pub fn set_flex(&self, value: f32) {
        unsafe { ffi::YGNodeStyleSetFlex(self._node, value) }
    }

    pub fn get_flex_grow(&self) -> f32 {
        unsafe { ffi::YGNodeStyleGetFlexGrow(self._node) }
    }

    pub fn set_flex_grow(&self, value: f32) {
        unsafe { ffi::YGNodeStyleSetFlexGrow(self._node, value) }
    }

    pub fn get_flex_shrink(&self) -> f32 {
        unsafe { ffi::YGNodeStyleGetFlexShrink(self._node) }
    }

    pub fn set_flex_shrink(&self, value: f32) {
        unsafe { ffi::YGNodeStyleSetFlexShrink(self._node, value) }
    }

    pub fn get_flex_basis(&self) -> f32 {
        unsafe { ffi::YGNodeStyleGetFlexBasis(self._node) }
    }

    pub fn set_flex_basis(&self, value: f32) {
        unsafe { ffi::YGNodeStyleSetFlexBasis(self._node, value) }
    }

    pub fn get_margin(&self, edge: ffi::Edge) -> f32 {
        unsafe { ffi::YGNodeStyleGetMargin(self._node, edge) }
    }

    pub fn set_margin(&self, edge: ffi::Edge, value: f32) {
        unsafe { ffi::YGNodeStyleSetMargin(self._node, edge, value) }
    }

    pub fn get_padding(&self, edge: ffi::Edge) -> f32 {
        unsafe { ffi::YGNodeStyleGetPadding(self._node, edge) }
    }

    pub fn set_padding(&self, edge: ffi::Edge, value: f32) {
        unsafe { ffi::YGNodeStyleSetPadding(self._node, edge, value) }
    }

    pub fn get_border(&self, edge: ffi::Edge) -> f32 {
        unsafe { ffi::YGNodeStyleGetBorder(self._node, edge) }
    }

    pub fn set_border(&self, edge: ffi::Edge, value: f32) {
        unsafe { ffi::YGNodeStyleSetBorder(self._node, edge, value) }
    }

    pub fn get_position(&self, edge: ffi::Edge) -> f32 {
        unsafe { ffi::YGNodeStyleGetPosition(self._node, edge) }
    }

    pub fn set_position(&self, edge: ffi::Edge, value: f32) {
        unsafe { ffi::YGNodeStyleSetPosition(self._node, edge, value) }
    }

    pub fn set_measure_func(&self,
                            func: extern "C" fn(ffi::Node, f32, MeasureMode, f32, MeasureMode)
                                                -> Size) {
        unsafe { ffi::YGNodeSetMeasureFunc(self._node, func) }
    }

    pub fn mark_dirty(&self) {
        unsafe { ffi::YGNodeMarkDirty(self._node) }
    }

    pub fn is_dirty(&self) -> bool {
        unsafe { ffi::YGNodeIsDirty(self._node) }
    }

    // boolean hasNewLayout();
    // void markLayoutSeen();
    // void copyStyle(YogaNodeType srcNode);
    // TODO int indexOf(YogaNodeType child);
    // TODO boolean isMeasureDefined();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
