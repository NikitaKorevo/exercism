use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Allergies(u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Self::get_allergens(&Self(score));
        Self(score)
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        *self.get_allergens().get(allergen).unwrap()
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut result = vec![];

        for (allergen, state) in self.get_allergens() {
            if state {
                result.push(allergen);
            }
        }

        result
    }

    fn get_allergens(&self) -> BTreeMap<Allergen, bool> {
        let mut current_score: u32 = self.0 % 256;
        let mut list: BTreeMap<Allergen, bool> = BTreeMap::new();
        let mut allergens = [
            Allergen::Eggs,
            Allergen::Peanuts,
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes,
            Allergen::Chocolate,
            Allergen::Pollen,
            Allergen::Cats,
        ];

        allergens.sort();
        allergens.reverse();

        for name in allergens {
            if current_score >= name as u32 {
                current_score -= name as u32;
                list.insert(name, true);
            } else {
                list.insert(name, false);
            }
        }

        list
    }
}
