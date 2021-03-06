use libc::{c_float, c_void, int32_t, uint32_t};

pub mod align;
pub mod dimensions;
pub mod direction;
pub mod edge;
pub mod experimental_feature;
pub mod flex_direction;
pub mod flex_wrap;
pub mod justify_content;
pub mod log_level;
pub mod measure_mode;
pub mod overflow;
pub mod position_type;
pub mod print_options;
pub mod size;
pub mod unit;
pub mod value;

pub use self::align::Align;
pub use self::dimensions::Dimensions;
pub use self::direction::Direction;
pub use self::edge::Edge;
pub use self::experimental_feature::ExperimentalFeature;
pub use self::flex_direction::FlexDirection;
pub use self::flex_wrap::FlexWrap;
pub use self::justify_content::JustifyContent;
pub use self::log_level::LogLevel;
pub use self::measure_mode::MeasureMode;
pub use self::overflow::Overflow;
pub use self::position_type::PositionType;
pub use self::print_options::PrintOptions;
pub use self::size::Size;
pub use self::unit::Unit;
pub use self::value::Value;

#[repr(C)]
#[derive(Debug)]
pub struct Node {}

#[link(name = "yoga")]
extern "C" {
    pub fn YGNodeNew() -> *mut Node;
    pub fn YGNodeFree(node: *mut Node);
    pub fn YGNodeFreeRecursive(node: *mut Node);
    pub fn YGNodeReset(node: *mut Node);
    pub fn YGNodeGetInstanceCount() -> int32_t;

    pub fn YGNodeInsertChild(node: *mut Node, child: *const Node, index: uint32_t);
    pub fn YGNodeRemoveChild(node: *mut Node, child: *const Node);
    pub fn YGNodeGetChild(node: *const Node, index: uint32_t) -> *mut Node;
    pub fn YGNodeGetParent(node: *const Node) -> *mut Node;
    pub fn YGNodeGetChildCount(node: *const Node) -> uint32_t;

    pub fn YGNodeCalculateLayout(
        node: *mut Node,
        availableWidth: c_float,
        availableHeight: c_float,
        parentDirection: Direction
    );

    pub fn YGNodeMarkDirty(node: *mut Node);
    pub fn YGNodeIsDirty(node: *const Node) -> bool;

    pub fn YGNodeGetContext(node: *const Node) -> *mut c_void;
    pub fn YGNodeSetContext(node: *mut Node, context: *mut c_void);
    pub fn YGNodeSetMeasureFunc(
        node: *mut Node,
        func: extern "C" fn(*mut Node, c_float, MeasureMode, c_float, MeasureMode) -> Size
    );

    pub fn YGNodeStyleGetDirection(node: *const Node) -> Direction;
    pub fn YGNodeStyleSetDirection(node: *mut Node, direction: Direction);
    pub fn YGNodeStyleGetFlexDirection(node: *const Node) -> FlexDirection;
    pub fn YGNodeStyleSetFlexDirection(node: *mut Node, flexDirection: FlexDirection);
    pub fn YGNodeStyleGetJustifyContent(node: *const Node) -> JustifyContent;
    pub fn YGNodeStyleSetJustifyContent(node: *mut Node, justify_content: JustifyContent);
    pub fn YGNodeStyleGetAlignContent(node: *const Node) -> Align;
    pub fn YGNodeStyleSetAlignContent(node: *mut Node, align_content: Align);
    pub fn YGNodeStyleGetAlignItems(node: *const Node) -> Align;
    pub fn YGNodeStyleSetAlignItems(node: *mut Node, align_items: Align);
    pub fn YGNodeStyleGetAlignSelf(node: *const Node) -> Align;
    pub fn YGNodeStyleSetAlignSelf(node: *mut Node, align_self: Align);
    pub fn YGNodeStyleGetPositionType(node: *const Node) -> PositionType;
    pub fn YGNodeStyleSetPositionType(node: *mut Node, overflow: PositionType);
    pub fn YGNodeStyleGetFlexWrap(node: *const Node) -> FlexWrap;
    pub fn YGNodeStyleSetFlexWrap(node: *mut Node, overflow: FlexWrap);
    pub fn YGNodeStyleGetOverflow(node: *const Node) -> Overflow;
    pub fn YGNodeStyleSetOverflow(node: *mut Node, overflow: Overflow);

    pub fn YGNodeStyleSetFlex(node: *mut Node, flex: c_float);
    pub fn YGNodeStyleGetFlexGrow(node: *const Node) -> c_float;
    pub fn YGNodeStyleSetFlexGrow(node: *mut Node, flex_grow: c_float);
    pub fn YGNodeStyleGetFlexShrink(node: *const Node) -> c_float;
    pub fn YGNodeStyleSetFlexShrink(node: *mut Node, flex_shrink: c_float);

    pub fn YGNodeStyleGetFlexBasis(node: *const Node) -> c_float;
    pub fn YGNodeStyleSetFlexBasis(node: *mut Node, flex_basis: c_float);
    pub fn YGNodeStyleSetFlexBasisPercent(node: *const Node, flex_basis: c_float);
    pub fn YGNodeStyleSetFlexBasisAuto(node: *const Node);

    pub fn YGNodeStyleGetPosition(node: *const Node, edge: Edge) -> Value;
    pub fn YGNodeStyleSetPosition(node: *mut Node, edge: Edge, position: c_float);
    pub fn YGNodeStyleSetPositionPercent(node: *mut Node, edge: Edge, position: c_float);

    pub fn YGNodeStyleGetMargin(node: *const Node, edge: Edge) -> Value;
    pub fn YGNodeStyleSetMargin(node: *mut Node, edge: Edge, margin: c_float);
    pub fn YGNodeStyleSetMarginPercent(node: *mut Node, edge: Edge, position: c_float);
    pub fn YGNodeStyleSetMarginAuto(node: *mut Node, edge: Edge);

    pub fn YGNodeStyleGetPadding(node: *const Node, edge: Edge) -> Value;
    pub fn YGNodeStyleSetPadding(node: *mut Node, edge: Edge, padding: c_float);
    pub fn YGNodeStyleSetPaddingPercent(node: *mut Node, edge: Edge, position: c_float);

    pub fn YGNodeStyleGetBorder(node: *const Node, edge: Edge) -> c_float;
    pub fn YGNodeStyleSetBorder(node: *mut Node, edge: Edge, border: c_float);

    pub fn YGNodeStyleGetWidth(node: *const Node) -> c_float;
    pub fn YGNodeStyleSetWidth(node: *mut Node, width: c_float);
    pub fn YGNodeStyleSetWidthPercent(node: *const Node, width: c_float);
    pub fn YGNodeStyleSetWidthAuto(node: *const Node);

    pub fn YGNodeStyleGetHeight(node: *const Node) -> c_float;
    pub fn YGNodeStyleSetHeight(node: *mut Node, height: c_float);
    pub fn YGNodeStyleSetHeightPercent(node: *const Node, height: c_float);
    pub fn YGNodeStyleSetHeightAuto(node: *const Node);

    pub fn YGNodeStyleGetMinWidth(node: *const Node) -> c_float;
    pub fn YGNodeStyleSetMinWidth(node: *mut Node, min_width: c_float);
    pub fn YGNodeStyleSetMinWidthPercent(node: *const Node, min_width: c_float);

    pub fn YGNodeStyleGetMinHeight(node: *const Node) -> c_float;
    pub fn YGNodeStyleSetMinHeight(node: *mut Node, min_height: c_float);
    pub fn YGNodeStyleSetMinHeightPercent(node: *const Node, min_height: c_float);

    pub fn YGNodeStyleGetMaxWidth(node: *const Node) -> c_float;
    pub fn YGNodeStyleSetMaxWidth(node: *mut Node, max_width: c_float);
    pub fn YGNodeStyleSetMaxWidthPercent(node: *const Node, max_width: c_float);

    pub fn YGNodeStyleGetMaxHeight(node: *const Node) -> c_float;
    pub fn YGNodeStyleSetMaxHeight(node: *mut Node, max_height: c_float);
    pub fn YGNodeStyleSetMaxHeightPercent(node: *const Node, max_height: c_float);

    pub fn YGNodeLayoutGetLeft(node: *const Node) -> c_float;
    pub fn YGNodeLayoutGetTop(node: *const Node) -> c_float;
    pub fn YGNodeLayoutGetRight(node: *const Node) -> c_float;
    pub fn YGNodeLayoutGetBottom(node: *const Node) -> c_float;
    pub fn YGNodeLayoutGetWidth(node: *const Node) -> c_float;
    pub fn YGNodeLayoutGetHeight(node: *const Node) -> c_float;
    pub fn YGNodeLayoutGetDirection(node: *const Node) -> Direction;
}
