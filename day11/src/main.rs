use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufRead;
use std::io::{self, Write};
use std::iter::Iterator;
use std::path::Component::ParentDir;
use std::ptr::copy;

fn read_from_file() -> Vec<Vec<i32>> {
    let mut file = File::open("test");
    let result: Vec<Vec<i32>> = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            let mut result: Vec<Vec<i32>> = Vec::new();

            let mut nums: Vec<i32> = Vec::new();
            for line in lines {
                if let Ok(line) = line {
                    line.chars().for_each(|x| nums.push(x.to_string().parse::<i32>().unwrap()));

                }
                result.push(nums.clone());
                nums.clear();
            }
            result
        }
        Err(e) => panic!("Cannot process file: {}", e),
    };
    result
}

fn do_task_one() {


}

fn do_task_two() {

}

fn main() {
    let mut inputs = read_from_file();

    println!("{:?}", inputs);
    do_task_one();
    //do_task_two(&inputs);
}
