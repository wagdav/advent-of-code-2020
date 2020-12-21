use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::read_to_string;

#[derive(Clone, Debug, PartialEq)]
struct Food {
    ingredients: HashSet<String>,
    allergens: Vec<String>,
}

impl Food {
    fn new(line: &str) -> Option<Self> {
        let mut parts = line.trim().split(" (contains ");
        let ingredients = parts
            .next()?
            .split(' ')
            .map(|i| i.trim().to_string())
            .collect();
        let allergens = parts
            .next()?
            .trim_end_matches(')')
            .split(',')
            .map(|i| i.trim().to_string())
            .collect();

        Some(Self {
            ingredients,
            allergens,
        })
    }
}

fn parse(file: &str) -> Vec<Food> {
    file.lines().filter_map(|l| Food::new(l)).collect()
}

fn solve(food: &[Food]) -> (usize, String) {
    let mut ingredients_per_allergen: HashMap<String, HashSet<String>> = HashMap::new();

    for f in food {
        for a in &f.allergens {
            let mut x: HashSet<String> = ingredients_per_allergen
                .get(a)
                .unwrap_or(&f.ingredients)
                .to_owned();

            x = x.intersection(&f.ingredients).cloned().collect();

            ingredients_per_allergen.insert(a.to_string(), x);
        }
    }

    let inert_ingredients: HashSet<String> = ingredients_per_allergen
        .values()
        .flat_map(|x| x.iter().map(|v| v.to_string()))
        .collect();

    let num_inert = food
        .iter()
        .map(|f| {
            let food_ingredients: HashSet<String> = f.ingredients.iter().cloned().collect();
            food_ingredients.difference(&inert_ingredients).count()
        })
        .sum();

    let mut out: Vec<(String, String)> = vec![];
    let keys: Vec<String> = ingredients_per_allergen.keys().cloned().collect();

    while ingredients_per_allergen.values().any(|x| x.len() > 1) {
        for i in &keys {
            if ingredients_per_allergen[i].len() == 1 && out.iter().all(|(a, _)| a != i) {
                for v in &ingredients_per_allergen[i] {
                    out.push((i.to_string(), v.to_string())); // allergen, ingredient
                }

                for j in &keys {
                    if i != j {
                        ingredients_per_allergen
                            .get_mut(j)
                            .unwrap()
                            .remove(&out.iter().last().unwrap().1);
                    }
                }
            }
        }
    }

    out.sort_by(|(a1, _), (a2, _)| a1.cmp(a2));

    (num_inert, out.iter().map(|(_, i)| i).join(","))
}

fn solve_part1(food: &[Food]) -> usize {
    solve(&food).0
}

fn solve_part2(food: &[Food]) -> String {
    solve(&food).1
}

fn main() -> std::result::Result<(), Box<dyn Error>> {
    let file = read_to_string("inputs/day21.txt")?;

    let input = parse(&file);

    println!("Part 1 {:?}", solve_part1(&input));
    println!("Part 2 {:?}", solve_part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "\
        mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
        trh fvjkl sbzzf mxmxvkd (contains dairy)
        sqjhc fvjkl (contains soy)
        sqjhc mxmxvkd sbzzf (contains fish)\
    ";

    #[test]
    fn solves_part1() {
        let food = parse(&EXAMPLE);
        assert_eq!(solve_part1(&food), 5);
    }

    #[test]
    fn solves_part2() {
        let food = parse(&EXAMPLE);
        assert_eq!(solve_part2(&food), "mxmxvkd,sqjhc,fvjkl");
    }
}
