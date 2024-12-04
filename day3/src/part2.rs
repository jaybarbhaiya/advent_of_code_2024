use regex::Regex;

fn find_multiply_pairs(find_line: &str) -> i32 {
    let mut p_sum = 0;
    let re = Regex::new(r"mul\((\d+,\d+)\)").unwrap();

    for cap in re.captures_iter(find_line) {
        if let Some(pair) = cap.get(1) {
            let int_pair: Vec<i32> = pair
                .as_str()
                .split(',')
                .filter_map(|num| num.parse::<i32>().ok())
                .collect();

            if int_pair.len() == 2 {
                p_sum += int_pair[0] * int_pair[1];
            }
        }
    }

    p_sum
}

pub fn mull_it_over(input: &str) {
    let mega_line = input.trim();

    let inst_blocks: Vec<&str> = mega_line.split("don't()").collect();
    let mut mul_result = 0;

    if let Some(first_block) = inst_blocks.get(0) {
        mul_result += find_multiply_pairs(first_block);
    }

    for block in inst_blocks.iter().skip(1) {
        if let Some(i) = block.find("do()") {
            let do_string = &block[i..];
            mul_result += find_multiply_pairs(do_string);
        }
    }

    dbg!(mul_result);
}
