extern crate gcc;

fn main() {
    gcc::compile_library("libyoga.a",
                         &["/usr/local/lib/yoga/yoga/Yoga.c",
                           "/usr/local/lib/yoga/yoga/YGNodeList.c"]);
}
