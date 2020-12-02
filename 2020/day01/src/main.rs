use std::fs;

/*
 * Challenge Specs #1
 *
 * [1721, 979, 366, 299, 675, 1456]
 * [299, 366, 675, 979, 1456, 1721]
 *
 * 299 + 1721 = 2020
 *
 * In this list, the two entries that sum to 2020 are 1721 and 299.
 * Multiplying them together produces 1721 * 299 = 514579, so the correct answer is 514579.
 *
 */
fn solution1(xs: &Vec<i32>) {
    let mut ys = xs.clone();
    loop {
        ys.rotate_right(1);
        let res: Vec<_> = xs
            .iter()
            .zip(ys.iter())
            .filter(|(&x, &y)| x + y == 2020)
            .map(|(&x, &y)| x * y)
            .collect();

        if !res.is_empty() {
            println!("res: {:?}", res);
            break;
        }
    }
}

/*
 * Challenge Specs 2
 *
 * Using the above example again, the three entries that sum to 2020 are 979, 366, and 675.
 * Multiplying them together produces the answer, 241861950
 *
 */
fn solution2(xs: &Vec<i32>) {
    let mut ys = xs.clone();
    let mut zs = xs.clone();
    zs.rotate_right(1);
    for x in xs {
        ys.rotate_right(1);
        zs.rotate_right(1);
        let res: Vec<_> = ys
            .iter()
            .zip(zs.iter())
            .filter(|(&y, &z)| x + y + z == 2020)
            .map(|(&y, &z)| x * y * z)
            .collect();

        if !res.is_empty() {
            println!("res: {:?}", res);
            break;
        }
    }
}

fn main() {
    let mut contents: Vec<i32> = fs::read_to_string("input.txt")
        .expect("something went wrong")
        .split("\n")
        .filter(|x| !x.is_empty())
        .map(|x| x.parse::<i32>().unwrap())
        .collect();

    contents.sort();

    solution1(&contents);
    solution2(&contents);
}
