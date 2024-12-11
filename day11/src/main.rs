use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufRead;
use std::io::{self, Write};
use std::iter::Iterator;
use std::path::Component::ParentDir;
use std::ptr::copy;
use std::str::FromStr;

fn read_from_file() -> Vec<String> {
    let mut file = File::open("inputs");
    let result: Vec<String> = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            let mut result: Vec<String> = Vec::new();

            for line in lines {
                if let Ok(line) = line {
                    line.split(" ")
                        .collect::<Vec<&str>>()
                        .iter()
                        .for_each(|x| result.push((x.to_string())));
                }
            }
            result
        }
        Err(e) => panic!("Cannot process file: {}", e),
    };
    result
}

fn do_task_one(inputs: Vec<String>) {
    // idea: just copy every blink
    let mut source = inputs.clone();
    let mut target: Vec<String> = Vec::new();
    for _b in 0..25 {
        target.clear();
        for i in 0..source.len() {
            let mut a = source[i].clone();

            if a == "0" {
                a = "1".to_string();
            } else if a.len() % 2 == 0 {
                let mut num_str = &a[..a.len() / 2];
                let mut num: i64 = FromStr::from_str(num_str).unwrap();
                target.push(num.to_string());
                num_str = &a[a.len() / 2..];
                num = FromStr::from_str(num_str).unwrap();
                a = num.to_string();
            } else {
                let num: i64 = FromStr::from_str(a.as_str()).unwrap();
                a = (num * 2024).to_string();
            }

            target.push(a);
        }
        //println!("{:?}", target);
        source = target.clone();
    }
    println!("{:?}", source.len());
}

fn do_task_two(inputs: Vec<String>) {
    // idea: just copy every blink
    let mut source: HashMap<String, i64> = HashMap::new();
    for i in 0..inputs.len() {
        source.insert(inputs[i].clone(), 1);
    }
    let mut target: HashMap<String, i64> = HashMap::new();

    for _b in 0..75 {
        for (num, cnt) in source.iter() {
            if num == "0" {
                target.insert(
                    "1".to_string(),
                    *cnt + target.get("1").or(Option::from(&0)).unwrap(),
                );
            } else if num.len() % 2 == 0 {
                let mut num_str = &num[..num.len() / 2];
                let mut new_num: i64 = FromStr::from_str(num_str).unwrap();
                target.insert(
                    new_num.to_string(),
                    *cnt + target
                        .get(&new_num.to_string())
                        .or(Option::from(&0))
                        .unwrap(),
                );
                num_str = &num[num.len() / 2..];
                new_num = FromStr::from_str(num_str).unwrap();
                target.insert(
                    new_num.to_string(),
                    *cnt + target
                        .get(&new_num.to_string())
                        .or(Option::from(&0))
                        .unwrap(),
                );
            } else {
                let num: i64 = FromStr::from_str(num.as_str()).unwrap();
                target.insert(
                    (num * 2024).to_string(),
                    *cnt + target
                        .get(&(num * 2024).to_string())
                        .or(Option::from(&0))
                        .unwrap(),
                );
            }
        }
        //println!("{:?}", target);
        source = target.clone();
        target.clear();
    }
    let mut sum = 0;
    source.iter().for_each(|(str, num)| sum += num);
    println!("{:?}", sum);
}

fn main() {
    let mut inputs = read_from_file();

    println!("{:?}", inputs);
    //do_task_one(inputs);
    do_task_two(inputs);
}
