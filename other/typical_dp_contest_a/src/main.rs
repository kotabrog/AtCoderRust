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

use std::collections::{HashSet, VecDeque, HashMap};

fn main() {
    input! {
        n: usize,
        list: [i32; n],
    }

    let mut dp: Vec<Vec<HashMap<i32, i32>>> = vec![vec![]; 10001];

    let mut ans = HashSet::new();
    let mut queue = VecDeque::new();

    let mut add_map = HashMap::new();
    for p in &list {
        let value = add_map.get_mut(p);
        if let Some(v) = value {
            *v += 1;
        } else {
            add_map.insert(*p, 1);
        }
    }

    queue.push_back((vec![0], add_map));
    ans.insert(0);

    while !queue.is_empty() {
        let (vec, add_map) = queue.pop_front().unwrap();

        let mut check_vec = vec![];
        'outer: for v in vec {
            let dp_list = &dp[v as usize];
            for dp_map in dp_list {
                let mut add_inner_flag = false;
                for (key, value) in &add_map {
                    let dp_value = dp_map.get(key);
                    if let Some(dp_v) = dp_value {
                        if dp_v < value {
                            add_inner_flag = true;
                            break;
                        }
                    } else {
                        break;
                    }
                }
                if !add_inner_flag {
                    continue 'outer;
                }
            }
            check_vec.push(v);
            dp[v as usize].push(add_map.clone());
        }

        if check_vec.is_empty() {
            continue;
        }

        for k in add_map.keys() {
            let mut new_add_map = add_map.clone();
            let value = new_add_map.get_mut(k).unwrap();
            *value -= 1;
            if *value == 0 {
                new_add_map.remove(k);
            }
            let new_vec = check_vec.iter().map(|&x| x + *k).collect();
            for &s in &new_vec {
                ans.insert(s);
            }
            if !new_add_map.is_empty() {
                queue.push_back((new_vec, new_add_map));
            }
        }

        // if add_vec.len() == 1 {
        //     let a = add_vec[0];
        //     for &s in &vec {
        //         ans.insert(s + a);
        //     }
        //     continue;
        // }
        // for i in 0..add_vec.len() {
        //     let a = add_vec[i];
        //     let new_vec = vec.iter().map(|&x| x + a).collect();
        //     for &s in &new_vec {
        //         ans.insert(s);
        //     }
        //     let mut new_add_vec = add_vec.clone();
        //     new_add_vec.remove(i);
        //     queue.push_front((new_vec, new_add_vec));
        // }
    }
    println!("{}", ans.len());
}
