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

fn cmp_chars(a: &Vec<char>, b: &Vec<char>, index: usize) -> bool {
    for i in 0..b.len() {
        if a[index + i] != b[i] && a[index + i] != '?' {
            return false;
        }
    }
    true
}

fn main() {
    input! {
        s_d: chars,
        t: chars,
    }

    for i in (0..s_d.len() - t.len() + 1).rev() {
        if cmp_chars(&s_d, &t, i) {
            let mut s_d = s_d;
            for j in 0..t.len() {
                s_d[i + j] = t[j];
            }
            for j in 0..s_d.len() {
                if s_d[j] == '?' {
                    s_d[j] = 'a';
                }
            }
            println!("{}", s_d.into_iter().collect::<String>());
            return;
        }
    }
    println!("UNRESTORABLE")
}
