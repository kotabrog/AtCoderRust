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
        x: i32,
    }
    let d = x % 10;
    let c = (x / 10) % 10;
    let b = (x / 100) % 10;
    let a = x / 1000;

    for flag in 0..2u32.pow(3) {
        let mut temp = a;
        temp = temp + if flag & 0b1 == 0b1 { b } else { -b };
        temp = temp + if flag & 0b10 == 0b10 { c } else { -c };
        temp = temp + if flag & 0b100 == 0b100 { d } else { -d };
        if temp == 7 {
            println!("{}{}{}{}{}{}{}=7", 
                     a,
                     if flag & 0b1 == 0b1 { '+' } else { '-' },
                     b,
                     if flag & 0b10 == 0b10 { '+' } else { '-' },
                     c,
                     if flag & 0b100 == 0b100 { '+' } else { '-' },
                     d
            );
            return ();
        }
    }
}
