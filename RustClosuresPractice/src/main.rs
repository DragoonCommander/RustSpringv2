// Task 1
/*
fn main() {
    let operation = |a: i32, b: i32| a * b;
    println!("Result: {}", operation(10, 5)); // Output: Result: 50
}
*/

// Task 2
/*
fn track_changes() {
    let mut tracker = 0;
    let mut update = || {
        tracker += 1;
        println!("Update count: {}", tracker);
    };

    update(); // Update count: 1
    update(); // Update count: 2
}

fn main() {
    track_changes();
}
*/

// Task 3
/*
// Using map and collect
fn process_vector_with_map<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    vec.into_iter().map(f).collect()
}

// Using for loop
fn process_vector_with_for_loop<F>(vec: Vec<i32>, f: F) -> Vec<i32>
where
    F: Fn(i32) -> i32,
{
    let mut result = Vec::new();
    for x in vec {
        result.push(f(x));
    }
    result
}

fn main() {
    let numbers = vec![1, 2, 3];

    let doubled = process_vector_with_map(numbers.clone(), |x| x * 2);
    let replaced = process_vector_with_for_loop(numbers, |x| if x > 2 { 0 } else { x });

    println!("Doubled: {:?}", doubled);   // Output: [2, 4, 6]
    println!("Replaced: {:?}", replaced); // Output: [1, 2, 0]
}
*/

// Task 5

use std::{thread, time::Duration};

struct ComputeCache<T>
where
    T: Fn() -> String,
{
    computation: T,
    value: Option<String>,
}

impl<T> ComputeCache<T>
where
    T: Fn() -> String,
{
    fn new(computation: T) -> Self {
        ComputeCache {
            computation,
            value: None,
        }
    }

    fn get_result(&mut self) -> String {
        match &self.value {
            Some(v) => {
                println!("Retrieved from cache instantly!");
                v.clone()
            }
            None => {
                let result = (self.computation)();
                self.value = Some(result.clone());
                result
            }
        }
    }
}

fn main() {
    let mut cache = ComputeCache::new(|| {
        println!("Computing (this will take 2 seconds)...");
        thread::sleep(Duration::from_secs(2));
        "Hello, world!".to_string()
    });

    println!("First call:");
    println!("Result: {}", cache.get_result());

    println!("\nSecond call:");
    println!("Result (cached): {}", cache.get_result());
}

