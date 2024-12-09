use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufRead;
use std::io::{self, Write};
use std::iter::Iterator;
use std::path::Component::ParentDir;
use std::ptr::copy;

fn read_from_file() -> (Vec<char>, Vec<(i32, i32)>, i32, i32) {
    let mut file = File::open("inputs");
    let result: (Vec<char>, Vec<(i32, i32)>, i32, i32) = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            let mut chars: Vec<char> = Vec::new();
            let mut res: Vec<(i32, i32)> = Vec::new();
            let mut height: i32 = 0;
            let mut width: i32 = 0;
            for line in lines {
                if let Ok(line) = line {
                    if width == 0 {
                        width = line.len() as i32;
                    }
                    let mc = line.chars().collect::<Vec<char>>();
                    mc.iter().enumerate().for_each(|(i, c)| {
                        if *c != '.'  {
                            chars.push(*c);
                            res.push( (i as i32, height));
                        }
                    });
                    height += 1;
                }
            }
            (chars, res, width, height)
        }
        Err(e) => panic!("Cannot process file: {}", e),
    };
    result
}


fn do_task_one(chars: Vec<char>, inputs: &Vec<(i32, i32)>, width: i32, height: i32) {
    let mut sum = 0;
    let mut seen: Vec<((i32, i32),(i32, i32))> = Vec::new();
    let mut found: HashSet<(i32, i32)> = HashSet::new();
    for i in 0..inputs.len() {
        for j in i+1..inputs.len() {
            if chars[i] == chars[j] {
                //println!("init {:?} {:?}", inputs[i], inputs[j]);
                let p1 = inputs[i];
                let p2 = inputs[j];


                let dx = p2.0 - p1.0;
                let dy = p2.1 - p1.1;

                let point1: (i32, i32) = (p1.0 - dx, p1.1 - dy);
                let point2: (i32, i32) = (p2.0 + dx, p2.1 + dy);

                println!("{:?} {:?} {:?} {:?} {:?}", chars[i], p1, p2, point1, point2);

                if point1.0 >= 0 && point1.0 < width && point1.1 >= 0 && point1.1 < height {
                    println!("{:?} {:?} {:?} {:?}", chars[i], p1, p2, point1);
                    found.insert(point1);
                }

                if point2.0 >= 0 && point2.0 < width && point2.1 >= 0 && point2.1 < height {
                    println!("{:?} {:?} {:?} {:?}", chars[i], p1, p2, point2);
                    found.insert(point2);
                }
            }
        }
    }

    println!("{:?}",  found.len());
}

fn do_task_two(chars: Vec<char>, inputs: &Vec<(i32, i32)>, width: i32, height: i32) {
    let mut sum = 0;
    let mut seen: Vec<((i32, i32),(i32, i32))> = Vec::new();
    let mut found: HashSet<(i32, i32)> = HashSet::new();
    for i in 0..inputs.len() {
        for j in i+1..inputs.len() {
            if chars[i] == chars[j] {
                //println!("init {:?} {:?}", inputs[i], inputs[j]);
                let p1 = inputs[i];
                let p2 = inputs[j];


                let dx = p2.0 - p1.0;
                let dy = p2.1 - p1.1;

                println!("{:?} {:?}", p1, p2);
                for h in 0..height {
                    for w in 0..width {

                        if (p1.0*(p2.1-h)+p2.0*(h - p1.1)+w*(p1.1-p2.1)).abs() == 0 {
                            println!("{:?} {:?} {:?} {:?}", chars[i], p1, p2, (w,h));
                            found.insert((w,h));
                        }
                    }
                }
            }
        }
    }

    println!("{:?} {:?}",  found.len(), found);
}

fn main() {
    let inputs = read_from_file();

    println!("{:?}", inputs);
    //do_task_one(inputs.0, &inputs.1, inputs.2, inputs.3);
    do_task_two(inputs.0, &inputs.1, inputs.2, inputs.3);
}
