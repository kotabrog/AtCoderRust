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

fn partial_match(s: &Vec<char>, t: &str, pos: usize) -> bool {
    let mut i = 0;
    while i < t.len() {
        if s.len() <= pos + i || s[pos + i] != t.chars().nth(i).unwrap() {
            return false;
        }
        i += 1;
    }

    true
}

fn main() {
    input! {
        s: chars,
    }

    let mut pos = 0;
    let mut prev_pos = 0;
    let mut prev_candidate_pos = 0;
    let mut candidate_start_pos = 0;
    let candidate = vec!["dream", "dreamer", "erase", "eraser"];

    while pos < s.len() {
        let mut matched = false;
        for i in candidate_start_pos..candidate.len() {
            let c = candidate[i];
            if partial_match(&s, c, pos) {
                candidate_start_pos = 0;
                prev_pos = pos;
                pos += c.len();
                matched = true;
                prev_candidate_pos = i + 1;
                break;
            }
        }
        if !matched {
            if pos == prev_pos {
                println!("NO");
                return;
            }
            pos = prev_pos;
            candidate_start_pos = prev_candidate_pos;
        }
    }
    println!("YES");
}
