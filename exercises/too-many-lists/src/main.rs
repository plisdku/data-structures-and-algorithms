// use too_many_lists::first::DumbList;

use std::mem::size_of;

fn observe_null_pointer_optimization() {
    println!("{}", size_of::<Option<i64>>()); // 16
    println!("{}", size_of::<Option<&i64>>()); // 8
}

fn main() {
    println!("Hello, whirled!");

    observe_null_pointer_optimization();
}

// #[derive(Copy, Clone)]
// struct NotCopyable();

// struct Foo {
//     x: NotCopyable,
// }

// fn thing(foo: &mut Foo) {
//     // let y: NotCopyable = foo.x;
//     let y = mem::replace(&mut foo.x, NotCopyable());
// }
