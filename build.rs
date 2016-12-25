extern crate gcc;

fn main() {
    gcc::compile_library("libyoga.a",
                         &["./yoga/yoga/Yoga.c", "./yoga/yoga/YGNodeList.c"]);
}
