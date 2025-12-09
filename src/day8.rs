use core::f64;
use std::{fs, mem::take};

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
    let input: String = fs::read_to_string("./inputs/day8.txt").unwrap();
    let values: Vec<Coords> = input.lines().map(|line| Coords::from(line)).collect();
    //println!("{values:?}");
    let mut all_pairs: Vec<(Coords,Coords, f64)> = Vec::new();
    let mut pairs: Vec<(Coords, Coords, f64)>;
    let mut res = 1;
    for i in &values {
        'inner: for j in &values {
            if i == j {
                //println!("{i:?} == {j:?}");
                continue 'inner;
            }
            let distance =   ((j.x-i.x).powi(2)+(j.y-i.y).powi(2) + (j.z-i.z).powi(2)).sqrt();
            //if !all_pairs.contains(&(*j, *i, distance)) {
                all_pairs.push((*i,*j, distance));
            //}
        }
    }
    let mut circuits: Vec<Vec<Coords>> = Vec::new();
    all_pairs.sort_by(|f,g| f.2.total_cmp(&g.2));

    pairs = all_pairs.into_iter().enumerate().filter(|f| f.0 %2 == 0).take(1000).map(|f| f.1).collect();
    println!("All: {:?}", pairs);

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
        println!("Circuit Count: {circuit_count}");
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
        println!("Circuit: {:?}, Length: {}", circuit, circuit.len());
    }

    circuits.sort_by(|f,g| g.len().cmp(&f.len()));
    for circuit in &circuits {
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
pub fn p2() {
    let input: String = fs::read_to_string("./inputs/day8.txt").unwrap();
    let values: Vec<Coords> = input.lines().map(|line| Coords::from(line)).collect();
    //println!("{values:?}");
    let mut all_pairs: Vec<(Coords,Coords, f64)> = Vec::new();
    let mut res = 0;
    let mut success = false;
    let mut success_two = false;
    for i in &values {
        'inner: for j in &values {
            if i == j {
                //println!("{i:?} == {j:?}");
                continue 'inner;
            }
            let distance =   ((j.x-i.x).powi(2)+(j.y-i.y).powi(2) + (j.z-i.z).powi(2)).sqrt();
            //if !all_pairs.contains(&(*j, *i, distance)) {
                all_pairs.push((*i,*j, distance));
            //}
        }
    }
    let mut take_this = 1000;

    loop {
    println!("Running with {take_this} connections");
    let mut pairs: Vec<(Coords, Coords, f64)>;
    let mut circuits: Vec<Vec<Coords>> = Vec::new();
    all_pairs.sort_by(|f,g| f.2.total_cmp(&g.2));

    pairs = all_pairs.clone().into_iter().enumerate().filter(|f| f.0 %2 == 0).take(take_this).map(|f| f.1).collect();
    //println!("All: {:?}", pairs);

    //subset.iter().zip(pairs.clone()).for_each(|f| //println!("P1: {:?}, P2: {:?}", f.0, f.1));


    for pair in pairs.clone() {
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
    let mut circuit_count = circuits.len();

    loop {
        println!("Circuit Count: {circuit_count}");
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
    if circuit_count > 1 || !values.iter().all(|v| circuits[0].contains(v)){
        println!("Couldn't filter down to 1 circuit, running again");
        if success {
            take_this+=1;
            success_two = true;
        }
        else {
            take_this+=100;
        }
        continue;
    }

    else {

        if success_two {
            println!("Last Pair: {:?}", &pairs[pairs.len()-1]);
            println!("Results: {}", pairs[pairs.len()-1].0.x * pairs[pairs.len()-1].1.x);
            break;
        }
        println!("Could filter with {take_this} circuits, trying with fewer");
        success = true;
        take_this-=10;
    }

    //println!("{circuits:?}");

    let mut loop_counter = 0;
    for circuit in &mut circuits {
        circuit.sort_by(|f, g| f.x.total_cmp(&g.x));
        circuit.dedup();
        //println!("Circuit: {:?}, Length: {}", circuit, circuit.len());
    }

    circuits.sort_by(|f,g| g.len().cmp(&f.len()));
    for circuit in &circuits {
        res = circuit.len().max(1);
        loop_counter+=1;
        if loop_counter == 3 {
            break;
        }
    }


    //println!("{circuits:?}");
    println!("{}", circuits.len());
    println!("Result: {res}");
    }

}