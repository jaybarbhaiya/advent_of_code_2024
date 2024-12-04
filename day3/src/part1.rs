use regex::Regex;

pub fn mull_it_over(input: &str) {
    let mut sum = 0;

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    for cap in re.captures_iter(input) {
        let num1: i32 = cap[1].parse().unwrap();
        let num2: i32 = cap[2].parse().unwrap();
        let product = num1 * num2;
        sum += product;
    }

    dbg!(sum);
}
