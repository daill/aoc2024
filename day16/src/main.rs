use std::cmp::max;
use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};
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
    let mut file = File::open("inputs");
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

fn go(walls: &HashSet<(i32, i32)>, space: &mut HashSet<(i32, i32)>, np: &(i32, i32), end: &(i32, i32), path: &mut Vec<(i32, i32)>, paths: &mut Vec<Vec<(i32, i32)>>, queue: &mut VecDeque<(i32, i32)>) {







}

fn do_task_one(walls: &HashSet<(i32, i32)>, space: &mut HashSet<(i32, i32)>, start: &(i32, i32), end: &(i32, i32)) {
    let mut visited: Vec<(i32, i32)> = Vec::new();
    let mut paths: Vec<(i32, i32)> = Vec::new();
    let mut queue: VecDeque<(i32, i32)> = VecDeque::new();
    let mut orientation: VecDeque<(i32, i32)> = VecDeque::new();
    let mut costmap: HashMap<(i32, i32), i32> = HashMap::new();

    queue.push_back(*start);
    orientation.push_back((1,0));
    costmap.insert(*start, 1);

    let mut prev = start.clone();
    loop {
        if let Some(p) = queue.pop_front() {
            let mut prevorientation = orientation.pop_front().unwrap();
            for (x,y) in [(1,0) , (0,-1) , (-1,0) , (0,1) ] {
                let nnp = &(p.0+x, p.1+y);
                if space.contains(nnp) {
                    let mut d = 1;
                    if prevorientation != (x,y) {
                        d += 1000;
                    }
                    let prevcost = costmap.get(&p).unwrap();

                    //println!("{:?} {:?} {:?} {:?} {:?} {:?} {:?}", prev, p, nnp, (x,y), prevorientation, &prevcost, d);
                    if let Some(costs) = costmap.get(nnp) {
                        if costs > &(prevcost+d) {
                            costmap.insert(*nnp, (prevcost+d));
                            queue.push_back(*nnp);
                            orientation.push_back((x,y));

                        }
                    } else {
                        queue.push_back(*nnp);
                        orientation.push_back((x,y));
                        costmap.insert(*nnp, (prevcost+d));
                    }


                    space.remove(&nnp);


                }
            }
            prev = p;
        } else {
            break;
        }
    }

    println!("{:?}", costmap);
    let mut sum = 0;
    for (x,y) in [(1,0) , (0,-1) , (-1,0) , (0,1) ] {
        let bend = (end.0+x, end.1+y);
        if let Some(val) = costmap.get(&bend) {
            if sum == 0 || val < &sum {
                sum = *val;
            }
        }
    }
    println!("{:?}", sum);


}



fn do_task_two() {

}

fn main() {
    let mut inputs = read_from_file();

    println!("{:?} {:?} {:?} {:?}", &inputs.0, &inputs.1, &inputs.2,&inputs.3);
    do_task_one(&inputs.0, &mut inputs.1, &inputs.2, &inputs.3);
    //do_task_two(&inputs.0, &inputs.1, &inputs.2);
}

