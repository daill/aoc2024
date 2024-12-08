use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufRead;
use std::io::{self, Write};
use std::iter::Iterator;
use std::ptr::copy;

fn read_from_file() -> (Vec<char>, Vec<(i32, i32)>, i32, i32) {
    let mut file = File::open("test");
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
    let mut found: Vec<(i32, i32)> = Vec::new();
    for i in 0..inputs.len() {
        for j in 0..inputs.len() {
            if chars[i] == chars[j] && i != j {
                //println!("init {:?} {:?}", inputs[i], inputs[j]);
                let p1 = inputs[i];
                let p2 = inputs[j];

                if (!seen.contains(&(p1, p2)) && !seen.contains(&(p2, p1))){
                    //println!("{:?} {:?} {:?}", p1, p2, seen);
                    seen.push((p1, p2));

                    let dx = p2.0 - p1.0;
                    let dy = p2.1 - p1.1;

                    let m = (dy as f32 / dx as f32);
                    // b = y1 - mx1
                    let b = (p1.1 as f32 - m * p1.0 as f32);

                    let mut points: Vec<((i32, i32), i32, i32)> = Vec::new();


                    for x in 0..width {
                        let y = (m * (x as f32) + b);

                        if y >= 0.0 && y.fract() == 0.00000 && y < height as f32 {

                            let point = (x, y as i32);
                            if !inputs.contains(&point) {
                                //if point != p1 && point != p2 {
                                let l1 = (p1.0 - point.0).abs() + (p1.1 - point.1).abs();
                                let l2 = (p2.0 - point.0).abs() + (p2.1 - point.1).abs();
                                points.push((point, l1, l2));
                                //println!(" --- x {:?} y {:?} {:?} {:?}", x, (m * (x as f32) + b), m, b);
                                //}
                            }

                        }
                        //println!("{:?}", points);

                    }

                    for p in 0..points.len() {
                        if points[p].1 * 2 == points[p].2 || points[p].2 * 2 == points[p].1 {
                            println!("{:?} {:?} {:?} {:?}", chars[i], inputs[i], inputs[j], points[p]);
                            sum += 1;
                            found.push(points[p].0);
                        }
                    }
                }
            }
        }
    }
    println!("{:?} {:?} {:?}", sum, found.len(), found);
}

fn do_task_two() {

}

fn main() {
    let inputs = read_from_file();

    println!("{:?}", inputs);
    do_task_one(inputs.0, &inputs.1, inputs.2, inputs.3);
    //do_task_two(&inputs);
}
