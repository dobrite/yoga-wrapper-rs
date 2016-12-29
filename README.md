A verbatim rust wrapper for [Yoga](https://github.com/facebook/yoga). Probably most likely very unsafe.

Expects the yoga repo to be cloned in `/usr/local/lib/` (see `build.rs`).

#### Running the "C" example:

```rust
let mut root = yoga::Node::new();
root.set_width(500.0);
root.set_height(120.0);
root.set_flex_direction(yoga::FlexDirection::Row);
root.set_padding(yoga::Edge::All, 20.0);

let mut image = yoga::Node::new();
image.set_width(80.0);
image.set_margin(yoga::Edge::End, 20.0);

let mut text = yoga::Node::new();
text.set_height(25.0);
text.set_align_self(yoga::Align::Center);
text.set_flex_grow(1.0);

root.insert_child(&image, 0);
root.insert_child(&text, 1);

root.calculate_layout();
```

#### Output:

```text
height: 120
width:  500
left:   0
top:    0
-------------
height: 80
width:  80
left:   20
top:    20
-------------
height: 25
width:  360
left:   120
top:    47.5
```

TODO:

- [ ] can use cached measurement
- [ ] copy style
- [ ] YG_NODE_PROPERTYs
- [ ] print func and node print
- [ ] logger
- [ ] boolean hasNewLayout();
- [ ] void markLayoutSeen();
- [ ] void copyStyle(YogaNodeType srcNode);
- [ ] int indexOf(YogaNodeType child);
- [ ] boolean isMeasureDefined();
- [ ] simple renderer (text via rustbox?)
- [ ] higher level "safe" wrapper

License: MIT (see MIT-LICENSE)
