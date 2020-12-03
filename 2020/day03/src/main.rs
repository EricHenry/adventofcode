use std::fs;

const TREE: char = '#';
struct Movement(usize, usize); // (Right, Down)

fn solve_1(Movement(right, down): &Movement, geology: &Vec<&str>) -> usize {
    geology
        .iter()
        .enumerate()
        .filter_map(|(i, g)| if i % down == 0 { Some(*g) } else { None })
        .collect::<Vec<&str>>()
        .iter()
        .enumerate() // re-enumerate
        .map(|(i, g)| {
            let col = (i * right) % g.len();
            match g.chars().nth(col) {
                Some(terrain) if terrain == TREE => 1,
                _ => 0,
            }
        })
        .sum()
}

fn solve_2(mvs: &Vec<Movement>, geology: &Vec<&str>) -> usize {
    mvs.iter().map(|m| solve_1(m, geology)).product()
}

fn main() {
    let geology = fs::read_to_string("input.txt").expect("something went wrong");
    let gs: Vec<&str> = geology.split("\n").filter(|x| !x.is_empty()).collect();

    let answer_1 = solve_1(&Movement(3, 1), &gs);
    let movements = vec![
        Movement(1, 1),
        Movement(3, 1),
        Movement(5, 1),
        Movement(7, 1),
        Movement(1, 2),
    ];
    let answer_2 = solve_2(&movements, &gs);
    println!("Answer 1: {}", answer_1);
    println!("Answer 2: {}", answer_2);
}
