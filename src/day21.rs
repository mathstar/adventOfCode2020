use crate::day::Day;
use std::collections::{HashMap, HashSet};

pub struct Day21 {}

#[derive(Debug)]
struct Recipe {
    ingredients: Vec<String>,
    known_allergens: Vec<String>
}

fn parse_input(input: &str) -> Vec<Recipe> {
    let mut recipes = Vec::new();

    for line in input.lines() {
        let mut split = line.split(" (contains ");

        let mut ingredients = Vec::new();
        for ingredient in split.next().unwrap().split(" ") {
            ingredients.push(ingredient.to_owned());
        }

        let mut known_allergens = Vec::new();
        for allergen in split.next().unwrap().strip_suffix(")").unwrap().split(", ") {
            known_allergens.push(allergen.to_owned());
        }

        recipes.push(Recipe {ingredients, known_allergens});
    }

    recipes
}

impl Day for Day21 {
    fn part1(&self, input: &str) -> String {
        let recipes = parse_input(input);

        let mut possible_allergen_translations:HashMap<String, HashSet<String>> = HashMap::new();
        for recipe in recipes.iter() {
            for allergen in recipe.known_allergens.iter() {
                let mut new_translations = HashSet::new();
                for ingredient in recipe.ingredients.iter() {
                    new_translations.insert(ingredient.to_owned());
                }
                match possible_allergen_translations.get_mut(allergen) {
                    Some(translations) => {
                        *translations = translations.intersection(&new_translations).map(|s| s.to_owned()).collect();
                    },
                    None => {
                        possible_allergen_translations.insert(allergen.to_owned(), new_translations);
                    }
                }
            }
        }

        let possible_allergens = possible_allergen_translations.values().flat_map(|s| s.iter().map(|s| s.to_owned())).collect::<HashSet<String>>();
        let mut non_allergen_appearances = 0;
        for recipe in recipes.iter() {
            for ingredient in recipe.ingredients.iter() {
                if !possible_allergens.contains(ingredient) {
                    non_allergen_appearances += 1;
                }
            }
        }

        non_allergen_appearances.to_string()
    }

    fn part2(&self, input: &str) -> String {
        let recipes = parse_input(input);

        let mut possible_allergen_translations:HashMap<String, HashSet<String>> = HashMap::new();
        for recipe in recipes.iter() {
            for allergen in recipe.known_allergens.iter() {
                let mut new_translations = HashSet::new();
                for ingredient in recipe.ingredients.iter() {
                    new_translations.insert(ingredient.to_owned());
                }
                match possible_allergen_translations.get_mut(allergen) {
                    Some(translations) => {
                        *translations = translations.intersection(&new_translations).map(|s| s.to_owned()).collect();
                    },
                    None => {
                        possible_allergen_translations.insert(allergen.to_owned(), new_translations);
                    }
                }
            }
        }

        let mut fixed_translations = HashMap::new();
        while !possible_allergen_translations.is_empty() {
            let fixed_translation;
            let owned_allergen;
            {
                let (allergen, translations) = possible_allergen_translations.iter()
                    .filter(|(allergen, translations)| !fixed_translations.contains_key(*allergen) && translations.len() == 1)
                    .next()
                    .unwrap();
                fixed_translation = translations.iter().next().unwrap().to_owned();
                owned_allergen = allergen.to_owned();
            }
            possible_allergen_translations.remove(&owned_allergen);
            fixed_translations.insert(owned_allergen, fixed_translation.to_owned());

            for translations in possible_allergen_translations.values_mut() {
                translations.remove(&fixed_translation);
            }
        }

        let mut translations = fixed_translations.iter().collect::<Vec<(&String, &String)>>();
        translations.sort_by_key(|(k,_)| k.to_string());

        translations.iter().map(|(_,v)| v.to_string() + ",").collect::<String>().strip_suffix(",").unwrap().to_owned()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        assert_eq!(Day21{}.part1("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)"), "5");
    }

    #[test]
    fn part2_test1() {
        assert_eq!(Day21{}.part2("mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)"), "mxmxvkd,sqjhc,fvjkl");
    }
}
