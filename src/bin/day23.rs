use std::collections::HashSet;
use itertools::Itertools;

fn main() {
    let sample = include_str!("../../inputs/sample23.txt");
    println!("Sample");
    solve(&sample);

    let input = include_str!("../../inputs/day23.txt");
    println!("\nPuzzle");
    solve(&input);
}

fn solve(input: &str) {
    let connections = parse_connections(input);
    let mut nodes: HashSet<&String> = HashSet::new();
    let mut edges: HashSet<(&String, &String)> = HashSet::new();
    for (first, second) in &connections {
        nodes.insert(first);
        nodes.insert(second);
        edges.insert((first, second));
        edges.insert((second, first));
    }
    part1(&nodes, &edges);
    part2(&nodes, &edges);
}

fn part1(nodes: &HashSet<&String>, edges: &HashSet<(&String, &String)>) {
    let mut triples: HashSet<Vec<&String>> = HashSet::new();
    for first in nodes {
        if first.starts_with("t") {
            for second in nodes {
                if edges.contains(&(*first, *second)) {
                    for third in nodes {
                        if edges.contains(&(*first, *third)) && edges.contains(&(*second, *third)) {
                            let mut triple = vec![*first, *second, *third];
                            triple.sort();
                            triples.insert(triple);
                        }
                    }
                }
            }
        }
    }
    println!("Clusters with 3 nodes, at least one starting with 't': {}", triples.len());
}

fn part2(nodes: &HashSet<&String>, edges: &HashSet<(&String, &String)>) {
    let mut clusters: HashSet<Vec<&String>> = HashSet::new();
    for node in nodes {
        clusters.insert(vec![node]);
    }
    for node in nodes {
        let mut new_clusters = HashSet::new();
        for cluster in clusters.iter() {
            if is_connected_to_all(node, cluster, edges) {
                let mut new_cluster = cluster.clone();
                new_cluster.push(*node);
                new_cluster.sort();
                new_clusters.insert(new_cluster);
            }
        }
        clusters.extend(new_clusters);
    }
    let max_cluster = clusters.iter().max_by(|a, b| a.len().cmp(&b.len())).unwrap();
    println!("Password: {}", max_cluster.iter().format(","));
}

fn is_connected_to_all(node: &String, cluster: &Vec<&String>, edges: &HashSet<(&String, &String)>) -> bool {
    for n in cluster.iter() {
        if !edges.contains(&(n, node)) {
            return false;
        }
    }
    true
}

fn parse_connections(input: &str) -> Vec<(String, String)> {
    input.lines()
        .map(|line| (String::from(&line[..2]), String::from(&line[3..])))
        .collect()
}
