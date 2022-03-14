mod types;
use rand::prelude::*;
use std::env;
use types::Node;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut rng = rand::thread_rng();
    let numbers = match args.len() {
        2 => {
            let numbers = args[1].clone();
            numbers
                .split(",")
                .map(|part| part.parse::<i32>().unwrap_or(0))
                .collect::<Vec<i32>>()
        }
        _ => {
            let mut numbers: Vec<i32> = (0..9).collect();
            numbers.shuffle(&mut rng);
            numbers
        }
    };
    let start = types::puzzle::State::from(&[
        [numbers[0], numbers[1], numbers[2]],
        [numbers[3], numbers[4], numbers[5]],
        [numbers[6], numbers[7], numbers[8]],
    ]);
    println!("Inicio: \n{}", start.to_string());
    let end: types::puzzle::State = types::puzzle::State::from(&[[0, 1, 2], [3, 4, 5], [6, 7, 8]]);
    let tree = types::Tree {
        root: start.clone(),
    };
    println!("Buscando ...");
    match tree.breadth_first_search(&end) {
        Ok(res) => {
            println!("Encontrado: \n{}", res.to_string());
            println!("Pasos: ");
            match res.steps {
                None => (),
                Some(steps) => {
                    let mut start = start;
                    for step in &steps {
                        let node = start.move_space(&step);
                        println!("{}", node.to_string());
                        start = node;
                    }
                }
            }
        }
        Err(_) => {
            println!("Sin solución")
        }
    };
}
