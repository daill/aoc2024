use std::cmp::max;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufRead;
use std::io::{self, Write};
use std::iter::Iterator;
use std::path::Component::ParentDir;
use std::ptr::copy;
use std::str::FromStr;
use std::sync::atomic::fence;
use regex::Regex;

fn read_from_file() -> Vec<(String, (i64, i64))> {
    let mut file = File::open("inputs");
    let result: Vec<(String, (i64, i64))> = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            let mut result: Vec<(String, (i64, i64))> = Vec::new();
            let butre = Regex::new(r"(?m)^Button\W(\w+)\W+(\w+)(\S)(\d+),\W+(\w+)(\S)(\d+)$").unwrap();
            let prre = Regex::new(r"(?m)^Prize:\W(\w+)=(\d+),\W+(\w+)=(\d+)$").unwrap();

            for line in lines {
                if let Ok(line) = line {
                    if line.starts_with("Button") {
                        butre.captures_iter((&line).as_str()).for_each(|c| {
                            let mut but = (c[1].to_string(),(c[4].parse::<i64>().unwrap(),c[7].parse::<i64>().unwrap()));
                            if &c[3] == "-" {
                                but.1.0 = -but.1.0;
                            }
                            if &c[6] == "-" {
                                but.1.1 = -but.1.1;
                            }
                            result.push(but);
                        });
                    }
                    if line.starts_with("Prize") {
                        prre.captures_iter((&line).as_str()).for_each(|c| {
                            let prize = ("P".to_string(),(c[2].parse::<i64>().unwrap(), c[4].parse::<i64>().unwrap()));
                            result.push(prize);
                        });

                    }
                }

            }
            result
        }
        Err(e) => panic!("Cannot process file: {}", e),
    };
    result
}


fn do_task_one(inputs: &Vec<(String, (i64, i64))>) {
    let mut sum = 0;
    for i in (0..inputs.len()).step_by(3) {
        println!("{:?} {:?} {:?}", &inputs[i], &inputs[i+1], &inputs[i+1]);
        let mut ba = &inputs[i];
        let mut bb = &inputs[i+1];
        let p = &inputs[i+2];

        let mut x = 0;
        let mut y = 0;
        let mut cost = 0;
        let mut old = 0;

        let mut acnt = 1;
        let mut cnt = 0;
        let mut found = false;
        let mut gcnt = max(max(p.1.0/bb.1.0, p.1.1/bb.1.1), max(p.1.0/ba.1.0, p.1.1/ba.1.1));

        // generate cost matrix
        loop {
            // a x 1 ... a x rest
            // b rest ... b x 1

            cnt += 1;
            cost = 0;

            x = acnt*ba.1.0;
            y = acnt*ba.1.1;
            cost = acnt*3;

            while x < p.1.0 && y < p.1.1 {
                x += bb.1.0;
                y += bb.1.1;
                cost += 1;
            }


            if (x == p.1.0 && y == p.1.1) && (old == 0 || cost < old)  {
                old = cost;
                println!("{:?} {:?}", acnt, cnt)

            }

            acnt += 1;
            x = 0;
            y = 0;

            if cnt > gcnt {
                break;

            }

        }
        if old != 0 {
            sum += old;
            println!("{:?}", old);
        }

    }
    println!("{:?}", sum);
}

fn do_task_two(inputs: &Vec<(String, (i64, i64))>) {
    let mut sum = 0;
    for i in (0..inputs.len()).step_by(3) {
        //println!("{:?} {:?} {:?}", &inputs[i], &inputs[i+1], &inputs[i+1]);
        let mut ba = &inputs[i];
        let mut bb = &inputs[i+1];
        let mut p = (&inputs[i+2].0, (inputs[i+2].1.0+10000000000000, inputs[i+2].1.1+10000000000000));

        let mut cost = 0;

        //x=-0.234*y+89.362
        let diffb: f64 = -bb.1.0 as f64/ba.1.0 as f64;
        let diffp: f64 = p.1.0 as f64/ba.1.0 as f64;
        //println!("{:?} {:?}", diffb, diffp);

        //println!("{:?} {:?}", (ba.1.1 as f64 *diffb), (ba.1.1 as f64 * diffp));
        let x1 = (ba.1.1 as f64 *diffb) + bb.1.1 as f64;
        let x2 = (p.1.1 as f64 - (ba.1.1 as f64 * diffp)) as f64 /x1;
        println!("{:?} {:?} {:?}  {:?}", x2, x2.fract(),  x2 as i64, x2.round());

        if x2.fract() <  1.5e-5 || x2.fract() >= 0.9 {
            let x3 = (p.1.0-((x2.round()) as i64 * bb.1.0))/ba.1.0;
            //println!("{:?}", x3);
            cost = ((x2.round()) as i64);
            cost += x3 * 3
        }


        //println!("{:?}", cost);
        sum += cost;

    }
    println!("{:?}", sum);
}

fn main() {
    let mut inputs = read_from_file();

    println!("{:?}", inputs);
    //do_task_one(&inputs);
    do_task_two(&inputs);
}
