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

// use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        list: [(i64, i64); n],
    }

    let mut list = list;
    list.sort_by_key(|&(x, l)| x + l);

    let mut ans = n;
    let mut skip_to_index = 0;
    for i in 0..list.len() {
        if i < skip_to_index {
            continue;
        }
        let (x, l) = list[i];
        for j in i + 1..list.len() {
            let (x0, l0) = list[j];
            if x + l <= x0 - l0 {
                skip_to_index = j;
                ans -= j - i - 1;
                break;
            }
        }
    }
    ans -= list.len() - skip_to_index - 1;

    println!("{}", ans);

    // let mut list = list;
    // list.sort_by_key(|&(x, l)| x + l);
    // list.reverse();

    // let mut covered_count_sum = 0;
    // let mut covered_count = vec![0; n];
    // for (i, (x, l)) in list.iter().enumerate() {
    //     for (j, (x0, l0)) in list[i + 1..].iter().enumerate() {
    //         if x - l < x0 + l0 {
    //             covered_count[i] += 1;
    //             covered_count[i + j + 1] += 1;
    //             covered_count_sum += 1;
    //         }
    //     }
    // }

    // covered_count.sort();
    // covered_count.reverse();
    // let mut ans = n;
    // for i in 0..n {
    //     if covered_count_sum == 0 {
    //         break;
    //     }
    //     covered_count_sum -= covered_count[i] * 2;
    //     ans -= 1;
    // }

    // println!("{}", ans);

    // let mut list = list;
    // list.sort_by_key(|&(x, l)| x + l);
    // list.reverse();

    // let mut covered_count = Vec::with_capacity(n);
    // for (i, (x, l)) in list.iter().enumerate() {
    //     let mut count = 0;
    //     let mut covered = vec![];
    //     for (x0, l0) in &list[i + 1..] {
    //         if x - l < x0 + l0 {
    //             count += 1;
    //             covered.push(i);
    //         }
    //     }
    //     covered_count.push((covered, count));
    // }

    // loop {
    //     let mut max_count = 0;
    //     let mut max_index = 0;
    //     for (i, (_, count)) in covered_count.iter().enumerate() {
    //         if *count > max_count {
    //             max_count = *count;
    //             max_index = i;
    //         }
    //     }

    //     if max_count == 0 {
    //         break;
    //     }

    //     let (covered, _) = covered_count.remove(max_index);
    //     for i in covered {
    //         let (_, count) = &mut covered_count[i];
    //         *count -= 1;
    //     }
    // }

    // println!("{}", covered_count.len());

    // let mut list = list;
    // list.sort_by_key(|&(a, _)| a);

    // let mut ans = 0;

    // let mut queue = VecDeque::new();

    // if n == 1 {
    //     println!("{}", 1);
    //     return;
    // }

    // queue.push_back((0, 1, 0));

    // while !queue.is_empty() {
    //     let (i, j, count) = queue.pop_front().unwrap();
    //     let (x0, l0) = list[i];
    //     let (x1, l1) = list[j];
    //     let x0_right = x0 + l0;
    //     let x1_left = x1 - l1;

    //     if x1_left < x0_right {
    //         if j + 1 < n {
    //             queue.push_back((i, j + 1, count));
    //             queue.push_back((j, j + 1, count));
    //         } else {
    //             ans = std::cmp::max(ans, count + 1);
    //         }
    //     } else {
    //         if j + 1 < n {
    //             queue.push_back((j, j + 1, count + 1));
    //         } else {
    //             ans = std::cmp::max(ans, count + 2);
    //         }
    //     }
    // }

    // println!("{}", ans);
}
