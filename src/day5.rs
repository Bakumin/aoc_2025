use std::{collections::HashSet, fs, path::Path};

pub fn split_paragraphs(file_path: impl AsRef<Path>) -> Vec<String> {
    fs::read_to_string(file_path).unwrap().split("\r\n\r\n").map(|s| s.to_string()).collect::<Vec<String>>()

}


pub fn p1() {
    let paragraphs: Vec<String> = split_paragraphs("inputs/day5.txt");
    //println!("{input}");
    let ranges: Vec<(isize, isize)> = paragraphs[0].lines().map(|line| line.split_once("-").and_then(|f| Some((f.0.parse::<isize>().unwrap(), f.1.parse::<isize>().unwrap()))).unwrap()).collect();
    let values: Vec<isize> = paragraphs[1].lines().map(|line| line.parse::<isize>().unwrap()).collect();
    let mut res = 0;
    for v in values {
        if ranges.iter().find(|predicate| predicate.0 <= v && predicate.1 >= v).is_some() {
            res+=1;
        }
    }

    println!("{res}");


}

pub fn p2() {
    let paragraphs: Vec<String> = split_paragraphs("inputs/day5.txt");
    //println!("{input}");
    let mut ranges: Vec<(isize, isize)> = paragraphs[0].lines().map(|line| line.split_once("-").and_then(|f| Some((f.0.parse::<isize>().unwrap(), f.1.parse::<isize>().unwrap()))).unwrap()).collect();
    let mut flipped = false;
    loop {
        let mut new_ranges: Vec<(isize, isize)> = Vec::new();
        for r in ranges.clone() {
            match new_ranges.iter().find(|f| f.0 <= r.0 && f.1 >= r.1).to_owned() {
                Some(found) => {
                    println!("{}-{} doesn't change because {}-{} is fully contained", found.0, found.1, r.0, r.1);

                    continue;
                },
                None => {},
            }

            match new_ranges.iter().position(|f| f.0 <= r.0 && f.1 < r.1 && r.0 < f.1).to_owned() {
                Some(found) => {
                    println!("{}-{} changes to {}-{} using {}-{}", new_ranges[found].0, new_ranges[found].1, new_ranges[found].0, r.1, r.0, r.1);

                    new_ranges[found] = (new_ranges[found].0, r.1);
                    continue;
                },
                None => {},
            }
            
            match new_ranges.iter().position(|f| f.0 > r.0 && f.1 >= r.1 && r.1 >= f.0-1).clone() {
                Some(found) => {
                    println!("{}-{} changes to {}-{} using {}-{}", new_ranges[found].0, new_ranges[found].1, r.0, new_ranges[found].1, r.0, r.1);

                    new_ranges[found] = (r.0, new_ranges[found].1);
                    continue;
                },
                None => {},
            }
            
            match new_ranges.iter().position(|f| f.0 >= r.0 && f.1 <= r.1).clone() {
                Some(found) => {
                    println!("{}-{} changes to {}-{} ", new_ranges[found].0, new_ranges[found].1, r.0, r.1);

                    new_ranges[found] = (r.0, r.1);
                    continue;
                },
                None => {},
            }
            new_ranges.push(r);
        }
        println!("length: {}", new_ranges.len());
        if ranges.len() == new_ranges.len() {
            if flipped {
            break;
            }
            else {
                flipped = true;
                new_ranges.reverse();
            }
        }

        ranges = new_ranges;
    }
    let mut res = 0;
    for range in &ranges {
        res+= range.1+1-range.0;
    }
ranges.sort();
    println!("{:?}", ranges);
    println!("{res}");


}
