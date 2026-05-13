
    /* Given a list of integers, use a vector and return the median (when sorted, the value in the middle position) and mode (the value that occurs most often; a hash map will be helpful here) of the list
*/

use std::collections::HashMap;

fn main() {
    let numbers = vec![2, 5, 1, 8, 2, 9, 3, 2, 7];
    
    let median = find_median(&numbers);
    let mode = find_mode(&numbers);
    
    println!("List: {:?} \n Median: {} \n Mode: {}", numbers, median, mode);
}

fn find_median(numbers: &Vec<i32>) -> f64 {
    let mut sorted = numbers.clone();
    sorted.sort();
    
    let len = sorted.len();
    if len % 2 == 0 {
        
        (sorted[len/2 - 1] + sorted[len/2]) as f64 / 2.0
    } else {
        
        sorted[len/2] as f64
    }
}

fn find_mode(numbers: &Vec<i32>) -> i32 {
    let mut frequency = HashMap::new();
    
    for &num in numbers {
        *frequency.entry(num).or_insert(0) += 1;
    }
    
    
    let mut mode = numbers[0];
    let mut max_count = 0;
    
    for (&num, &count) in &frequency {
        if count > max_count {
            max_count = count;
            mode = num;
        }
    }
    
    mode
}