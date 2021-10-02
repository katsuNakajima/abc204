use std::usize;

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
    () => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            line.pop();
            line
        }
    )
}

#[allow(unused_macros)]
macro_rules! parse_vec {
    ( $t:ty ) => (
        {
            let mut line = String::new();
            ::std::io::stdin().read_line(&mut line).unwrap();
            let iter = line.split_whitespace();
            iter.map(|v| v.parse::<$t>().unwrap()).collect::<Vec<_>>()
        }
    )
}

fn main() {
    let (x, y) = parse_line!(usize,usize);
    let mut ans = 0 as usize;
    match x {
        0 => if y == 0 {
            ans = 0;
        }
        else if y == 1 {
            ans = 2;
        }
        else if y == 2 {
            ans = 1;
        },
        1 => if y == 0 {
            ans = 2;
        }
        else if y == 1 {
            ans = 1;
        }
        else if y == 2 {
            ans = 0;
        },
        2 => if y == 0 {
            ans = 1;
        }
        else if y == 1 {
            ans = 0;
        }
        else if y == 2 {
            ans = 2;
        },
        _ => (),
    }
    println!("{}",ans);
}
