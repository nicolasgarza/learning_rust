fn main() {
    let vec = vec![3,2,5,1,4,4,4];
    println!("Median: {}", find_median(&vec));
    println!("Mode: {}", find_mode(&vec));
}

fn find_median(vec: &Vec<i32>) -> f64 {
    match vec.len() {
        0 => 0.0,
        _ => {
            let mut vec2 = vec.clone();
            vec2.sort();
            if vec2.len() & 1 == 0 {
                return (vec2[vec2.len()/2] + vec2[vec2.len()/2 - 1]) as f64 / 2.0;
            }
            vec2[vec2.len()/2] as f64
        }
    }
}

fn find_mode(vec: &Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    for &number in vec {
        let count = map.entry(number).or_insert(0);
        *count += 1;
    }

    let mut mode = 0;
    let mut max = 0;
    for (&key, &value) in &map {
        if value > max {
            max = value;
            mode = key;
        }
    }
    mode
}
