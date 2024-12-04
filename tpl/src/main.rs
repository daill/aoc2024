use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::absolute;

fn read_from_file() -> (Vec<i32>, Vec<i32>){
    let mut file = File::open("inputs");
    let result:(Vec<i32>, Vec<i32>) = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            let mut left: Vec<i32> = Vec::new();
            let mut right: Vec<i32> = Vec::new();
            for line in lines {
                if let Ok(line) = line {
                    let mut splitted = line.split("   ");
                    left.push(splitted.next().unwrap().parse::<i32>().unwrap());
                    right.push(splitted.next().unwrap().parse::<i32>().unwrap());
                }
            }
            (left, right)
        }
        Err(e) => panic!("Cannot process file: {}", e),
    };
    result
}


fn do_task_one() {

}

fn do_task_two(inputs: &mut (Vec<i32>, Vec<i32>)) {
}

fn main() {
    let mut inputs = read_from_file();
    //do_task_one();
    //do_task_two();

    println!("{:?}", inputs);
}
