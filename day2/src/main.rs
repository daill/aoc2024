use itertools::Itertools;
use std::collections::HashMap;
use std::env::temp_dir;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::absolute;
use std::path::Component::ParentDir;

fn read_from_file() -> (Vec<Vec<i32>>) {
    let mut file = File::open("inputs");
    let result: (Vec<Vec<i32>>) = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            let mut result: Vec<Vec<i32>> = Vec::new();
            for line in lines {
                if let Ok(line) = line {
                    let splitted = line
                        .split(" ")
                        .map(|x| x.parse().unwrap())
                        .collect::<Vec<i32>>();
                    result.push(splitted);
                }
            }
            (result)
        }
        Err(e) => panic!("Cannot process file: {}", e),
    };
    result
}

fn do_task_one(inputs: &mut Vec<Vec<i32>>) {
    let mut count = inputs.len();
    for reactor in inputs {
        let mut valid = true;
        let mut up = true;
        reactor.iter().enumerate().for_each(|(i, x)| {
            if let Some(y) = reactor.get(i + 1) {
                println!("{}, {}", x, y);
                let mut temp = true;

                if (x - y).abs() < 1 || (x - y).abs() > 3 {
                    valid = false;
                }

                temp = x > y;

                if i == 0 {
                    up = temp;
                } else {
                    if up != temp {
                        valid = false;
                    }
                }
            }
        });
        if !valid {
            count -= 1;
            println!("{:?} false", reactor);
        }
    }
    println!("{}", count);
}

fn check_vector(reactor: &mut Vec<i32>, skipped: bool) -> bool {
    let mut i = 0;
    let mut n = i+1;
    let mut dir = 0;
    while i < reactor.len() {
        let mut x = reactor.get(i).unwrap();
        if let Some(y) = reactor.get(n) {
            //println!("see {:?} {:?}", x, y);
            let diff = (x - y).abs();
            let mut tmp_dir = 1;
            if x > y {
                tmp_dir = -1;
            }

            if i == 0 {
                dir = tmp_dir;
            }

            if dir != tmp_dir {
                if (skipped) {
                    println!("problem {:?} {:?} {:?} {:?} ", x, y, i, n);
                    return false;
                }

                println!("problem {:?} {:?} {:?} {:?} remove {:?}", x, y, i, n, i);
                reactor.remove(i);
                return check_vector(reactor, true);
            }

            if diff == 0  {
                if (skipped) {
                    println!("problem {:?} {:?} {:?} {:?} ", x, y, i, n);
                    return false;
                }

                println!("problem {:?} {:?} {:?} {:?} remove {:?}", x, y, i, n, i);
                reactor.remove(i);
                return check_vector(reactor, true);
            }

            if  !(1..=3).contains(&diff) {
                if (skipped) {
                    println!("problem {:?} {:?} {:?} {:?} ", x, y, i, n);
                    return false;
                }

                println!("problem {:?} {:?} {:?} {:?} remove {:?}", x, y, i, n, i);
                reactor.remove(i);
                return check_vector(reactor, true);
            }

            i = n;
            n += 1;
        } else {

            break;
        }
    }
    return true;

    }

fn do_task_two(inputs: &mut Vec<Vec<i32>>) {
    let mut count = 0;
    for reactor in inputs {
        let res = check_vector(reactor, false);
        println!("{:?} {:?}", res, reactor);
        if res {
            count +=1;
        }
    }
    println!("{:?}", count);
}


fn main() {
    let mut inputs = read_from_file();
    //do_task_one(&mut inputs);
    do_task_two(&mut inputs);
}
