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
        h: usize,
        w: usize,
        s: [usize; 2],
        g: [usize; 2],
        map: [chars; h],
    }

    let g = (g[0] - 1, g[1] - 1);

    let mut queue = VecDeque::new();
    queue.push_back((s[0] - 1, s[1] - 1, 0));

    let mut visited = vec![vec![false; w]; h];

    while !queue.is_empty() {
        let (i, j, count) = queue.pop_front().unwrap();

        if (i, j) == g {
            println!("{}", count);
            return;
        }

        if map[i][j] == '#' || visited[i][j] {
            continue;
        }

        let count = count + 1;
        if i > 0 {
            queue.push_back((i - 1, j, count));
        }
        if i < h - 1 {
            queue.push_back((i + 1, j, count));
        }
        if j > 0 {
            queue.push_back((i, j - 1, count));
        }
        if j < w - 1 {
            queue.push_back((i, j + 1, count));
        }

        visited[i][j] = true;
    }
}
