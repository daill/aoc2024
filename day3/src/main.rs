use itertools::Itertools;
use std::collections::HashMap;
use std::env::temp_dir;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::absolute;
use std::path::Component::ParentDir;
use regex::Regex;

fn read_from_file() -> String {
    let mut file = File::open("inputs");
    let result: (String) = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            let mut text = String::new();
            for line in lines {
                if let Ok(line) = line {
                    text.push_str(&line);
                }
            }
            text
        }
        Err(e) => panic!("Cannot process file: {}", e),
    };
    result
}

fn do_task_one(input: &String) {
    let re = Regex::new(r"(?m)mul[(](\d{1,3}),(\d{1,3})[)]").unwrap();
    let captures = re.captures_iter(input);
    let mut sum = 0;
    for capture in captures {
        let ext_cap: (&str, [&str; 2]) = capture.extract();
        sum += ext_cap.1[0].parse::<i32>().unwrap() * ext_cap.1[1].parse::<i32>().unwrap();
    }
    println!("{:?}", sum);

}


fn do_task_two(input: &String) {
    let re = Regex::new(r"(don't\(\)|do\(\))()|mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let captures = re.captures_iter(input);
    let mut sum = 0;
    let mut add = true;
    for capture in captures {
        println!("{:?}", capture);
        let ext_cap: (&str, [&str; 2]) = capture.extract();
        println!("{:?}", ext_cap);

        if ext_cap.1[0].eq("don't()")  {
            println!("{:?}", ext_cap.1);
            add = false;
            continue;
        }
        if ext_cap.1[0].eq("do()"){
            add = true;
            continue;
        }
        if add {
            println!("{:?} {:?}", ext_cap.1[0], ext_cap.1[1]);
            sum += ext_cap.1[0].parse::<i32>().unwrap() * ext_cap.1[1].parse::<i32>().unwrap();
        }
        //println!("{:?}", ext_cap);
    }
    println!("{:?}", sum);
}


fn main() {
    let mut inputs = read_from_file();
    println!("{:?}", inputs);
    //do_task_one(&inputs);
    do_task_two(&inputs);
}
