const INPUT: &str = include_str!("../input.txt");

fn main() {
    let nums: Vec<_> = INPUT.lines().map(|l| l.parse::<u16>().unwrap()).collect();
    let mut good_nums = vec![];
    for num in nums {
        let mut on_count = 0;
        for n in 0..u16::BITS {
            on_count += num >> n & 1;
        }
        if on_count % 2 == 0 {
            good_nums.push(num & 0b0111_1111_1111_1111);
        }
    }

    let average = good_nums.iter().sum::<u16>() / good_nums.len() as u16;
    println!("Average is {average}")
}
