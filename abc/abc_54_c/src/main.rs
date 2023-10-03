macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}

macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};

    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };

    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };

    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut edges = vec![vec![false; n]; n];
    for (a, b) in ab {
        edges[a - 1][b - 1] = true;
        edges[b - 1][a - 1] = true;
    }

    let mut ans = 0;

    let mut queue = VecDeque::new();
    let mut visited = vec![false; n];
    visited[0] = true;
    queue.push_back((0, visited, 1));
    while !queue.is_empty() {
        let (v, visited, count) = queue.pop_front().unwrap();
        if count == n {
            ans += 1;
            continue;
        }
        for i in 0..n {
            if edges[v][i] && !visited[i] {
                let mut visited = visited.clone();
                visited[i] = true;
                queue.push_back((i, visited, count + 1));
            }
        }
    }
    println!("{}", ans)
}
