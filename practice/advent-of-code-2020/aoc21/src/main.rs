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

    let (found_ingredients, _, _) = trim_match(&possible, index2.len());

    let result: usize = foods
        .iter()
        .map(|f| f.count_without_allergens(found_ingredients))
        .sum();

    writeln!(io::stdout(), "Part 1: {result}")?;
    writeln!(io::stdout(), "> Time elapsed is: {:?}", start.elapsed())?;
    Ok(result)
}

fn part2(foods: &[Food], index1: &Index, index2: &Index) -> Result<String> {
    let start = Instant::now();

    let possible = possible_match(foods, index1, index2);

    let (_, _, mut matched) = trim_match(&possible, index2.len());

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
                .filter(|f| f.allergens & h2 != 0)
                .all(|f| (f.ingredients.0 & h1.0 != 0) || (f.ingredients.1 & h1.1 != 0))
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
) -> ((u128, u128), u128, Vec<(usize, usize)>) {
    let mut found_ingredients = (0, 0);
    let mut found_allergens = 0;

    let mut matched = vec![];
    while hash_count_one(found_allergens) < allergen_count {
        for (&k, v) in possible.iter() {
            if k & found_allergens != 0 {
                continue;
            }
            let temp: Vec<_> = v
                .iter()
                .filter(|h| h.0 & found_ingredients.0 == 0 && h.1 & found_ingredients.1 == 0)
                .collect();
            if temp.len() == 1 {
                found_allergens |= k;
                found_ingredients.0 |= temp[0].0;
                found_ingredients.1 |= temp[0].1;
                let a_id = hash_to_ids(k)[0];
                let i_id = hash_to_ids(temp[0].0).get(0).unwrap_or(&0)
                    + hash_to_ids(temp[0].1).get(0).unwrap_or(&0);
                matched.push((a_id, i_id));
            }
        }
    }
    (found_ingredients, found_allergens, matched)
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

fn update_found(id1: (u128, u128), id2: u128, found1: &mut (u128, u128), found2: &mut u128) {
    println!("found {:?} <-> {}", id1, id2);

    found1.0 |= id1.0;
    found1.1 |= id1.1;
    *found2 |= id2;
}

#[derive(Clone)]
struct Food {
    ingredients: (u128, u128),
    allergens: u128,
}

impl Food {
    fn same_ingredients(&self, other: &Food) -> (u128, u128) {
        (
            self.ingredients.0 & other.ingredients.0,
            self.ingredients.1 & other.ingredients.1,
        )
    }

    fn same_allergens(&self, other: &Food) -> u128 {
        self.allergens & other.allergens
    }

    fn count_without_allergens(&self, found_ingredients: (u128, u128)) -> usize {
        let (mut h1, mut h2) = self.ingredients;
        h1 &= !found_ingredients.0;
        h2 &= !found_ingredients.1;
        hash_count_one(h1) + hash_count_one(h2)
    }

    fn ingredients_count(&self) -> usize {
        let (h1, h2) = self.ingredients;
        hash_count_one(h1) + hash_count_one(h2)
    }

    fn allergens_count(&self) -> usize {
        hash_count_one(self.allergens)
    }

    fn extract(&self, b: &Food, found1: &(u128, u128), found2: &u128) -> Food {
        Food {
            ingredients: self.same_ingredients(b),
            allergens: self.same_allergens(b),
        }
        .exclude(found1, found2)
    }

    fn get_ingredients_ids(&self) -> Vec<usize> {
        let (h1, h2) = self.ingredients;
        let mut ingredients = hash_to_ids(h1);
        ingredients.extend(hash_to_ids(h2));
        ingredients
    }

    fn get_allergens_ids(&self) -> Vec<usize> {
        hash_to_ids(self.allergens)
    }

    fn exclude(&self, found1: &(u128, u128), found2: &u128) -> Food {
        let (mut h1, mut h2) = self.ingredients;
        h1 &= !found1.0;
        h2 &= !found1.1;

        let mut h3 = self.allergens;
        h3 &= !found2;
        Food {
            ingredients: (h1, h2),
            allergens: h3,
        }
    }

    fn matched(&self) -> Option<((u128, u128), u128)> {
        let (h1, h2) = self.ingredients;
        let h3 = self.allergens;
        let count1 = self.ingredients_count();
        let count2 = self.allergens_count();

        if count2 == 1 && count1 == count2 {
            return Some(((h1, h2), h3));
        }
        None
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
