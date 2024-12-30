use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../inputs/day-23.txt");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let mut adj: HashMap<&str, Vec<&str>> = HashMap::new();

    input.lines().for_each(|line| {
        let (n1, n2) = line.split_once("-").unwrap();

        adj.entry(n1).or_default().push(n2);
        adj.entry(n2).or_default().push(n1);
    });

    let mut ans = 0;

    let mut visited = HashSet::new();

    for (k, v) in adj.iter() {
        if k.starts_with("t") {
            for neighbor in v {
                if k == neighbor {
                    continue;
                }

                for neighbor2 in adj.get(neighbor).unwrap() {
                    if neighbor2 == neighbor
                        || neighbor2 == k
                        || !v.contains(neighbor2)
                        || visited.contains(&(k, neighbor, neighbor2))
                    {
                        continue;
                    }

                    visited.insert((k, neighbor, neighbor2));
                    visited.insert((k, neighbor2, neighbor));
                    visited.insert((neighbor, k, neighbor2));
                    visited.insert((neighbor, neighbor2, k));
                    visited.insert((neighbor2, neighbor, k));
                    visited.insert((neighbor2, k, neighbor));

                    ans += 1;
                }
            }
        }
    }

    ans
}

fn find_network(nodes: HashSet<String>, adj: &HashMap<&str, HashSet<&str>>) -> HashSet<String> {
    let mut largest_network = nodes.clone();

    for (k, v) in adj.iter() {
        if !nodes.contains(&k.to_string()) && nodes.iter().all(|n| v.contains(n.as_str())) {
            let mut new_nodes = nodes.clone();
            new_nodes.insert(k.to_string());
            let network = find_network(new_nodes, adj);

            if network.len() > largest_network.len() {
                largest_network = network;
            }
        }
    }

    largest_network
}

fn bron_kerbosch<'a>(
    r: &mut HashSet<&'a str>,
    p: &mut HashSet<&'a str>,
    x: &mut HashSet<&'a str>,
    adj: &HashMap<&'a str, HashSet<&'a str>>,
) -> HashSet<&'a str> {
    if p.is_empty() && x.is_empty() {
        return r.clone();
    }

    let mut largest_clique = HashSet::new();

    for v in p.clone().iter() {
        let mut n_r = r.clone();
        n_r.insert(v);

        let mut n_p = p
            .clone()
            .intersection(&adj[v])
            .cloned()
            .collect::<HashSet<_>>();
        let mut n_x = x
            .clone()
            .intersection(&adj[v])
            .cloned()
            .collect::<HashSet<_>>();
        let clique = bron_kerbosch(&mut n_r, &mut n_p, &mut n_x, adj);

        if clique.len() > largest_clique.len() {
            largest_clique = clique;
        }

        p.remove(v);
        x.insert(v);
    }

    largest_clique
}

fn part2(input: &str) -> String {
    let mut adj: HashMap<&str, HashSet<&str>> = HashMap::new();

    input.lines().for_each(|line| {
        let (n1, n2) = line.split_once("-").unwrap();

        adj.entry(n1).or_default().insert(n2);
        adj.entry(n2).or_default().insert(n1);
    });

    // let mut largest_network = HashSet::new();
    //
    // for n in adj.keys() {
    //     let mut nodes = HashSet::new();
    //     nodes.insert(n.to_string());
    //
    //     if largest_network.len() > adj[n].len() {
    //         continue;
    //     }
    //
    //     let net = find_network(nodes, &adj);
    //
    //     if net.len() > largest_network.len() {
    //         largest_network = net;
    //     }
    // }
    let mut r = HashSet::new();
    let mut p = adj.keys().cloned().collect::<HashSet<_>>();
    let mut x = HashSet::new();
    let largest_network = bron_kerbosch(&mut r, &mut p, &mut x, &adj);

    let mut largest_network = largest_network.into_iter().collect::<Vec<_>>();

    largest_network.sort();

    largest_network.join(",")
}

#[allow(dead_code)]
const TEST_INPUT: &str = "\
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";

#[test]
fn test_part1() {
    assert_eq!(part1(TEST_INPUT), 7);
}

#[test]
fn test_part2() {
    assert_eq!(part2(TEST_INPUT), "co,de,ka,ta".to_string());
}
