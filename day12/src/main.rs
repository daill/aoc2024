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

fn read_from_file() -> (HashMap<String, Vec<(i32, i32)>>, i32, i32) {
    let mut file = File::open("inputs");
    let result: (HashMap<String, Vec<(i32, i32)>>, i32, i32) = match file {
        Ok(file) => {
            let lines = io::BufReader::new(file).lines();
            let mut result: HashMap<String, Vec<(i32, i32)>> = HashMap::new();
            let mut height = 0;
            let mut width= 0;
            for line in lines {
                if let Ok(line) = line {
                    width = line.len() as i32;
                    line.chars().enumerate().for_each(|(i, x)| result.entry(x.to_string()).or_insert_with(|| Vec::new()).push((i as i32, height)));
                }
                height += 1;
            }
            (result, width, height)
        }
        Err(e) => panic!("Cannot process file: {}", e),
    };
    result
}

fn calc(coord: &Vec<(i32, i32)>, start:&(i32, i32), seen: &mut Vec<(i32, i32)>, width: i32, height: i32, area: &mut HashSet<(i32, i32)>) -> (i32, i32) {
    let mut fences = 4;
    let mut cnt = 1;
    area.insert(start.clone());
    seen.push(start.clone());
    for (x,y) in [(0,-1),(1, 0),(0,1), (-1,0)] {
        let p = &(&start.0+x, &start.1+y);
        if (p.0 >= 0 && p.0 < width && p.1 >= 0 && p.1 < height) {
            if coord.contains(p) {
                fences-=1;
                if !seen.contains(p) {
                    //println!("see {:?} {:?}", p, fences);
                    let r = calc(&coord, p, seen, width, height, area);
                    fences += r.0;
                    cnt += 1;
                    //println!("after see {:?} {:?}", p, fences);
                }
            }
        }
        //println!("{:?} {:?}", p, fences);
    }

    (fences, cnt)
}


fn do_task_one(inputs: &HashMap<String, Vec<(i32, i32)>>, width: i32, height: i32) {
    let mut fencecount: Vec<(String, i32, i32)> = Vec::new();
    let mut area: HashSet<(i32, i32)> = HashSet::new();
    for (c, coord) in inputs {
        let mut seen: Vec<(i32, i32)> = Vec::new();
        println!("{:?} {:?}", c, coord);
        for i in 0..coord.len() {
            let p = &coord[i];
            let mut count = 0;
            if !seen.contains(p) {
                let r = calc(&coord, p, &mut seen, width, height, &mut area);
                count += r.0;
            }
            if count != 0 {
                println!("area {:?}", area);
                fencecount.push((c.clone(), area.len() as i32, count));
                count = 0;

            }
        }
        area.clear();
        seen.clear();
    }
    println!("{:?}", fencecount);
    let mut sum = 0;
    for fences in fencecount {
        sum += fences.1*fences.2;
    }
    println!("{:?}", sum);
}

fn do_task_two(inputs: &HashMap<String, Vec<(i32, i32)>>, width: i32, height: i32) {
    let mut fencecount: Vec<(String, i32, i32, HashSet<(i32, i32)>)> = Vec::new();
    let mut area: HashSet<(i32, i32)> = HashSet::new();
    let mut sum = 0;
    for (c, coord) in inputs {
        let mut seen: Vec<(i32, i32)> = Vec::new();
        //println!("{:?} {:?}", c, coord);
        for i in 0..coord.len() {
            let p = &coord[i];
            let mut count = 0;
            if !seen.contains(p) {
                let r = calc(&coord, p, &mut seen, width, height, &mut area);
                count += r.0;
            }
            if count != 0 {

                //println!("area {:?}", area);
                fencecount.push((c.clone(), area.len() as i32, count, area.clone()));
                count = 0;
                area.clear();
            }
        }

        seen.clear();
    }
    //println!("{:?}", fencecount);

    for i in 0..fencecount.len() {
        let (name, areasize, fence, area) = &fencecount[i];
        let mut corners = 0;
        for p in area {

            //000 0X0 000 000
            //0X0 0X0 XX0 0XX
            //0X0 000 000 000

            //0X? ?X0 000 000
            //0XX XX0 XX0 0XX
            //000 000 ?X0 0X?

            //000 00X X00 XXX
            //0X0 0XX XX0 0X0
            //XXX 00X X00 000

            let point = p;
            let top = &(point.0, point.1-1);
            let right = &(point.0+1, point.1);
            let left = &(point.0-1, point.1);
            let bottom = &(point.0, point.1+1);
            let topr = &(point.0+1, point.1-1);
            let topl = &(point.0-1, point.1-1);
            let bottomr = &(point.0+1, point.1+1);
            let bottoml = &(point.0-1, point.1+1);

            //println!("check {:?} {:?} topl {:?} top {:?} topr {:?} right {:?} left {:?} bottoml {:?} bottom {:?} bottomr {:?} {:?}", name, p, topl, top, topr, right, left, bottoml, bottom, bottomr, (area.contains(bottom) && area.contains(right)));
            if area.contains(top) && area.contains(topr) && area.contains(topl) && area.contains(left) && area.contains(right) && area.contains(bottom) && area.contains(bottoml)  && area.contains(bottomr) {
                // ignore
                continue;
            }

            if (!area.contains(top) && !area.contains(topr) && !area.contains(topl) && !area.contains(left) && !area.contains(right) && !area.contains(bottom) && !area.contains(bottoml)  && !area.contains(bottomr)) ||
                        (area.contains(top) && !area.contains(topr) && !area.contains(topl) && area.contains(left) && area.contains(right) && area.contains(bottom) && !area.contains(bottoml) && !area.contains(bottomr)){
                // single
                //println!("added 4 no neighbor or plus");
                corners += 4;
                continue;
            }
            if (!area.contains(top) && !area.contains(topr) && area.contains(left) && !area.contains(right) && !area.contains(bottom)  && !area.contains(bottomr)) ||
                    (!area.contains(top) && !area.contains(topl) && !area.contains(left) && area.contains(right) && !area.contains(bottom) && !area.contains(bottoml)) ||
                    (area.contains(top) && !area.contains(left) && !area.contains(right) && !area.contains(bottom) && !area.contains(bottoml)  && !area.contains(bottomr)) ||
                    (!area.contains(top) && !area.contains(topr) && !area.contains(topl) && !area.contains(left) && !area.contains(right) && area.contains(bottom)) {
                // ends - |
                //println!("added 2 end");
                corners += 2;
                continue;
            }


                if area.contains(top) && area.contains(left) && !area.contains(topl) {
                   // println!("added 1 corner inner");
                    corners += 1;
                }
                if (area.contains(top) && area.contains(left) && !area.contains(right) && !area.contains(bottom)) {
                    //println!("added 1 corner");
                    corners += 1;
                }
                if area.contains(bottom) && area.contains(left) && !area.contains(bottoml) {
                    //println!("added 1 corner inner");
                    corners += 1;
                }
                if area.contains(bottom) && area.contains(left) && (!area.contains(right) && !area.contains(top)) {
                    //println!("added 1 corner");

                    corners += 1;
                }
                if (area.contains(bottom) && area.contains(right) && !area.contains(bottomr)) {
                    //println!("added 1 corner inner");
                    corners += 1;
                }
                if (area.contains(bottom) && area.contains(right) && !area.contains(left) && !area.contains(top)) {
                    //println!("added 1 corner");
                    corners += 1;
                }
                if area.contains(top) && area.contains(right) && !area.contains(topr) {
                    //println!("added 1 corner inner");
                    corners += 1;
                }
                if (area.contains(top) && area.contains(right) && !area.contains(left) && !area.contains(bottom)) {
                    //println!("added 1 corner");
                    corners += 1;
                }

        }

        sum += corners*area.len();
        println!("{:?} {:?} {:?} {:?}", name, corners, area, sum);
    }


}

fn main() {
    let mut inputs = read_from_file();

    println!("{:?}", inputs);
    //do_task_one(&inputs.0, inputs.1, inputs.2); \
    do_task_two(&inputs.0, inputs.1, inputs.2);
}
