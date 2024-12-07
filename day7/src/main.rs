use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufRead;
use std::io::{self, Write};
use std::iter::Iterator;
use std::ptr::copy;

fn read_from_file() -> Vec<(i64, Vec<i64>)> {
    let mut file = File::open("inputs");
    let result: Vec<(i64, Vec<i64>)> = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            let mut res: Vec<(i64, Vec<i64>)> = Vec::new();
            for line in lines {
                if let Ok(line) = line {
                    let splitted: Vec<&str> = line.split(": ").collect();
                    let target: i64 = splitted[0].trim().parse().unwrap();
                    let mut numbers: Vec<i64> = Vec::new();
                    for number in splitted[1].split(" ") {
                        numbers.push(number.parse().unwrap());
                    }
                    res.push((target, numbers));
                }
            }
            res
        }
        Err(e) => panic!("Cannot process file: {}", e),
    };
    result
}

fn do_calc(target: i64, numbers: &Vec<i64>, num: usize, operator: char, sequence: &mut Vec<char>) -> bool {
    if num == numbers.len()-1 {
        println!("{:?} {:?}", target, sequence);
        if target == numbers[numbers.len()-1] {
            println!("true {:?}", true);
            true
        } else {
            false
        }
    } else {
        let mut ret = false;
        let mut calc_res = (target - numbers[num]) as f64;
        println!("{:?} {:?} {:?} {:?}", target, calc_res, numbers[num], '-');

        if calc_res > 0.0 {
            sequence.push('-');
            ret = do_calc(calc_res as i64, numbers, num + 1, operator, sequence);
            if ret {
                return true
            }
        }

        sequence.pop();

        calc_res = (target as f64) / (numbers[num] as f64);
        println!("{:?} {:?} {:?} {:?}", target, calc_res, numbers[num], '/');

        if calc_res.fract() == 0.0 {
            sequence.push('/');
            ret = do_calc(calc_res as i64, numbers, num + 1, operator, sequence);
            if ret {
                return true
            }
        }
        false
    }
}

fn do_calc_two(target: i64, numbers: &Vec<i64>, num: usize, operator: char, sequence: &mut Vec<char>) -> bool {
    if num >= numbers.len()-1 {
        println!("{:?} {:?}", target, sequence);
        if target == numbers[numbers.len()-1] {
            println!("true {:?}", true);
            true
        } else {
            false
        }
    } else {
        let mut ret = false;
        let mut calc_res = (target - numbers[num]) as f64;
        println!("{:?} {:?} {:?} {:?}", target, calc_res, numbers[num], '-');

        if calc_res > 0.0 {
            sequence.push('-');
            ret = do_calc_two(calc_res as i64, numbers, num + 1, operator, sequence);
            if ret {
                return true
            }
        }

        sequence.pop();

        calc_res = (target as f64) / (numbers[num] as f64);
        println!("{:?} {:?} {:?} {:?}", target, calc_res, numbers, '/');

        if calc_res.fract() == 0.0 {
            sequence.push('/');
            ret = do_calc_two(calc_res as i64, numbers, num + 1, operator, sequence);
            if ret {
                return true
            }
        }

        sequence.pop();


        let mstr = target.to_string().chars().collect::<Vec<char>>();
        let nstr = numbers[num].to_string().chars().collect::<Vec<char>>();

        if mstr.len() > nstr.len() {
            println!("{:?} {:?} {:?} {:?}",mstr, mstr.len(), nstr.len(), nstr);
            println!("------{:?} {:?} {:?}", target, &mstr[(mstr.len()-nstr.len())..], nstr);
            if &mstr[(mstr.len()-nstr.len())..] == nstr {
                let tstr = String::from_iter(&mstr[..(mstr.len()-numbers[num].to_string().len())]);
                let pstr: i64 = tstr.parse::<i64>().unwrap();
                println!("{:?} {:?} {:?} {:?} {:?}", target, pstr, numbers[num], numbers, '|' );

                sequence.push('|');
                ret = do_calc_two(pstr, numbers, num + 1, operator, sequence);
                if ret {
                    return true
                }
            }
        }


        false
    }
}


fn do_task_one(inputs: &Vec<(i64, Vec<i64>)>) {
    let mut sum = 0;
    for (target, numbers) in inputs {
        let mut nums_rev = numbers.clone();
        let mut sequence: Vec<char> = Vec::new();
        nums_rev.reverse();
        println!("{:?} {:?}", target, numbers);
        let ret = do_calc(*target, &nums_rev, 0, ' ', &mut sequence);
        if ret {
            sum += target;
        }
    }
    println!("{:?}", sum);
}

fn do_task_two(inputs: &Vec<(i64, Vec<i64>)>) {
    let mut sum = 0;
    for (target, numbers) in inputs {
        let mut nums_rev = numbers.clone();
        let mut sequence: Vec<char> = Vec::new();
        nums_rev.reverse();
        println!("{:?} {:?}", target, numbers);
        let ret = do_calc_two(*target, &nums_rev, 0, ' ', &mut sequence);
        if ret {
            println!("added {:?}", target);

            sum += target;
        }
    }
    println!("{:?}", sum);
}

fn main() {
    let inputs = read_from_file();

    println!("{:?}", inputs);
    //do_task_one(&inputs);
    do_task_two(&inputs);
}
