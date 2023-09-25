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
        h: usize,
        w: usize,
        s: [chars; h],
    }

    let mut ans = vec![vec!['#'; w]; h];
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '#' {
                continue;
            }

            let mut count = 0;
            for k in -1..2 {
                for l in -1..2 {
                    if i as i32 + k < 0 || i as i32 + k >= h as i32 {
                        continue;
                    }
                    if j as i32 + l < 0 || j as i32 + l >= w as i32 {
                        continue;
                    }
                    if s[(i as i32 + k) as usize][(j as i32 + l) as usize] == '#' {
                        count += 1;
                    }
                }
            }
            ans[i][j] = format!("{}", count).chars().nth(0).unwrap();
        }
    }
    println!("{}", ans.iter().map(|x| x.iter().collect::<String>()).collect::<Vec<String>>().join("\n"));
}
