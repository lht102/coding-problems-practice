struct Solution;

impl Solution {
    pub fn match_players_and_trainers(players: Vec<i32>, trainers: Vec<i32>) -> i32 {
        let (mut players, mut trainers) = (players, trainers);
        players.sort_unstable();
        trainers.sort_unstable();
        let mut res = 0;
        let mut i = 0;
        for p in players {
            while i < trainers.len() && trainers[i] < p {
                i += 1;
            }
            if i == trainers.len() {
                break;
            }
            res += 1;
            i += 1;
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let players = vec![4, 7, 9];
        let trainers = vec![8, 2, 5, 8];
        assert_eq!(Solution::match_players_and_trainers(players, trainers), 2);

        let players = vec![1, 1, 1];
        let trainers = vec![10];
        assert_eq!(Solution::match_players_and_trainers(players, trainers), 1);
    }
}
