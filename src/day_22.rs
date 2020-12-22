use std::collections::{HashSet, VecDeque};

pub fn day_22_1(lines: &[String]) -> usize {
    let (mut player1, mut player2) = parse(lines);

    while !player1.is_empty() && !player2.is_empty() {
        let play1 = player1.pop_front().unwrap();
        let play2 = player2.pop_front().unwrap();
        if play1 > play2 {
            player1.push_back(play1);
            player1.push_back(play2);
        } else {
            player2.push_back(play2);
            player2.push_back(play1);
        }
    }

    compute_score(&player1, &player2)
}

pub fn day_22_2(lines: &[String]) -> usize {
    let (player1, player2) = parse(lines);

    let (result1, result2) = recursive_combat(player1, player2);

    compute_score(&result1, &result2)
}

fn parse(lines: &[String]) -> (VecDeque<usize>, VecDeque<usize>) {
    let mut players = lines.split(|l| l == "");
    (
        players
            .next()
            .unwrap()
            .iter()
            .skip(1)
            .map(|s| s.parse().unwrap())
            .collect(),
        players
            .next()
            .unwrap()
            .iter()
            .skip(1)
            .map(|s| s.parse().unwrap())
            .collect(),
    )
}

fn compute_score(player1: &VecDeque<usize>, player2: &VecDeque<usize>) -> usize {
    if player1.is_empty() {
        player2
            .iter()
            .enumerate()
            .map(|(i, n)| (player2.len() - i) * n)
            .sum()
    } else {
        player1
            .iter()
            .enumerate()
            .map(|(i, n)| (player1.len() - i) * n)
            .sum()
    }
}

fn recursive_combat(
    player1: VecDeque<usize>,
    player2: VecDeque<usize>,
) -> (VecDeque<usize>, VecDeque<usize>) {
    let mut player1_history: HashSet<VecDeque<usize>> = HashSet::new();
    let mut player2_history: HashSet<VecDeque<usize>> = HashSet::new();

    let mut player1 = player1;
    let mut player2 = player2;
    while !player1.is_empty() && !player2.is_empty() {
        if !player1_history.insert(player1.clone()) || !player2_history.insert(player2.clone()) {
            return (player1, VecDeque::new());
        }

        let play1 = player1.pop_front().unwrap();
        let play2 = player2.pop_front().unwrap();
        if play1 > player1.len() || play2 > player2.len() {
            if play1 > play2 {
                player1.push_back(play1);
                player1.push_back(play2);
            } else {
                player2.push_back(play2);
                player2.push_back(play1);
            }
        } else {
            let (_, result2) = recursive_combat(
                player1.iter().copied().take(play1).collect(),
                player2.iter().copied().take(play2).collect(),
            );
            if result2.is_empty() {
                player1.push_back(play1);
                player1.push_back(play2);
            } else {
                player2.push_back(play2);
                player2.push_back(play1);
            }
        }
    }
    (player1, player2)
}

#[cfg(test)]
mod tests {
    use crate::day_22::{day_22_1, day_22_2};

    fn test_data() -> Vec<String> {
        "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10"
        .lines()
        .map(String::from)
        .collect()
    }

    #[test]
    fn test_day_22_1() {
        assert_eq!(day_22_1(&test_data()), 306);
    }

    #[test]
    fn test_day_22_2() {
        assert_eq!(day_22_2(&test_data()), 291);
    }
}
