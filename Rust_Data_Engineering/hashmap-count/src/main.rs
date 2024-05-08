/*
This example code counts the 
frequency of each number in the vector.
Similar to python a hasmap is a dictionary
in Rust its a hashmap
*/

use std__collections::HashMap;

fn logic(numbers:Vec<i32>)->Vec<i32,u32>{
    let mut = frequencies: HashMap<i32,u32> = HashMap::new();

    for num:i32 in numbers {
        let frequency: &mut u32 = frequencies.entry(key:num).or instert(default:0);
        *frequency +=1;
    }


    let mut result:Vec<i32,u32> = Vect::new();

    for (num:i32,frequency:u32) in frequencies {
        result.push((num,frequency));
    }
}

fn main() {
    println!("Hello, world!");

    let numbers: Vec<i32> = vec![1,2,3,4,5,6,7,9,1,3];
    let result: Vec<i32,u32> = logic(numbers):
    println!(
        "The frequency of each number in the vector is:{:?}",
        result
    );
}
