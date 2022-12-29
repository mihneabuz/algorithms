use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct Solution {}

#[derive(PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
struct Task {
    pub processing_time: i32,
    pub initial_idx: usize,
    pub enqueue_time: i32,
}

impl Solution {
    pub fn get_order(tasks: Vec<Vec<i32>>) -> Vec<i32> {
        let mut tasks = tasks.into_iter().enumerate().map(|(i, t)| {
            Task { enqueue_time: t[0], processing_time: t[1], initial_idx: i }
        }).collect::<Vec<_>>();

        tasks.sort_by_key(|task| -task.enqueue_time);

        let mut res = vec![];
        let mut current_time = 0;
        let mut available_tasks = BinaryHeap::new();

        while tasks.len() > 0 || available_tasks.len() > 0 {
            while tasks.len() > 0 && tasks.last().unwrap().enqueue_time <= current_time {
                available_tasks.push(Reverse(tasks.pop().unwrap()));
            }

            if let Some(Reverse(task)) = available_tasks.pop() {
                res.push(task.initial_idx as i32);
                current_time += task.processing_time;
            } else {
                current_time = tasks.last().unwrap().enqueue_time;
            }
        }

        return res;
    }
}

fn main() {
    let input = vec![
        vec![1, 2],
        vec![2, 4],
        vec![3, 2],
        vec![4, 1],
    ];

    println!("{:?}", Solution::get_order(input));
}
