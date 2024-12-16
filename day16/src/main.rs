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

fn read_from_file() -> (HashSet<(i32, i32)>, HashSet<(i32, i32)>, (i32, i32), (i32, i32)) {
    let mut file = File::open("test");
    let result: (HashSet<(i32, i32)>, HashSet<(i32, i32)>, (i32, i32), (i32, i32)) = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();

            let mut walls:HashSet<(i32, i32)> = HashSet::new();
            let mut space:HashSet<(i32, i32)> = HashSet::new();
            let mut start:(i32, i32) = (0,0);
            let mut end:(i32, i32) = (0,0);
            let mut height = 0;
            let mut width = 0;

            for line in lines {
                if let Ok(line) = line {
                    println!("{:?}", line);

                    width = line.len() as i32;
                    line.chars().into_iter().enumerate().for_each(|(i, x)| {
                        if x == '#' {
                            walls.insert((i as i32, height));
                        }
                        if x == 'E' {
                            end = (i as i32, height);
                        }
                        if x == 'S' {
                            start = (i as i32, height);
                        }
                        if x == '.' {
                            space.insert((i as i32, height));
                        }

                    });
                    height += 1;

                }

            }
            (walls, space, start, end)
        }
        Err(e) => panic!("Cannot process file: {}", e),
    };
    result
}

fn go(walls: &HashSet<(i32, i32)>, space: &HashSet<(i32, i32)>, np: &(i32, i32), end: &(i32, i32), path: &mut Vec<(i32, i32)>, paths: &mut Vec<Vec<(i32, i32)>>) {
    if end == np {
        println!("found {:?}", path.len());
        paths.push(path.clone());
        return;
    }


    if !walls.contains(np) {

        //println!("next spot {:?}", np);

        if !path.is_empty() && path.contains(np) {
            // skip
        } else {
            path.push(*np);
            for (x,y) in [(1,0) , (0,-1) , (-1,0) , (0,1) ] {
                go(walls, space, &(np.0+x, np.1+y), end, path, paths);
            }
            path.pop();
        }
    }

}

fn do_task_one(walls: &HashSet<(i32, i32)>, space: &HashSet<(i32, i32)>, start: &(i32, i32), end: &(i32, i32)) {
    let mut path: Vec<(i32, i32)> = Vec::new();
    let mut paths: Vec<Vec<(i32, i32)>> = Vec::new();

    go(walls, space, start, end, &mut path, &mut paths);

    let mut score = 0;
    let mut path: &Vec<(i32, i32)> = &Vec::new();
    paths.iter().for_each(|p| {
        let mut psum = 0;
        let mut prevorientation = (0,0);
        for i in 1..p.len() {
            if (p[i].0 - p[i - 1].0, p[i].1 - p[i - 1].1) != prevorientation {
                prevorientation = (p[i].0 - p[i - 1].0, p[i].1 - p[i - 1].1);
                psum += 1000;
            }

        }
        psum += p.len();
        if score == 0 {
            score = psum;
            path = &p;
        } else {
            if psum < score {
                score = psum;
                path = &p;
            }
        }
    });

    println!("{:?} {:?}", score, path);

}



fn do_task_two() {

}

fn main() {
    let mut inputs = read_from_file();

    println!("{:?} {:?} {:?} {:?}", &inputs.0, &inputs.1, &inputs.2,&inputs.3);
    do_task_one(&inputs.0, &inputs.1, &inputs.2, &inputs.3);
    //do_task_two(&inputs.0, &inputs.1, &inputs.2);
}

