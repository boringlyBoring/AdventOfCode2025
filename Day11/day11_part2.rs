use std::{collections::HashMap, fs};

fn parse(input: &String) -> HashMap<String, Vec<String>> {
    let mut graph = HashMap::new();
    input.lines().for_each(|line| {
        let row: Vec<&str> = line.split(":").collect();
        let neighbour: Vec<String> = row[1]
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect();
        graph.insert(row[0].to_string(), neighbour);
    });
    graph
}

fn dfs(
    u: &str,
    destination: &str,
    graph: &HashMap<String, Vec<String>>,
    cache: &mut HashMap<String, u64>,
) -> u64 {
    if u == destination {
        return 1;
    }
    if let Some(&value) = cache.get(u) {
        return value;
    }
    let mut count = 0;
    if let Some(neighbour) = graph.get(u) {
        for v in neighbour {
            count += dfs(v, destination, graph, cache);
        }
    }
    cache.insert(u.to_string(), count);
    count
}

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read string");
    let graph = parse(&input);
    let mut cache1 = HashMap::new();
    let mut cache2 = HashMap::new();
    let mut cache3 = HashMap::new();

    let ans = dfs("svr", "fft", &graph, &mut cache1)
        * dfs("fft", "dac", &graph, &mut cache2)
        * dfs("dac", "out", &graph, &mut cache3);
    println!("ans: {}", ans);
}
