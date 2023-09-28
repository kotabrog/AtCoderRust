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
        map: [chars; h],
    }

    let mut queue = VecDeque::new();

    'outer: for i in 0..h {
        for j in 0..w {
            if map[i][j] == 's' {
                queue.push_back((i, j));
                break 'outer;
            }
        }
    }

    let mut visited = vec![vec![false; w]; h];
    while !queue.is_empty() {
        let (i, j) = queue.pop_front().unwrap();
        if visited[i][j] {
            continue;
        }
        visited[i][j] = true;
        if map[i][j] == 'g' {
            println!("Yes");
            return;
        }
        if i > 0 && map[i - 1][j] != '#' {
            queue.push_front((i - 1, j));
        }
        if i < h - 1 && map[i + 1][j] != '#' {
            queue.push_front((i + 1, j));
        }
        if j > 0 && map[i][j - 1] != '#' {
            queue.push_front((i, j - 1));
        }
        if j < w - 1 && map[i][j + 1] != '#' {
            queue.push_front((i, j + 1));
        }
    }
    println!("No")
}
