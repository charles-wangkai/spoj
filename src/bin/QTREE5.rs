// https://www.hankcs.com/program/algorithm/poj-3415-common-substrings.html

#![allow(non_snake_case)]

use std::{
    cmp::Reverse,
    collections::BinaryHeap,
    io::{stdin, BufRead, BufReader},
    usize,
};

fn main() {
    let mut br = BufReader::new(stdin());

    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let N: i32 = split.next().unwrap().parse().unwrap();
    let mut a = Vec::new();
    let mut b = Vec::new();
    for _ in 0..N - 1 {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        let mut split = line.split_whitespace();
        a.push(split.next().unwrap().parse().unwrap());
        b.push(split.next().unwrap().parse().unwrap());
    }
    let mut line = String::new();
    br.read_line(&mut line).unwrap();
    let mut split = line.split_whitespace();
    let Q = split.next().unwrap().parse().unwrap();
    let mut instructions = Vec::new();
    for _ in 0..Q {
        let mut line = String::new();
        br.read_line(&mut line).unwrap();
        instructions.push(line);
    }

    println!("{}", solve(&a, &b, &instructions));
}

fn solve(a: &[usize], b: &[usize], instructions: &[String]) -> String {
    let N = a.len() + 1;

    let mut adj_vecs = vec![Vec::new(); N];
    for i in 0..a.len() {
        adj_vecs[a[i] - 1].push(b[i] - 1);
        adj_vecs[b[i] - 1].push(a[i] - 1);
    }

    let centroids = compute_centroids(&adj_vecs);

    let mut ancestors = vec![vec![None; ((usize::BITS - N.leading_zeros()) as usize) + 2]; N];

    let mut depths = vec![0; N];
    search(&mut depths, &mut ancestors, &adj_vecs, usize::MAX, 0, 0);

    for j in 1..ancestors[0].len() {
        for i in 0..ancestors.len() {
            ancestors[i][j] = ancestors[i][j - 1].and_then(|a| ancestors[a][j - 1]);
        }
    }

    let mut heaps = vec![BinaryHeap::new(); N];

    let mut result = Vec::new();
    let mut whites = vec![false; N];
    for instruction in instructions {
        let mut split = instruction.split_whitespace();
        let kind: char = split.next().unwrap().parse().unwrap();
        let v = split.next().unwrap().parse::<usize>().unwrap() - 1;
        if kind == '0' {
            whites[v] ^= true;
            if whites[v] {
                let mut node = v;
                while node != usize::MAX - 1 {
                    let distance = compute_distance(&depths, &ancestors, v, node);
                    heaps[node].push((Reverse(distance), v, distance));

                    node = centroids[node];
                }
            }
        } else {
            let mut min_distance = i32::MAX;
            let mut node = v;
            while node != usize::MAX - 1 {
                let white_min_distance = find_white_min_distance(&mut heaps, &whites, node);
                if white_min_distance != i32::MAX {
                    min_distance = min_distance
                        .min(compute_distance(&depths, &ancestors, v, node) + white_min_distance);
                }

                node = centroids[node];
            }

            result.push(if min_distance == i32::MAX {
                -1
            } else {
                min_distance
            });
        }
    }

    result
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

fn find_white_min_distance(
    heaps: &mut Vec<BinaryHeap<(Reverse<i32>, usize, i32)>>,
    whites: &[bool],
    node: usize,
) -> i32 {
    while !heaps[node].is_empty() && !whites[heaps[node].peek().unwrap().1] {
        heaps[node].pop();
    }

    heaps[node].peek().map_or(i32::MAX, |e| e.2)
}

fn compute_distance(depths: &[i32], ancestors: &[Vec<Option<usize>>], u: usize, v: usize) -> i32 {
    depths[u] + depths[v] - 2 * depths[find_lca(depths, ancestors, u, v)]
}

fn find_lca(depths: &[i32], ancestors: &[Vec<Option<usize>>], mut u: usize, mut v: usize) -> usize {
    if depths[u] > depths[v] {
        return find_lca(depths, ancestors, v, u);
    }

    for k in 0..ancestors[0].len() {
        if (((depths[v] - depths[u]) >> k) & 1) == 1 {
            v = ancestors[v][k].unwrap();
        }
    }
    if u == v {
        return u;
    }

    for k in (0..ancestors[0].len()).rev() {
        if ancestors[u][k] != ancestors[v][k] {
            u = ancestors[u][k].unwrap();
            v = ancestors[v][k].unwrap();
        }
    }

    ancestors[u][0].unwrap()
}

fn search(
    depths: &mut [i32],
    ancestors: &mut [Vec<Option<usize>>],
    adj_vecs: &[Vec<usize>],
    parent: usize,
    node: usize,
    depth: i32,
) {
    if parent != usize::MAX {
        ancestors[node][0] = Some(parent);
    }

    depths[node] = depth;

    for &adj in &adj_vecs[node] {
        if adj != parent {
            search(depths, ancestors, adj_vecs, node, adj, depth + 1);
        }
    }
}

fn compute_centroids(adj_vecs: &[Vec<usize>]) -> Vec<usize> {
    let N = adj_vecs.len();

    let mut centroids = vec![usize::MAX; N];

    let mut subtree_sizes = vec![0; N];
    compute_subtree_sizes(&mut subtree_sizes, adj_vecs, &centroids, usize::MAX, 0);

    search_centroids(
        &mut centroids,
        adj_vecs,
        &mut subtree_sizes,
        N as i32,
        usize::MAX - 1,
        usize::MAX,
        0,
    );

    centroids
}

fn search_centroids(
    centroids: &mut [usize],
    adj_vecs: &[Vec<usize>],
    subtree_sizes: &mut [i32],
    current_subtree_size: i32,
    prev_centroid: usize,
    parent: usize,
    node: usize,
) {
    for &adj in &adj_vecs[node] {
        if adj != parent
            && centroids[adj] == usize::MAX
            && 2 * subtree_sizes[adj] > current_subtree_size
        {
            search_centroids(
                centroids,
                adj_vecs,
                subtree_sizes,
                current_subtree_size,
                prev_centroid,
                node,
                adj,
            );

            return;
        }
    }

    centroids[node] = prev_centroid;

    for &adj in &adj_vecs[node] {
        if centroids[adj] == usize::MAX {
            compute_subtree_sizes(subtree_sizes, adj_vecs, centroids, usize::MAX - 1, adj);
            search_centroids(
                centroids,
                adj_vecs,
                subtree_sizes,
                subtree_sizes[adj],
                node,
                node,
                adj,
            );
        }
    }
}

fn compute_subtree_sizes(
    subtree_sizes: &mut [i32],
    adj_vecs: &[Vec<usize>],
    centroids: &[usize],
    parent: usize,
    node: usize,
) {
    subtree_sizes[node] = 1;

    for &adj in &adj_vecs[node] {
        if adj != parent && centroids[adj] == usize::MAX {
            compute_subtree_sizes(subtree_sizes, adj_vecs, centroids, node, adj);
            subtree_sizes[node] += subtree_sizes[adj];
        }
    }
}
