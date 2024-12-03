pub fn safe_reports(input: &str) {
    let mut safe_report_counter = 0;
    let input = input.trim();
    for line in input.lines() {
        let mut is_unsafe = false;
        let mut is_ascending = true;
        let report: Vec<&str> = line.trim().split_whitespace().collect();
        let report: Vec<i32> = report
            .into_iter()
            .map(|s| s.parse().expect("Failed to parse i32"))
            .collect();
        for i in 0..report.len() {
            if i == 0 && report[i] > report[i + 1] {
                is_ascending = false;
            }
            if i != report.len() - 1 {
                is_unsafe = {
                    let diff;
                    if is_ascending {
                        diff = report[i + 1] - report[i];
                    } else {
                        diff = report[i] - report[i + 1];
                    }
                    if diff <= 0 || diff > 3 {
                        true
                    } else {
                        false
                    }
                };
                if is_unsafe {
                    break;
                }
            }
        }
        if !is_unsafe {
            safe_report_counter = safe_report_counter + 1;
        }
    }
    dbg!(safe_report_counter);
}
