use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::absolute;

fn read_from_file() -> (Vec<i32>, Vec<i32>){
    let mut file = File::open("test");
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


fn do_task_one(inputs: &mut (Vec<i32>, Vec<i32>)) {
    let mut left = &mut inputs.0;
    let mut right = &mut inputs.1;
    left.sort();
    right.sort();
    let result:Vec<i32> = left.iter().enumerate().map(|(i, l)| (l-right[i]).abs() ).collect();
    let sum:i32 = result.iter().sum();
    println!("{:?}", sum);
}

fn do_task_two(inputs: &mut (Vec<i32>, Vec<i32>)) {
    // just sum up the occurences
    // we first sort and than simply count by iterating
    let mut left = &mut inputs.0;
    let mut right = &mut inputs.1;
    left.dedup();
    let mut cnt: HashMap<i32, i32> = HashMap::new();
    right.iter().map(|r| cnt.entry(r.clone()).or_insert(r.clone()) =+ 1);

    println!("{:?}", cnt);
}

fn main() {
    let mut inputs = read_from_file();
    //do_task_one(&mut inputs);
    do_task_two(&mut inputs);





    println!("{:?}", inputs);
}
