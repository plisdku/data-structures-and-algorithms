#[allow(unused_imports)]
use maximum_subsequence_sum::{
    generate_random_integers, max_sub_sum_cubic, max_sub_sum_linear, max_sub_sum_nlogn,
    max_sub_sum_quadratic,
};

fn main() {
    println!("Maximum subsequence sum!");

    let functions = [
        max_sub_sum_cubic,
        max_sub_sum_quadratic,
        max_sub_sum_nlogn,
        max_sub_sum_linear,
    ];

    // let r = generate_random_integers(10, 3, 4);

    // let r = vec![1, 2, 3, -100, 4, 5];
    let r = [1i32, 2, 3, -100, 4, 5];

    for fun in functions {
        let s = fun(&r);

        println!("Max subsequence sum = {}", s);
    }

    for rr in r {
        println!("{}", rr);
    }
}
