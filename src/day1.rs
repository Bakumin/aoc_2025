use std::{error, fs};
use regex;

pub fn p1() {
    let mut re = regex::Regex::new(r#"(?P<direction>\w)(?P<numbah>\d{1,3})"#).unwrap();
    let input = fs::read_to_string("./inputs/day1.txt").unwrap();
    let lines = input.lines().map(|line| 
        re.captures(line).and_then(|c| Some((c.name("direction").unwrap().as_str(), i64::from_str_radix(c.name("numbah").unwrap().as_str(), 10).unwrap()))).unwrap().clone()
    );
    
    let mut counter = 50;
    let mut res = 0;
    for line in lines {
        match line.0 {
            "R" => {
                counter += line.1
            },
            "L" => {
                counter -= line.1
            },
            _ => {
                println!("Issue parsing")
            }
        }
        counter = counter.rem_euclid(100);
        if counter == 0 {
            res+=1;
        }
    }

    println!("{res}")
}

pub fn p2() {
    let mut re = regex::Regex::new(r#"(?P<direction>\w)(?P<numbah>\d{1,3})"#).unwrap();
    let input = fs::read_to_string("./inputs/day1.txt").unwrap();
    let lines = input.lines().map(|line| 
        re.captures(line).and_then(|c| Some((c.name("direction").unwrap().as_str(), i64::from_str_radix(c.name("numbah").unwrap().as_str(), 10).unwrap()))).unwrap().clone()
    );
    
    let mut counter = 50;
    let mut res = 0;
    for line in lines {
        match line.0 {
            "R" => {
                counter += line.1
            },
            "L" => {
                counter -= line.1
            },
            _ => {
                println!("Issue parsing")
            }
        }
        if counter > 99 {
            println!("Counter is {counter} so we're adding {} to {res}", counter.abs()/100);
            res+= counter.abs()/100;
            
        }
        if counter < 0 {
            println!("Negative: Counter is {counter} so we're adding {} to {res}", (counter.abs()/100) +1);
            res+= (counter.abs()/100) +1;
            if line.1 == counter * -1 {
                res -= 1;
            }
        
        }
        if counter == 0 {
            res+=1;
        }
        counter = counter.rem_euclid(100);
    }

    println!("{res}")
}