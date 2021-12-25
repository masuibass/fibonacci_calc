use std::collections::HashMap;
use std::io;

fn main() {
    loop {
        println!("Please input n");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let n: u64 = match input.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };

        let mut memo: HashMap<u64, u64> = HashMap::new();
        memo.insert(0, 0);
        memo.insert(1, 1);
        println!("{}", fibonacci_memo(n, &mut memo));
        break;
    }
}

fn fibonacci_memo(n: u64, m: &mut HashMap<u64, u64>) -> u64 {
    return match m.get(&n) {
        None => {
            let new_num = fibonacci_memo(n - 2, m) + fibonacci_memo(n - 1, m);
            m.insert(n, new_num);
            new_num
        }
        _ => m[&n],
    };
}
