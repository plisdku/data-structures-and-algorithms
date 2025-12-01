use rand::Rng;

#[allow(dead_code)]
fn generate_random_integers(n: usize, min: i32, max: i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(min..=max)).collect()
}

#[allow(dead_code)]
/// Returns the maximum sum of any contiguous subsequence in the array.
///
/// Uses a cubic-time O(n³) algorithm that examines all possible subsequences.
/// Returns 0 if all subsequences have negative sums.
fn max_sub_sum_cubic(array: &[i32]) -> i32 {
    let mut max_sum: i32 = 0;

    // for start in 0..array.len() {
    //     for end in start..array.len() {
    //         let mut sum: i32 = 0;
    //         for ii in start..=end {
    //             sum += array[ii];
    //         }
    //         if sum > max_sum {
    //             max_sum = sum;
    //         }
    //     }
    // }

    for start in 0..array.len() {
        for end in start..array.len() {
            let mut sum: i32 = 0;
            for item in &array[start..=end] {
                sum += item;
            }
            if sum > max_sum {
                max_sum = sum;
            }
        }
    }

    max_sum
}

#[allow(dead_code)]
fn max_sub_sum_quadratic(array: &[i32]) -> i32 {
    let mut max_sum: i32 = 0;

    for start in 0..array.len() {
        let mut sum: i32 = 0;

        // for end in start..array.len() {
        //     sum += array[end];

        for item in &array[start..] {
            sum += item;

            if sum > max_sum {
                max_sum = sum;
            }
        }
    }

    max_sum
}

#[allow(dead_code)]
/// Divide-and-conquer implementation of maximum subsequence sum
fn max_sub_sum_nlogn(array: &[i32]) -> i32 {
    if array.is_empty() {
        0
    } else if array.len() == 1 {
        array[0]
    } else {
        let center: usize = array.len() / 2;

        let max_sub_sum_left: i32 = max_sub_sum_nlogn(&array[..center]);
        let max_sub_sum_right: i32 = max_sub_sum_nlogn(&array[center..]);

        let mut right_center_sum: i32 = 0;
        let mut left_center_sum: i32 = 0;

        let mut temp: i32 = 0;

        for item in array[..center].iter().rev() {
            temp += item;
            if temp > left_center_sum {
                left_center_sum = temp;
            }
        }

        // for ii in (0..center).rev() {
        //     temp += array[ii];
        //     if temp > left_center_sum {
        //         left_center_sum = temp;
        //     }
        // }

        temp = 0;
        for item in &array[center..] {
            temp += item;
            if temp > right_center_sum {
                right_center_sum = temp;
            }
        }

        // temp = 0;
        // for ii in center..array.len() {
        //     temp += array[ii];
        //     if temp > right_center_sum {
        //         right_center_sum = temp;
        //     }
        // }

        max_sub_sum_left.max(max_sub_sum_right.max(left_center_sum + right_center_sum))
    }
}

#[allow(dead_code)]
fn max_sub_sum_linear(array: &[i32]) -> i32 {
    let mut this_sum: i32 = 0;
    let mut max_sum: i32 = 0;

    // for ii in 0..array.len() {
    //     this_sum += array[ii];

    for item in array {
        this_sum += item;

        if this_sum > max_sum {
            max_sum = this_sum;
        }
        if this_sum < 0 {
            this_sum = 0;
        }
    }

    max_sum
}

fn main() {
    println!("Maximum subsequence sum!");

    // let r = generate_random_integers(10, 3, 4);

    // let r = vec![1, 2, 3, -100, 4, 5];
    let r = [1i32, 2, 3, -100, 4, 5];

    // let s = max_sub_sum_cubic(&r);

    // let s = max_sub_sum_quadratic(&r);

    // let s = max_sub_sum_nlogn(&r);
    let s = max_sub_sum_linear(&r);

    println!("Max subsequence sum = {}", s);

    for rr in r {
        println!("{}", rr);
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_cubic_simple() {
        use crate::max_sub_sum_cubic;
        let r = [1, 2, 3, -100, 4, 5];
        assert_eq!(max_sub_sum_cubic(&r), 9);
    }

    #[test]
    fn test_quadratic_simple() {
        use crate::max_sub_sum_quadratic;
        let r = [1, 2, 3, -100, 4, 5];
        assert_eq!(max_sub_sum_quadratic(&r), 9);
    }

    #[test]
    fn test_nlogn_simple() {
        use crate::max_sub_sum_nlogn;
        let r = [1, 2, 3, -100, 4, 5];
        assert_eq!(max_sub_sum_nlogn(&r), 9);
    }

    #[test]
    fn test_linear_simple() {
        use crate::max_sub_sum_linear;
        let r = [1, 2, 3, -100, 4, 5];
        assert_eq!(max_sub_sum_linear(&r), 9);
    }
}
