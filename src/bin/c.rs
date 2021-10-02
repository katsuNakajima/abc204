use petgraph::graph::Graph;
use petgraph::visit::{Dfs, NodeIndexable};
use petgraph::Directed;
use std::{u32, usize};

#[allow(unused_macros)]
macro_rules! parse_line {
    ( $t:ty ) => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            iter.next().unwrap().parse::<$t>().unwrap()
        }
    );

    ( $( $t:ty), +) => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.split_whitespace();
            ( $(iter.next().unwrap().parse::<$t>().unwrap()),* )
        }
    );
}

#[allow(unused_macros)]
macro_rules! read_line {
    () => {{
        let mut line = String::new();
        ::std::io::stdin().read_line(&mut line).unwrap();
        line.pop();
        line
    }};
}

#[allow(unused_macros)]
macro_rules! parse_vec {
    ( $t:ty ) => {{
        let mut line = String::new();
        ::std::io::stdin().read_line(&mut line).unwrap();
        let iter = line.split_whitespace();
        iter.map(|v| v.parse::<$t>().unwrap()).collect::<Vec<_>>()
    }};
}

fn main() {
    let (n, m) = parse_line!(usize, usize);
    let mut path: Vec<(u32, u32)> = Vec::new();
    let mut ans = 0 as u32;
    for _ in 0..m {
        let mut ab = parse_line!(u32, u32);
        ab.0 -= 1;
        ab.1 -= 1;
        path.push(ab);
    }
    if m != 0 {
        let graph = Graph::<(), (), Directed, u32>::from_edges(&path);
        for i in 0..n {
            let mut dfs = Dfs::new(&graph, graph.from_index(i));
            while let Some(_) = dfs.next(&graph) {
                ans += 1;
            }
        }
    } else {
        ans = n as u32;
    }
    println!("{}", ans);
}
