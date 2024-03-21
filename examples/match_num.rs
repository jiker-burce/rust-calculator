use lazy_static::lazy_static;
use regex::Regex;

fn main() {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d*\.?\d*)([+|\-|*|/]?)?").unwrap();
    }
    let test_str = "00123.45 + 0.12 * 0.0-44";
    // let mut i = 0;
    let mut result = String::new();
    for cap in RE.captures_iter(test_str) {
        let num = match  &cap[0].parse::<f32>() {
            Ok(num) => num.to_string(),
            Err(_e) => cap[0].to_string()
        };
        result.push_str(&num);
        // println!("{:?}: {:?},{:?}", i, num, &cap[1]);
        // i += 1;
        // let num_str = &cap[1];
        // let op_str = &cap[2];
        // let num = if num_str.starts_with("0") && num_str.matches('.').count() < 2 {
        //     &num_str[num_str.trim_start_matches('0').find(|c: char| c != '.').unwrap_or(0)..]
        // } else {
        //     num_str
        // };
        // println!("{}", num);
        // println!("{}", op_str);
    }
    println!("result: {:?}", result);
}