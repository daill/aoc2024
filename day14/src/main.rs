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

fn read_from_file() -> Vec<((i64, i64), (i64, i64))> {
    let mut file = File::open("test");
    let result: Vec<((i64, i64), (i64, i64))> = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            let mut result: Vec<((i64, i64), (i64, i64))> = Vec::new();
            let butre = Regex::new(r"(?m)^p=(\S*\d+),(\S*\d+)\Wv=(\S*\d+),(\S*\d+)$").unwrap();

            for line in lines {
                if let Ok(line) = line {
                    println!("{:?}", line);
                    butre.captures_iter((&line).as_str()).for_each(|c| {
                        let but = ((c[1].parse::<i64>().unwrap(),c[2].parse::<i64>().unwrap()),(c[3].parse::<i64>().unwrap(),c[4].parse::<i64>().unwrap()));
                        result.push(but);
                        println!("{:?}",c )
                    })
                }

            }
            result
        }
        Err(e) => panic!("Cannot process file: {}", e),
    };
    result
}


fn do_task_one(inputs: &Vec<((i64, i64), (i64, i64))>) {
    let width = 101;
    let height = 103;
    let seconds = 100;

    let mut robots = inputs.clone();

    for i in 0..robots.len() {
        let mut newx = (robots[i].0.0 + (robots[i].1.0 * seconds))%width;
        let mut newy = (robots[i].0.1 + (robots[i].1.1 * seconds))%height;
        if newx < 0 {
            newx = width+newx;
        }

        if newy < 0 {
            newy = height+newy;
        }

        robots[i].0.0 = newx;
        robots[i].0.1 = newy;
    }

    // q1 corner (5,3)
    let x = &(width/2);
    let y = &(height/2);

    let mut q: (i64,i64,i64,i64) = (0,0,0,0);

    robots.iter().for_each(|x| println!("{:?}", x));
    robots.iter().for_each(|((px, py),(vx, vy))| {
        if px < x && py < y {
            q.0 += 1;
        } else if px < x && py > y {
            q.3 += 1;
        } else if px > x && py < y {
            q.1 += 1;
        }else if px > x && py > y {
            q.2 += 1;
        }
    });

    println!("{:?} {:?} {:?} {:?} {:?}", q.0, q.1, q.2, q.3, q.0*q.1*q.2*q.3);

}

fn do_task_two(inputs: &Vec<((i64, i64), (i64, i64))>) {
    let width = 101;
    let height = 103;
    let seconds = 90000;
    let steps: i64 = 1;
    let points: HashSet<(i32, i32)> = HashSet::new();

    let mut robots = inputs.clone();

    for u in (0..seconds).step_by(steps as usize) {
        let mut points: HashSet<(i64,i64)> = HashSet::new();

        for i in 0..robots.len() {
            let mut newx = (robots[i].0.0 + (robots[i].1.0 * steps))%width;
            let mut newy = (robots[i].0.1 + (robots[i].1.1 * steps))%height;
            if newx < 0 {
                newx = width+newx;
            }

            if newy < 0 {
                newy = height+newy;
            }

            robots[i].0.0 = newx;
            robots[i].0.1 = newy;

            points.insert((newx, newy));
        }

        // q1 corner (5,3)
        let x = &(width/2);
        let y = &(height/2);

        let mut q: (i64,i64,i64,i64) = (0,0,0,0);

        points.iter().for_each(|(px, py)| {
            if px < x && py < y {
                q.0 += 1;
            } else if px < x && py > y {
                q.3 += 1;
            } else if px > x && py < y {
                q.1 += 1;
            }else if px > x && py > y {
                q.2 += 1;
            }
        });

        // wild guess
        if points.len() == inputs.len() {
            println!("{:?}", u+1);
            break;
        }
    }
}

fn print(robots: &Vec<((i64, i64), (i64, i64))>, width: usize, height: usize) {
    let mut pic = vec![vec![""; width]; height];
    for ((px,py),(_,_)) in robots {
        pic[*py as usize][*px as usize] = ".";
    }
    for row in pic {
        //println!("{:?}", row);
    }
}

fn main() {
    let mut inputs = read_from_file();

    println!("{:?}", inputs);
    //do_task_one(&inputs);
    do_task_two(&inputs);
}

