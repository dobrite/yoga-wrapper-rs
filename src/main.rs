extern crate yoga;

fn main() {
    let root = yoga::Node::new();
    root.set_width(500.0);
    root.set_height(120.0);
    root.set_flex_direction(yoga::FlexDirection::Row);
    root.set_padding(yoga::Edge::All, 20.0);

    let image = yoga::Node::new();
    image.set_width(80.0);
    image.set_margin(yoga::Edge::End, 20.0);

    let text = yoga::Node::new();
    text.set_height(25.0);
    text.set_align_self(yoga::Align::Center);
    text.set_flex_grow(1.0);

    root.insert_child(&image, 0);
    root.insert_child(&text, 1);

    root.calculate_layout();

    println!("height: {:?}", root.get_layout_height());
    println!("width:  {:?}", root.get_layout_width());
    println!("left:   {:?}", root.get_layout_left());
    println!("top:    {:?}", root.get_layout_top());

    println!("-------------");

    println!("height: {:?}", image.get_layout_height());
    println!("width:  {:?}", image.get_layout_width());
    println!("left:   {:?}", image.get_layout_left());
    println!("top:    {:?}", image.get_layout_top());

    println!("-------------");

    println!("height: {:?}", text.get_layout_height());
    println!("width:  {:?}", text.get_layout_width());
    println!("left:   {:?}", text.get_layout_left());
    println!("top:    {:?}", text.get_layout_top());
}
