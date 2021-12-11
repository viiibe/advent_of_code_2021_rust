fn main() {
    let depths = include_str!("../input.txt")
        .lines()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<u32>>();

    let mut counter: usize = 3;
    let mut depth_count: u32 = 0;

    while counter < depths.len() {
        let a = depths[(counter - 3)..=(counter - 1)].iter().sum::<u32>();
        let b = depths[(counter - 2)..=counter].iter().sum::<u32>();
        if a < b {
            depth_count += 1;
        }
        counter += 1;
    }

    println! {"Number of times depth measurement increases = {}", depth_count};
}
