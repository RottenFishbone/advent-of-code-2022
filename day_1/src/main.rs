fn main() {
    // Parse input into Vec
    let mut global_max = 0u64;
    let mut running_max = 0u64;
    for line in include_str!("../input").lines() {
        if line.is_empty() {
            if running_max > global_max {
                global_max = running_max;
            }

            running_max = 0;
            continue;
        }


        running_max += line.parse::<u64>().unwrap();
    }

    println!("{}", global_max);

}
