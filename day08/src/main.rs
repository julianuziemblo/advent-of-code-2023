use std::{collections::HashMap, fs};

#[allow(unused)]
const TEST1_FILE: &str = "test1.txt";
#[allow(unused)]
const TEST2_FILE: &str = "test2.txt";
#[allow(unused)]
const TEST3_FILE: &str = "test3.txt";
#[allow(unused)]
const INPUT_FILE: &str = "input.txt";

const START_NODE: &str = "AAA";
const END_NODE: &str = "ZZZ";

type Map<'a> = HashMap<&'a str, Directions<'a>>;

#[derive(Clone, Debug)]
enum Order {
    Left,
    Right,
}

#[derive(Clone, Debug)]
struct Directions<'a> {
    left: &'a str,
    right: &'a str,
}

fn parse_input(input: &str) -> (Vec<Order>, Map<'_>) {
    let tokens = input.trim().split("\r\n\r").collect::<Vec<&str>>();

    let order = tokens[0]
        .trim()
        .chars()
        .map(|ch| match ch {
            'L' => Order::Left,
            'R' => Order::Right,
            c => panic!("A character {c} appeared that should not be there!"),
        })
        .collect();

    let lines = tokens[1].trim().lines();

    let mut map = Map::with_capacity(lines.clone().count());

    for line in lines {
        let line = line.trim().split(" = ").collect::<Vec<&str>>();

        let node_name = line[0].trim();
        let node_lr_all = line[1].trim().split(", ").collect::<Vec<&str>>();
        let left = &node_lr_all[0][1..];
        let right = &node_lr_all[1][..(node_lr_all[1].len() - 1)];

        map.insert(node_name, Directions { left, right });
    }

    (order, map)
}

fn traverse(order: &Vec<Order>, map: &Map) -> u32 {
    let mut count = 0;

    let mut current_node = START_NODE;

    while current_node != END_NODE {
        let current_directions = &map[current_node];
        let current_order = &order[count % order.len()];
        let next_node = match current_order {
            Order::Left => current_directions.left,
            Order::Right => current_directions.right,
        };

        // println!("Current node: {}, next node: {}, current order: {:?}", current_node, next_node, current_order);

        current_node = next_node;

        count += 1;
    }
    // println!("Current node: {}", current_node);

    count as u32
}

fn find_all_starting<'a>(map: &'a Map) -> Vec<&'a str> {
    let mut starting = Vec::new();

    for (&node, _) in map.iter() {
        if node.ends_with('A') {
            starting.push(node);
        }
    }

    starting
}

fn find_all_ending<'a>(map: &'a Map) -> Vec<&'a str> {
    let mut starting = Vec::new();

    for (&node, _) in map.iter() {
        if node.ends_with('Z') {
            starting.push(node);
        }
    }

    starting
}

fn are_ending(nodes: &Vec<&str>) -> bool {
    for &node in nodes {
        if !node.ends_with('Z') {
            return false;
        }
    }

    true
}

#[allow(unused)]
fn ending_at_least(nodes: &Vec<&str>, num: usize) -> bool {
    let mut count = 0;
    for &node in nodes {
        if node.ends_with('Z') {
            count += 1;
        }
    }

    count >= num
}

#[allow(unused)]
/// Too many posibilities - can't work in reasonable time
///
/// Let it work for 2-3h, didn't yeld a result
/// (even in --release mode...)
fn traverse_parallel(order: &Vec<Order>, map: &Map) -> u32 {
    let mut current_nodes = find_all_starting(map);

    println!("[PART 2] Starting points: {:?}", current_nodes);
    println!("[PART 2] Ending points: {:?}", find_all_ending(map));

    let mut count = 0;

    while !are_ending(&current_nodes) {
        let nodes_iter = current_nodes.clone();
        for (i, &node) in nodes_iter.iter().enumerate() {
            let current_directions = &map[node];

            let current_order = &order[count % order.len()];
            let next_node = match current_order {
                Order::Left => current_directions.left,
                Order::Right => current_directions.right,
            };

            current_nodes[i] = next_node;
        }
        // if ending_at_least(&current_nodes, 5) {
        //     println!("[BREAKING] 5 NODES MATCH!");
        // }
        // if ending_at_least(&current_nodes, 4) {
        //     println!("Count at {:#?}", count.to_string()
        //         .as_bytes()
        //         .rchunks(3)
        //         .rev()
        //         .map(std::str::from_utf8)
        //         .collect::<Result<Vec<&str>, _>>()
        //         .unwrap()
        //         .join("_"));
        //     println!("Current nodes: {:?}", current_nodes);
        // }
        count += 1;
    }

    count as u32
}

fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;

    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }

    a
}

fn lcm(nums: &Vec<usize>) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }

    let a = nums[0];
    let b = lcm(&nums[1..].to_vec());

    a * b / gcd(a, b)
}

/// LCM
fn traverse_lcm(order: &Vec<Order>, map: &Map) -> usize {
    let mut nodes = find_all_starting(map);
    let mut counts = Vec::with_capacity(nodes.len());

    for node in nodes.iter_mut() {
        let mut count = 0;
        while !node.ends_with('Z') {
            let current_directions = &map[node];
            let current_order = &order[count % order.len()];
            let next_node = match current_order {
                Order::Left => current_directions.left,
                Order::Right => current_directions.right,
            };

            *node = next_node;
            count += 1;
        }
        counts.push(count);
    }

    lcm(&counts)
}

fn main() {
    let filename = INPUT_FILE;
    let input = fs::read_to_string(filename)
        .unwrap_or_else(|_| panic!("Couldn't read from file `{filename}`"));

    let (order, map) = parse_input(&input);
    // println!("Order: {order:#?}, map: {map:#?}");

    // PART 1
    let count = traverse(&order, &map);
    println!("[PART 1] Traversed graph, count = {}", count);

    // PART 2 - bruteforce (did not work)
    // let count = traverse_parallel(&order, &map);
    // println!("[PART 2] Traversed graph, count = {}", count);

    // PART 2 - LCM (it works!)
    let count = traverse_lcm(&order, &map);
    println!("[PART 2] 'Traversed' graph, count = {}", count);
}
