use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufRead;
use std::io::{self, Write};
use std::iter::Iterator;
use std::path::Component::ParentDir;
use std::ptr::copy;

fn read_from_file_t1() -> Vec<Vec<String>> {
    let mut file = File::open("inputs");
    let result: Vec<Vec<String>> = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            let mut result = Vec::new();
            for line in lines {
                let mut chars: Vec<String> = Vec::new();

                let mut fileid: u32 = 0;
                if let Ok(line) = line {
                    for (i, c) in line.chars().enumerate() {
                        let num:i32 = c.to_digit(10).unwrap() as i32;
                        if i%2 == 0 {
                            for n in 0..num {
                                chars.push(fileid.to_string());
                            }
                            fileid += 1;
                        } else {
                            for n in 0..num {
                                chars.push(".".to_string());
                            }
                        }
                    };

                }
                result.push(chars);
            }
            result
        }
        Err(e) => panic!("Cannot process file: {}", e),
    };
    result
}

fn read_from_file_t2() -> Vec<(i32, i32, i32, bool)> {
    let mut file = File::open("inputs");
    let result: Vec<(i32, i32, i32, bool)> = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            let mut result = Vec::new();
            let mut fileid: u32 = 0;
            for line in lines {
                let chars = line.unwrap().chars().collect::<Vec<char>>();
                for (i, c) in chars.iter().enumerate() {
                    let num:i32 = c.to_digit(10).unwrap() as i32;
                    let mut entry: (i32, i32, i32, bool) = (i as i32, fileid as i32, num, false);
                    if i%2 == 0 {
                        entry.3 = true;
                        fileid +=1;
                    } else {
                        entry.1 = 0;
                    }
                    result.push(entry);
                }
            }
            result
        }
        Err(e) => panic!("Cannot process file: {}", e),
    };
    result
}


fn do_task_one(inputs: &mut Vec<Vec<String>>) {
    for input in inputs {
        let mut back = input.len()-1;
        for i in 0..input.len() {
            if input[i] == "." {
                while input[back] == "." {
                    back -= 1;
                }
                if i > back {
                    break;
                }
                input.swap(i, back);
                //println!("{:?} {} {} {} {}", input, i, input[i], back, input[back]);
                back-=1;
            }
        }
        println!("{:?}", input);

        // calc sum
        let mut sum: i64 = 0;
        for i in 0..input.len() {
            if input[i] == "."{
                break;
            }
            sum += (i as i64) * input[i].parse::<i64>().unwrap();
        }

        println!("{:?}", sum);
    }
}

// (i32, i32, i32, bool) -> id, fileid, length, number (false) or file (true)
fn do_task_two(entries: &mut Vec<(i32, i32, i32, bool)>) {
    let mut vec: Vec<(i32, i32, bool)> = Vec::new();
    let mut back = entries.len()-1;
    for i in 0..entries.len() {
        let mut entry = entries[i];
        //println!("{:?}", entry);
        if !entry.3 {
            while entry.2 > 0 {
                // search for possible entry
                while back > i {
                    let mut be = entries[back];
                    if be.3 {
                        if be.2 >= 0 && be.2 <= entry.2 {
                            entry.2 -= be.2;
                            vec.push((be.1, be.2, true));
                            be.1 = 0;
                            be.3 = false;
                            entries[back] = be;
                            back = entries.len()-1;
                            //println!("{:?} {:?} {:?} {:?} {:?} {:?}", i, back, be, entry, vec, entries);

                            continue;
                        }
                    }
                    back -= 1;
                }
                if i >= back {
                    vec.push((entry.1, entry.2, false));
                    break;
                }
            }
            back = entries.len()-1;
        } else {
            vec.push((entry.1, entry.2, true));
            //println!("{:?}", vec);
        }

    }

    let mut sum: i64 = 0;
    let mut index = 0;
    for e in vec {

        for f in 0..e.1 {
            if e.2 {
                sum += (e.0 * index) as i64;
            }
            index += 1;
            //println!("{} {:?}", index, e);
        }
    }

    println!("{:?}", sum);
}

// digits alternate between indicating length and length of free space
// 12345 -> 1 block file (id 0), 2 block free, 3 block file (id 1) ..
// 0..111....22222 (. free space, number the file id, count of number is length of file)
// move one file at a time
fn main() {
    let mut inputs = read_from_file_t2();

    println!("{:?}", inputs);
    //do_task_one(&mut inputs);
    do_task_two(&mut inputs);
}
