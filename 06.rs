fn parse_answers(input: &str) -> Vec<&str> {
    input.split("\n\n").collect()
}

fn count_anyone_answer(group: &str) -> usize {
    ('a'..='z')
        .filter(|character: &char| group.contains(*character))
        .count()
}

fn solve_part_one(answers: &Vec<&str>) {
    let answer_count_sum: usize = answers
        .iter()
        .map(|group| count_anyone_answer(&group))
        .sum();
    println!("The total answers count summed up is {}.", answer_count_sum);
}

fn count_everyone_answer(group: &str) -> usize {
    ('a'..='z')
        .filter(|character: &char| group.lines().all(|form| form.contains(*character)))
        .count()
}

fn solve_part_two(answers: &Vec<&str>) {
    let answer_count_sum: usize = answers
        .iter()
        .map(|group| count_everyone_answer(&group))
        .sum();
    println!("The total answers count summed up is {}.", answer_count_sum);
}

fn main() {
    let input = include_str!("06_data.txt");

    let answers = parse_answers(&input);

    solve_part_one(&answers);
    solve_part_two(&answers);
}
