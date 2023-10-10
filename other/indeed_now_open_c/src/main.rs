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
        n: usize,
        m: usize,
        company: [(i32, i32, i32, i32); n],
        person: [(i32, i32, i32); m],
    }

    let mut dp = vec![vec![vec![0; 101]; 101]; 101];

    for (a, b, c, w) in &company {
        dp[*a as usize][*b as usize][*c as usize] = std::cmp::max(dp[*a as usize][*b as usize][*c as usize], *w);
    }

    for i in 0..101 {
        for j in 0..101 {
            for k in 0..101 {
                if i > 0 {
                    dp[i][j][k] = std::cmp::max(dp[i][j][k], dp[i - 1][j][k]);
                }
                if j > 0 {
                    dp[i][j][k] = std::cmp::max(dp[i][j][k], dp[i][j - 1][k]);
                }
                if k > 0 {
                    dp[i][j][k] = std::cmp::max(dp[i][j][k], dp[i][j][k - 1]);
                }
            }
        }
    }

    for (x, y, z) in &person {
        println!("{}", dp[*x as usize][*y as usize][*z as usize]);
    }
}
