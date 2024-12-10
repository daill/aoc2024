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

fn search(inputs: &Vec<Vec<i32>>, seen: &mut HashSet<(i32, i32)>, last: (i32, i32), found: &mut Vec<((i32, i32),(i32, i32))>, width: i32, height: i32, start: (i32, i32), t2: bool) {
    // possible ways up, down, left, right
    for i in [(0,-1),(-1,0), (0,1), (1,0)] {
        // up
        let mut new = (last.0+i.0, last.1+i.1);
        if !seen.contains(&new) && (new.0 < width && new.0 >= 0 && new.1 >= 0 && new.1 < height) {

            if (inputs[new.1 as usize][new.0 as usize] - inputs[last.1 as usize][last.0 as usize] == 1) {
                if (inputs[new.1 as usize][new.0 as usize] == 9) {
                    if (!t2 && !found.contains(&(start, new))){
                        println!("{:?}", new);
                        found.push((start,(new.0, new.1)));
                    } else {
                        println!("{:?}", new);
                        found.push((start,(new.0, new.1)));
                    }
                } else {
                    seen.insert(new);
                    search(inputs, seen, new, found, width, height, start, t2);
                    seen.remove(&new);
                }
            }
        }
    }
}

fn do_task_one(inputs: &Vec<Vec<i32>>) {
    let mut seen: HashSet<(i32, i32)>  = HashSet::new();
    let mut found: Vec<((i32, i32),(i32, i32))> = Vec::new();
    for y in 0..inputs.len() {
        let row: &Vec<i32> = inputs.get(y).unwrap();
        for x in 0..row.len() {
            if row[x] == 0 {
                println!("start {:?}", (x as i32,y as i32));
                seen.insert((x as i32,y as i32));
                // found start
                search(inputs, &mut seen, (x as i32,y as i32), &mut found, row.len() as i32, inputs.len() as i32, (x as i32,y as i32), false);
            }
        }
    }

    println!("found: {:?} {:?}", found, found.len());
}

fn do_task_two(inputs: &Vec<Vec<i32>>) {
    let mut seen: HashSet<(i32, i32)>  = HashSet::new();
    let mut found: Vec<((i32, i32),(i32, i32))> = Vec::new();
    for y in 0..inputs.len() {
        let row: &Vec<i32> = inputs.get(y).unwrap();
        for x in 0..row.len() {
            if row[x] == 0 {
                println!("start {:?}", (x as i32,y as i32));
                seen.insert((x as i32,y as i32));
                // found start
                search(inputs, &mut seen, (x as i32,y as i32), &mut found, row.len() as i32, inputs.len() as i32, (x as i32,y as i32), true);
            }
        }
    }

    println!("found: {:?} {:?}", found, found.len());
}

fn main() {
    let mut inputs = read_from_file();

    println!("{:?}", inputs);
    //do_task_one(&inputs);
    do_task_two(&inputs);
}
