use std::collections::HashMap;
use std::error::Error;
use std::io::{self, Read, Write};
use std::time::Instant;

#[allow(unused_macros)]
macro_rules! err {
    ($($tt:tt)*) => { Err(Box::<dyn Error>::from(format!($($tt)*))) }
}

type Result<T> = ::std::result::Result<T, Box<dyn Error>>;
type Id = usize;
type Index = HashMap<String, Id>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut index1 = Index::new();
    let mut index2 = Index::new();
    let foods: Vec<_> = input
        .lines()
        .map(|l| Food::from_input(l, &mut index1, &mut index2))
        .collect();

    part1(&foods, &index1, &index2)?;
    part2(&foods, &index1, &index2)?;
    Ok(())
}

fn part1(foods: &[Food], index1: &Index, index2: &Index) -> Result<usize> {
    let start = Instant::now();

    let possible = possible_match(foods, index1, index2);

    let (matched_food, _) = trim_match(&possible, index2.len());

    let result: usize = foods
        .iter()
        .map(|f| f.count_without_allergens(matched_food.ingredients))
        .sum();

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(foods: &[Food], index1: &Index, index2: &Index) -> Result<String> {
    let start = Instant::now();

    let possible = possible_match(foods, index1, index2);

    let (_, mut matched) = trim_match(&possible, index2.len());

    let index1: HashMap<_, _> = index1.iter().map(|(k, v)| (v, k)).collect();
    let index2: HashMap<_, _> = index2.iter().map(|(k, v)| (v, k)).collect();
    matched.sort_by_key(|(a_id, _)| index2.get(a_id).unwrap());
    let result = matched
        .iter()
        .map(|(_, id)| index1.get(id).unwrap().to_string())
        .collect::<Vec<_>>()
        .join(",");

    writeln!(io::stdout(), "Part 2: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn possible_match(
    foods: &[Food],
    index1: &Index,
    index2: &Index,
) -> HashMap<u128, Vec<(u128, u128)>> {
    let mut possible: HashMap<u128, Vec<(u128, u128)>> = HashMap::new();
    for &i_id in index1.values() {
        let h1 = ingredient_id_to_hash(i_id);
        for &a_id in index2.values() {
            let h2 = allergen_id_to_hash(a_id);
            if foods
                .iter()
                .filter(|f| f.contains_allergen(&h2))
                .all(|f| f.contains_ingredient(&h1))
            {
                possible.entry(h2).or_default().push(h1);
            }
        }
    }
    possible
}

fn trim_match(
    possible: &HashMap<u128, Vec<(u128, u128)>>,
    allergen_count: usize,
) -> (Food, Vec<(usize, usize)>) {
    let mut matched_food = Food {
        ingredients: (0, 0),
        allergens: 0,
    };

    let mut matched = vec![];
    while hash_count_one(matched_food.allergens) < allergen_count {
        for (&k, v) in possible.iter() {
            if matched_food.contains_allergen(&k) {
                continue;
            }
            let temp: Vec<_> = v
                .iter()
                .filter(|h| !matched_food.contains_ingredient(h))
                .collect();
            if temp.len() == 1 {
                matched_food.update(*temp[0], k);
                let a_id = hash_to_ids(k)[0];
                let i_id = hash_to_ids(temp[0].0).first().unwrap_or(&0)
                    + hash_to_ids(temp[0].1).first().unwrap_or(&0);
                matched.push((a_id, i_id));
            }
        }
    }
    (matched_food, matched)
}

fn ingredient_id_to_hash(id: usize) -> (u128, u128) {
    if id < 128 {
        (1 << id, 0)
    } else {
        (0, 1 << (id - 127))
    }
}

fn allergen_id_to_hash(id: usize) -> u128 {
    1u128 << id
}

fn hash_to_ids(num: u128) -> Vec<Id> {
    let mut r = vec![];
    for i in 0..128 {
        if (num >> i) & 1 == 1 {
            r.push(i);
        }
        if num == 0 {
            break;
        }
    }
    r
}

fn hash_count_one(num: u128) -> usize {
    let mut count = 0;
    for i in 0..128 {
        if (num >> i) & 1 == 1 {
            count += 1;
        }
        if num == 0 {
            break;
        }
    }
    count
}

#[derive(Clone)]
struct Food {
    ingredients: (u128, u128),
    allergens: u128,
}

impl Food {
    fn count_without_allergens(&self, found_ingredients: (u128, u128)) -> usize {
        let (mut h1, mut h2) = self.ingredients;
        h1 &= !found_ingredients.0;
        h2 &= !found_ingredients.1;
        hash_count_one(h1) + hash_count_one(h2)
    }

    fn contains_ingredient(&self, h: &(u128, u128)) -> bool {
        (self.ingredients.0 & h.0 != 0) || (self.ingredients.1 & h.1 != 0)
    }

    fn contains_allergen(&self, h: &u128) -> bool {
        self.allergens & h != 0
    }

    fn update(&mut self, id1: (u128, u128), id2: u128) {
        self.ingredients.0 |= id1.0;
        self.ingredients.1 |= id1.1;
        self.allergens |= id2;
    }

    fn from_input(line: &str, index1: &mut Index, index2: &mut Index) -> Self {
        fn get_id(name: &str, index: &mut Index) -> usize {
            if let Some(id) = index.get(name) {
                *id
            } else {
                let id = 1 + *index.values().max().unwrap_or(&0);
                index.insert(name.to_owned(), id);
                id
            }
        }

        let mut ingredients = vec![];
        let mut allergens = vec![];
        let mut part = 0;
        for word in line.trim().split(' ') {
            if word.trim().starts_with('(') {
                part = 1;
                continue;
            }
            if part == 0 {
                ingredients.push(get_id(word.trim(), index1))
            } else if part == 1 {
                let name = if let Some(name) = word.trim().strip_suffix([',', ')']) {
                    name.trim()
                } else {
                    word.trim()
                };
                allergens.push(get_id(name, index2))
            }
        }
        let mut r = (0, 0);
        for id in ingredients {
            let id = ingredient_id_to_hash(id);
            r.0 |= id.0;
            r.1 |= id.1;
        }
        let ingredients = r;

        let mut r = 0;
        for id in allergens {
            r |= allergen_id_to_hash(id);
        }
        let allergens = r;

        Self {
            ingredients,
            allergens,
        }
    }
}

#[test]
fn example_input() {
    let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
    trh fvjkl sbzzf mxmxvkd (contains dairy)
    sqjhc fvjkl (contains soy)
    sqjhc mxmxvkd sbzzf (contains fish)";

    let mut index1 = Index::new();
    let mut index2 = Index::new();
    let foods: Vec<_> = input
        .lines()
        .map(|l| Food::from_input(l, &mut index1, &mut index2))
        .collect();
    assert_eq!(part1(&foods, &index1, &index2).unwrap(), 5);
    assert_eq!(
        part2(&foods, &index1, &index2).unwrap(),
        "mxmxvkd,sqjhc,fvjkl"
    );
}
