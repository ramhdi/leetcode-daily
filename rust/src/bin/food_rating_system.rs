// https://leetcode.com/problems/design-a-food-rating-system/
// 2023/12/17

use std::collections::{BTreeSet, HashMap};

struct FoodRatings {
    food_rating_map: HashMap<String, i32>,
    food_cuisine_map: HashMap<String, String>,
    cuisine_food_map: HashMap<String, BTreeSet<(i32, String)>>,
}

impl FoodRatings {
    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let mut food_rating_map = HashMap::new();
        let mut food_cuisine_map = HashMap::new();
        let mut cuisine_food_map = HashMap::new();

        let zipped: Vec<(String, String, i32)> = foods
            .iter()
            .zip(cuisines.iter())
            .zip(ratings.iter())
            .map(|((a, b), &c)| (a.clone(), b.clone(), c))
            .collect();

        for (food, cuisine, rating) in zipped {
            food_rating_map.insert(food.clone(), rating);
            food_cuisine_map.insert(food.clone(), cuisine.clone());

            let entry = cuisine_food_map
                .entry(cuisine)
                .or_insert_with(BTreeSet::new);
            entry.insert((-rating, food));
        }

        return FoodRatings {
            food_rating_map,
            food_cuisine_map,
            cuisine_food_map,
        };
    }

    fn change_rating(&mut self, food: String, new_rating: i32) {
        if let Some(cuisine_name) = self.food_cuisine_map.get(&food) {
            if let Some(set) = self.cuisine_food_map.get_mut(cuisine_name) {
                if let Some(_) = set.take(&(-self.food_rating_map[&food], food.clone())) {
                    self.food_rating_map.insert(food.clone(), new_rating);
                    set.insert((-new_rating, food));
                }
            }
        }
    }

    fn highest_rated(&self, cuisine: String) -> String {
        return self
            .cuisine_food_map
            .get(&cuisine)
            .and_then(|set| set.iter().next().map(|&(_, ref food)| food.clone()))
            .unwrap();
    }
}

fn main() {
    let mut food_ratings = FoodRatings::new(
        ["kimchi", "miso", "sushi", "moussaka", "ramen", "bulgogi"]
            .iter()
            .map(|&s| s.to_string())
            .collect(),
        [
            "korean", "japanese", "japanese", "greek", "japanese", "korean",
        ]
        .iter()
        .map(|&s| s.to_string())
        .collect(),
        vec![9, 12, 8, 15, 14, 7],
    );
    println!("{}", food_ratings.highest_rated("korean".to_string()));
    println!("{}", food_ratings.highest_rated("japanese".to_string()));
    food_ratings.change_rating("sushi".to_string(), 16);
    println!("{}", food_ratings.highest_rated("japanese".to_string()));
    food_ratings.change_rating("ramen".to_string(), 16);
    println!("{}", food_ratings.highest_rated("japanese".to_string()));
}
