use core::f64;
use std::{fs};

#[derive(Clone, Copy, Debug, PartialEq)]
struct Coords {
    x: f64,
    y: f64,
    z: f64
}
impl Coords {
    pub fn from(s: &str) -> Self {
        let interm: Vec<&str> = s.split(",").collect();
        //println!("{s}");
        Self {
            x: interm[0].parse::<f64>().unwrap(),
            y: interm[1].parse::<f64>().unwrap(),
            z: interm[2].parse::<f64>().unwrap(),
        }
    
    }
}
pub fn finish_circuit(c: Coords, pairs: &mut Vec<(Coords, Coords, f64)>, mut circuit: Vec<Coords>) -> Vec<Coords> {



    todo!();
}





pub fn p1() {
    let input = fs::read_to_string("./inputs/day8.txt").unwrap();
    let values: Vec<Coords> = input.lines().map(|line| Coords::from(line)).collect();
    //println!("{values:?}");
    let mut all_pairs: Vec<(Coords,Coords, f64)> = Vec::new();
    let mut pairs: Vec<(Coords, Coords, f64)> = Vec::with_capacity(10);
    let mut res = 1;
    for i in &values {
        'inner: for j in &values {
            if i == j {
                //println!("{i:?} == {j:?}");
                continue 'inner;
            }
            let distance =   ((j.x-i.x).powi(2)+(j.y-i.y).powi(2) + (j.z-i.z).powi(2)).sqrt();
            if !all_pairs.contains(&(*j, *i, distance)) {
                all_pairs.push((*i,*j, distance));
            }
            //println!("i: {i:?}, j: {j:?}, distance: {distance}");
            if pairs.len() < 1000 {
                if !pairs.contains(&(*j, *i, distance)) {
                pairs.push((*i, *j, distance));
                if pairs.len() == 1000 {
                    pairs.sort_by(|f, g| f.2.total_cmp(&g.2));
                    //println!{"1: {:?}, 1000: {:?}", pairs[0], pairs[999]};
                }
            }
            }
            else {
                if pairs[999].2 > distance && !pairs.contains(&(*j, *i, distance)) {
                    pairs[999] = (*i, *j, distance);
                    pairs.sort_by(|f, g| f.2.total_cmp(&g.2));
                                        //println!{"1: {:?}, 1000: {:?}", pairs[0], pairs[999]};

                }
            }
        }
    }
    let mut circuits: Vec<Vec<Coords>> = Vec::new();

    all_pairs.sort_by(|f,g| f.2.total_cmp(&g.2));
    let subset: Vec<&(Coords, Coords, f64)> = all_pairs.iter().take(1000).collect();
    //println!("All: {:?}", subset);

    //subset.iter().zip(pairs.clone()).for_each(|f| //println!("P1: {:?}, P2: {:?}", f.0, f.1));


    for pair in pairs {
        //println!("Pair: {pair:?}");

        let pos = circuits.iter().position(|c| c.contains(&pair.0));    
        match pos {
            Some(p) => {
                circuits[p].push(pair.1);
            },
            None => {
                let new_c = vec![pair.0, pair.1];
                circuits.push(new_c);
            }
        }
    }

    loop {
        let mut circuit_count = circuits.len();
        //println!("Circuit Count: {circuit_count}");
        for i in 0..circuits.len() {
            for j in 0..circuits.len() {
                if i != j 
                {
                if circuits[j].iter().any(|co| circuits[i].contains(co)) {
                    let mut other = circuits[j].clone();
                    circuits[i].append(&mut other);
                    circuits[j] = Vec::new();

                }
            }
            }
        }
        circuits = circuits.into_iter().filter(|c| c.len() > 0).collect();
        if circuit_count == circuits.len() {
            break;
        }
        circuit_count = circuits.len();

    }



    //println!("{circuits:?}");

    let mut loop_counter = 0;
    for circuit in &mut circuits {
        circuit.sort_by(|f, g| f.x.total_cmp(&g.x));
        circuit.dedup();
        res *= circuit.len().max(1);
        loop_counter+=1;
        if loop_counter == 3 {
            break;
        }
    }


    //println!("{circuits:?}");
    println!("{}", circuits.len());
    println!("Result: {res}");


    //let sorted = values.sort_by(|a, b| (a.x+a.y+a.z));


}
pub fn p2() {}