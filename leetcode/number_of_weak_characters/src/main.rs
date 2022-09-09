struct Solution {}

impl Solution {
    pub fn number_of_weak_characters(mut properties: Vec<Vec<i32>>) -> i32 {
        let mut count = 0;

        properties.sort_by_key(|vec| (-vec[0], vec[1]));
        let mut max_def = 0;

        for def in properties.iter().map(|vec| vec[1]) {
            if def < max_def {
                count += 1;
            } else {
                max_def = def;
            }
        }

        count
    }
}

fn main() {
    let input1 = vec![vec![5, 5], vec![6, 3], vec![3, 6]];
    let input2 = vec![vec![2, 2], vec![3, 3]];
    let input3 = vec![vec![1, 5], vec![10, 4], vec![4, 3]];
    let input4 = vec![vec![7, 9], vec![10, 7], vec![6, 9], vec![10, 4], vec![7, 5], vec![7, 10]];
    let input5 = vec![vec![10, 1], vec![5, 1], vec![7, 10], vec![4, 1], vec![5, 9], vec![6, 9], vec![7, 2], vec![1, 10]];

    println!("{:?} -> {}", input1, Solution::number_of_weak_characters(input1.clone()));
    println!("{:?} -> {}", input2, Solution::number_of_weak_characters(input2.clone()));
    println!("{:?} -> {}", input3, Solution::number_of_weak_characters(input3.clone()));
    println!("{:?} -> {}", input4, Solution::number_of_weak_characters(input4.clone()));
    println!("{:?} -> {}", input5, Solution::number_of_weak_characters(input5.clone()));
}






