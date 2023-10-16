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

fn main() {
    input! {
        h: i32,
        n: usize,
        list: [(i32, i32); n],
    }

    let mut dp = vec![vec![std::i32::MAX; 10001]; 10001];
    dp[0][0] = 0;

    let mut max_a = 0;
    for (a, b) in list {
        dp[0][a as usize] = std::cmp::min(dp[0][a as usize], b);
        max_a = std::cmp::max(max_a, a);
    }

    let mut min_cost = std::i32::MAX;
    for i in (1..max_a as usize + 1).rev() {
        min_cost = std::cmp::min(min_cost, dp[0][i]);
        dp[0][i] = min_cost;
    }

    for i in 1..h as usize + 1 {
        dp[i][i] = dp[i - 1][i];
        for j in 1..max_a as usize + 1 {
            if i + j > h as usize {
                break;
            }
            dp[i][i + j] = std::cmp::min(dp[i - 1][i + j], dp[i][i] + dp[0][j]);
        }
    }
    println!("{}", dp[h as usize][h as usize]);
}
