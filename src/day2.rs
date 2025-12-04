use std::fs;

pub fn p1() {
    let input = fs::read_to_string("./inputs/day2.txt").unwrap();
    let pairs: Vec<(isize, isize)> = input.split(",").map(|f| f.split_once("-").unwrap()).map(|f| (f.0.parse::<isize>().unwrap(), f.1.parse::<isize>().unwrap())).collect();

    for pair in pairs{
        for i in pair.0..=pair.1 {
            
        }
    }



}


pub fn p2() {
    let input = fs::read_to_string("./inputs/day2.txt").unwrap();
    let pairs: Vec<(isize, isize)> = input.split(",").map(|f| f.split_once("-").unwrap()).map(|f| (f.0.parse::<isize>().unwrap(), f.1.parse::<isize>().unwrap())).collect();


}