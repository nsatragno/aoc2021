use std::{collections::HashMap, fs};

fn traverse<'a>(
    node: &'a str,
    mut visited: Vec<&'a str>,
    graph: &'a HashMap<String, Vec<String>>,
    paths: &mut Vec<Vec<&'a str>>,
) {
    visited.push(node);
    if node == "end" {
        paths.push(visited.clone());
        return;
    }
    for neighbour in graph.get(node).unwrap() {
        if neighbour.chars().next().unwrap().is_lowercase() && visited.contains(&neighbour.as_str())
        {
            // Small cave we already visited.
            continue;
        }
        traverse(neighbour.as_str(), visited.clone(), graph, paths);
    }
}

fn traverse2<'a>(
    node: &'a str,
    mut visited: Vec<&'a str>,
    graph: &'a HashMap<String, Vec<String>>,
    paths: &mut Vec<Vec<&'a str>>,
    repeated: bool,
) {
    visited.push(node);
    if node == "end" {
        paths.push(visited.clone());
        return;
    }
    for neighbour in graph.get(node).unwrap() {
        if neighbour == "start" {
            continue;
        }
        if neighbour.chars().next().unwrap().is_lowercase() && visited.contains(&neighbour.as_str())
        {
            if repeated {
                continue;
            }
            traverse2(neighbour.as_str(), visited.clone(), graph, paths, true);
        } else {
            traverse2(neighbour.as_str(), visited.clone(), graph, paths, repeated);
        }
    }
}

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();

    // Load all the nodes.
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for line in file.trim().split('\n') {
        let mut parts = line.trim().split('-');
        let first = String::from(parts.next().unwrap());
        let second = String::from(parts.next().unwrap());
        if let Some(existing) = graph.get_mut(&first) {
            existing.push(second.clone());
        } else {
            graph.insert(first.clone(), vec![second.clone()]);
        }
        if let Some(existing) = graph.get_mut(&second) {
            existing.push(first);
        } else {
            graph.insert(second, vec![first]);
        }
    }

    let mut paths: Vec<Vec<&str>> = Vec::new();
    traverse("start", Vec::new(), &graph, &mut paths);
    println!("Paths for first part: {}", paths.len());

    let mut paths: Vec<Vec<&str>> = Vec::new();
    traverse2("start", Vec::new(), &graph, &mut paths, false);
    println!("Paths for second part: {}", paths.len());
}
