fn main() {
    let state = (0usize, [0usize;12]);
    let final_state = include_str!("../../inputs/diag_reports.txt").lines()
        .fold(state, |(state, one_counts), line| {
            let count = state + 1;
            // assert_eq!(line.len() == 12);
            let mut next_one_counts = [0usize; 12];
            for i in 0..12 {
                next_one_counts[i] = one_counts[i] + match line.as_bytes()[i] {
                    b'1' => 1usize,
                    _ => 0usize,
                }
            }
            (count, next_one_counts)
        });
        let mut gamma = 0usize;
        for i in 0..12 {
            let bit: usize = if final_state.1[i] > (final_state.0 / 2) { 1 } else { 0 };
            gamma = (gamma << 1) + bit;
            println!("bit {} = {} -> gamma = {:b}", i, bit, gamma);
        }
        let mask = (1 << 12) - 1;
        let epsilon = !gamma & mask;
        println!("{}", gamma);
        println!("{}", epsilon);
        println!("{}", gamma * epsilon);
}