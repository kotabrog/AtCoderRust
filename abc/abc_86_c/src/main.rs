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
        t_x_y: [[i32; 3]; n],
    }

    let mut prev_t = 0;
    let mut prev_x = 0;
    let mut prev_y = 0;

    for i in 0..n {
        let t = t_x_y[i][0];
        let x = t_x_y[i][1];
        let y = t_x_y[i][2];

        let dt = t - prev_t;
        let dx = (x - prev_x).abs();
        let dy = (y - prev_y).abs();
        let d = dx + dy;
        if dt < d || (dt - d) % 2 != 0 {
            println!("No");
            return;
        }
        prev_t = t;
        prev_x = x;
        prev_y = y;
    }
    println!("Yes")
}
