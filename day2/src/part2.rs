pub fn safe_report_tolerate_single_bad_level(input: &str) {
    let mut safe_report_counter = 0;
    let input = input.trim();
    for line in input.lines() {
        let report: Vec<&str> = line.trim().split_whitespace().collect();
        let report: Vec<i32> = report
            .into_iter()
            .map(|s| s.parse().expect("Failed to parse i32"))
            .collect();

        if check_safety(&report, None) {
            safe_report_counter += 1;
        }
    }
    dbg!(safe_report_counter);
}

fn check_safety(check_report: &Vec<i32>, remove_index: Option<usize>) -> bool {
    // Clone the list to avoid modifying the original
    let mut check_report = check_report.clone();

    if let Some(index) = remove_index {
        if index >= check_report.len() {
            return false; // Ensure the index is valid
        }
        check_report.remove(index); // Remove the element at the given index
    }

    let mut safe = true;

    // Determine if the list is increasing
    let increasing = check_report[1] > check_report[0];

    for n in 0..check_report.len() - 1 {
        let difference = (check_report[n + 1] - check_report[n]).abs();

        if !(1..=3).contains(&difference) {
            safe = false;
        } else if increasing && check_report[n + 1] < check_report[n] {
            safe = false;
        } else if !increasing && check_report[n + 1] > check_report[n] {
            safe = false;
        }

        if !safe {
            if remove_index.is_none() {
                // If nothing has been removed yet, recursively check with numbers removed
                return check_safety(&check_report, Some(n))
                    || check_safety(&check_report, Some(n + 1))
                    || (n > 0 && check_safety(&check_report, Some(n - 1)));
            } else {
                // Already checking with a removed index, so it's not safe
                break;
            }
        }
    }

    safe
}
