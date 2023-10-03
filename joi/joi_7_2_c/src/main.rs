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

fn max_search(list: &Vec<i64>, value: i64, limit: i64) -> i64 {
    let mut left = 0;
    let mut right = list.len() - 1;

    if value + list[0] > limit {
        return value;
    }

    while left < right {
        let mid = (left + right) / 2;
        if list[mid] + value <= limit {
            if left == mid {
                right -= 1;
            }
            left = mid;
        } else {
            right = mid;
        }
    }
    value + list[left]
}

fn main() {
    input! {
        n: usize,
        m: i64,
        list: [i64; n],
    }

    let mut list_double = Vec::with_capacity(n * n);

    for i in 0..n {
        for j in 0..n {
            list_double.push(list[i] + list[j]);
        }
        list_double.push(list[i]);
    }

    list_double.sort();

    let mut ans = 0;
    for i in 0..list_double.len() {
        let value = list_double[i];
        if value > m {
            continue;
        }
        let max = max_search(&list_double, value, m);
        ans = std::cmp::max(ans, max);
    }

    println!("{}", ans);
}
