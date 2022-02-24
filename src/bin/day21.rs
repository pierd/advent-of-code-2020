use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Ingredient<'a>(&'a str);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Allergen<'a>(&'a str);

#[derive(Clone, Debug, PartialEq)]
struct Food<'a> {
    ingredients: HashSet<Ingredient<'a>>,
    allergens: HashSet<Allergen<'a>>,
}

impl<'a> Food<'a> {
    fn from_str(s: &'a str) -> Result<Self, ()> {
        let mut main_parts = s.split(" (contains ");
        let raw_ingredients = main_parts.next().ok_or(())?;
        let raw_allergens = main_parts.next().ok_or(())?.strip_suffix(')').ok_or(())?;
        if main_parts.next().is_some() {
            return Err(());
        }
        Ok(Self {
            ingredients: raw_ingredients
                .split_ascii_whitespace()
                .map(Ingredient)
                .collect(),
            allergens: raw_allergens.split(", ").map(Allergen).collect(),
        })
    }
}

fn parse_input<'a>(input: &'a str) -> Result<Vec<Food<'a>>, ()> {
    input.lines().map(Food::from_str).collect()
}

#[derive(Debug, Default)]
struct ResolvedAllergens<'a> {
    ingredient_to_allergen: HashMap<Ingredient<'a>, Allergen<'a>>,
}

impl<'a> ResolvedAllergens<'a> {
    fn count_ingredients_without_allergens(&self, foods: &[Food]) -> usize {
        foods
            .iter()
            .flat_map(|food| food.ingredients.iter())
            .filter(|ingredient| !self.ingredient_to_allergen.contains_key(*ingredient))
            .count()
    }

    fn canonical_dangerous_ingredient_list(&self) -> String {
        let mut resolved: Vec<_> = self
            .ingredient_to_allergen
            .iter()
            .map(|(ingredient, allergen)| (allergen, ingredient))
            .collect();
        resolved.sort_unstable_by_key(|(allergen, _)| allergen.0);
        resolved
            .into_iter()
            .map(|(_, ingredient)| ingredient.0)
            .collect::<Vec<_>>()
            .join(",")
    }
}

fn resolve<'a>(foods: &[Food<'a>]) -> ResolvedAllergens<'a> {
    // TODO: learn: how to map deref on iterator
    let all_allergens: HashSet<Allergen<'_>> = foods
        .iter()
        .flat_map(|food| food.allergens.iter())
        .map(|a| *a)
        .collect();

    let mut allergen_to_possible_ingredients = HashMap::new();
    for allergen in &all_allergens {
        let mut ingredients: Option<HashSet<Ingredient<'_>>> = None;
        for food in foods {
            if food.allergens.contains(allergen) {
                ingredients = if let Some(ingr) = ingredients {
                    Some(ingr.intersection(&food.ingredients).map(|a| *a).collect())
                } else {
                    Some(food.ingredients.clone())
                }
            }
        }
        allergen_to_possible_ingredients.insert(
            *allergen,
            ingredients.expect("allergen should have possible ingredients"),
        );
    }

    let mut ingredient_to_allergen = HashMap::new();
    while all_allergens.len() != ingredient_to_allergen.len() {
        if let Some((allergen, possibilities)) = allergen_to_possible_ingredients
            .iter()
            .find(|(_, possibilities)| possibilities.len() == 1)
        {
            let allergen = *allergen;
            let ingredient = *possibilities.iter().next().unwrap();
            ingredient_to_allergen.insert(ingredient, allergen);
            for (_, possibilities) in allergen_to_possible_ingredients.iter_mut() {
                possibilities.remove(&ingredient);
            }
        } else {
            panic!("can't find allergen to resolve");
        }
    }
    ResolvedAllergens {
        ingredient_to_allergen,
    }
}

fn main() {
    let foods = parse_input(include_str!("../../inputs/day21.txt"))
        .expect("input should be parsed correctly");
    let resolved = resolve(&foods);
    println!(
        "Part 1: {}",
        resolved.count_ingredients_without_allergens(&foods)
    );
    println!("Part 2: {}", resolved.canonical_dangerous_ingredient_list());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(
            Food::from_str("foo bar (contains baz, xyz)"),
            Ok(Food {
                ingredients: ["foo", "bar"].into_iter().map(Ingredient).collect(),
                allergens: ["baz", "xyz"].into_iter().map(Allergen).collect(),
            })
        );
    }

    const SAMPLE_INPUT: &str = concat!(
        "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)\n",
        "trh fvjkl sbzzf mxmxvkd (contains dairy)\n",
        "sqjhc fvjkl (contains soy)\n",
        "sqjhc mxmxvkd sbzzf (contains fish)",
    );

    #[test]
    fn test_part1_sample() {
        let foods = parse_input(SAMPLE_INPUT).expect("sample should parse");
        let resolved = resolve(&foods);
        dbg!(&resolved.ingredient_to_allergen);
        assert_eq!(resolved.count_ingredients_without_allergens(&foods), 5);
    }

    #[test]
    fn test_part2_sample() {
        let foods = parse_input(SAMPLE_INPUT).expect("sample should parse");
        let resolved = resolve(&foods);
        dbg!(&resolved.ingredient_to_allergen);
        assert_eq!(
            resolved.canonical_dangerous_ingredient_list(),
            "mxmxvkd,sqjhc,fvjkl"
        );
    }
}
