use std::collections::HashMap;
use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn find_all_recipes(
        recipes: Vec<String>,
        ingredients: Vec<Vec<String>>,
        supplies: Vec<String>,
    ) -> Vec<String> {
        let mut adj: HashMap<String, Vec<String>> = HashMap::new();
        let mut indeg: HashMap<String, usize> = HashMap::new();
        for (i, ele) in ingredients.iter().enumerate() {
            for e in ele {
                adj.entry(e.clone()).or_default().push(recipes[i].clone());
            }
            indeg.entry(recipes[i].clone()).or_insert(ele.len());
        }
        let mut q: VecDeque<String> = VecDeque::new();
        for s in &supplies {
            q.push_back(s.clone());
        }
        let mut res = Vec::new();
        while let Some(v) = q.pop_front() {
            for to in adj.get(&v).unwrap_or(&Vec::new()) {
                if let Some(cnt) = indeg.get_mut(to) {
                    *cnt -= 1;
                    if *cnt == 0 {
                        res.push(to.clone());
                        q.push_back(to.clone());
                    }
                }
            }
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let recipes = vec![String::from("bread")];
        let ingredients = vec![vec![String::from("yeast"), String::from("flour")]];
        let supplies = vec![
            String::from("yeast"),
            String::from("flour"),
            String::from("corn"),
        ];
        assert_eq!(
            Solution::find_all_recipes(recipes, ingredients, supplies),
            vec![String::from("bread")]
        );

        let recipes = vec![String::from("bread"), String::from("sandwich")];
        let ingredients = vec![
            vec![String::from("yeast"), String::from("flour")],
            vec![String::from("bread"), String::from("meat")],
        ];
        let supplies = vec![
            String::from("yeast"),
            String::from("flour"),
            String::from("meat"),
        ];
        assert_eq!(
            Solution::find_all_recipes(recipes, ingredients, supplies),
            vec![String::from("bread"), String::from("sandwich")]
        );

        let recipes = vec![
            String::from("bread"),
            String::from("sandwich"),
            String::from("burger"),
        ];
        let ingredients = vec![
            vec![String::from("yeast"), String::from("flour")],
            vec![String::from("bread"), String::from("meat")],
            vec![
                String::from("sandwich"),
                String::from("meat"),
                String::from("bread"),
            ],
        ];
        let supplies = vec![
            String::from("yeast"),
            String::from("flour"),
            String::from("meat"),
        ];
        assert_eq!(
            Solution::find_all_recipes(recipes, ingredients, supplies),
            vec![
                String::from("bread"),
                String::from("sandwich"),
                String::from("burger")
            ]
        );
    }
}
