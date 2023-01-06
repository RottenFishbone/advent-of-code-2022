fn main() {
    const PAYOFF: [u64; 3] = [3, 0, 6];

    let sum_1: u64 = include_str!("../input")
        .lines()
        .map(|line| {
            let chrs: Vec<char> = line.to_ascii_lowercase().chars().collect();
            let play = (chrs[0] as u8 - 'a' as u8, chrs[2] as u8 - 'x' as u8);
            let payoff_id = (((play.1*2%3) + play.0)%3) as usize;
            PAYOFF[payoff_id] + play.1 as u64 + 1
        })
        .sum();

    let sum_2: u64 = include_str!("../input")
        .lines()
        .map(|line| {
            let chrs: Vec<char> = line.to_ascii_lowercase().chars().collect();
            let play = (chrs[0] as u8 - 'a' as u8, chrs[2] as u8 - 'x' as u8);
            // Remap `play` from loss/draw/win into r/p/s
            let play = (play.0, (play.1 + play.0 + 2)%3);
            let payoff_id = (((play.1*2%3) + play.0)%3) as usize;
            PAYOFF[payoff_id] + play.1 as u64 + 1
        })
        .sum();

    println!("Part 1: {}\nPart 2: {}", sum_1, sum_2);
}       


/*
    Mapping which payoff we get for each combo

    0, 0 = 3 -> id: 0 + 0 % 3
    0, 1 = 0 -> id: 0 + 1 % 3
    0, 2 = 6 -> id: 0 + 2 % 3

    1, 0 = 6 -> id: 1*2 % 3 + 0 % 3
    1, 1 = 3 -> id: 1*2 % 3 + 1 % 3
    1, 2 = 0 -> id: 1*2 % 3 + 2 % 3

    2, 0 = 0 -> id: 2*2 % 3 + 0 % 3 = 1
    2, 1 = 6 -> id: 2*2 % 3 + 1 % 3 = 2
    2, 2 = 3 -> id: 2*2 % 3 + 2 % 3 = 0


    Part two: remapping to r/p/s

    0,0 -> 0,2 -> y =   0+(0+2)%3
    0,1 -> 0,0 ->       1+2%3
    0,2 -> 0,1 ->       2+2%3

    1,0 -> 1,0 -> y =   0+(1+2)%3
    1,1 -> 1,1 
    1,2 -> 1,2

    2,0 -> 2,1 -> y =   0+(2+2)%3
    2,1 -> 2,2
    2,2 -> 2,0


*/
