use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn parse(input: &String) -> HashMap<&str, Vec<&str>> {
    let mut graph = HashMap::new();
    input.lines().for_each(|line| {
        let row: Vec<&str> = line.split(":").collect();
        let neighbours: Vec<&str> = row[1].split(" ").filter(|s| !s.is_empty()).collect();
        graph.insert(row[0], neighbours);
    });
    graph
}

fn dfs<'a>(
    u: &str,
    destination: &str,
    graph: &HashMap<&str, Vec<&'a str>>,
    visited: &mut HashSet<&'a str>,
) -> u64 {
    if destination == u {
        return 1;
    }
    let mut count = 0;
    if let Some(neighbours) = graph.get(&u) {
        for v in neighbours {
            if visited.contains(v) {
                continue;
            }
            visited.insert(v);
            count += dfs(v, destination, graph, visited);
            visited.remove(v);
        }
    }
    count
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to get input from file");
    let graph = parse(&input);
    let mut visited: HashSet<&str> = HashSet::new();
    visited.insert("you");
    println!("Answer: {}", dfs("you", "out", &graph, &mut visited));
}
