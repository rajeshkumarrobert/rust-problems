use std::collections::HashSet;
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut given_word = word.to_lowercase().chars().collect::<Vec<char>>();
    given_word.sort();
    let mut res = HashSet::new();
    for slice in possible_anagrams{
        if word.to_lowercase() != slice.to_lowercase(){
            let mut new_word = slice.to_lowercase().chars().collect::<Vec<char>>();
            new_word.sort();
            if given_word.eq(&new_word){
                res.insert(*slice);
            }
        }
    }
    res
}

#[derive(Debug)]
pub struct Duration{
    pub age: f64
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        let earth_year_in_seconds = 31557600;
        let mut age = s as f64/earth_year_in_seconds as f64;
        age = (age * 100.0).round()/100.0;
        Duration { age: age }
    }
}

pub trait Planet {
    const ORBITAL_PERIOD: f64;
    fn years_during(d: &Duration) -> f64{
        d.age / Self::ORBITAL_PERIOD
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    const ORBITAL_PERIOD:f64 = 0.2408467;
}
impl Planet for Venus {
    const ORBITAL_PERIOD:f64 = 0.61519726;
}
impl Planet for Earth {
    const ORBITAL_PERIOD:f64 = 1.0;
}
impl Planet for Mars {
    const ORBITAL_PERIOD:f64 = 1.8808158;
}
impl Planet for Jupiter {
    const ORBITAL_PERIOD:f64 = 11.862615;
}
impl Planet for Saturn {
    const ORBITAL_PERIOD:f64 = 29.447498;
}
impl Planet for Uranus {
    const ORBITAL_PERIOD:f64 = 84.016846;
}
impl Planet for Neptune {
    const ORBITAL_PERIOD:f64 = 164.79132;
}

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    let res = match (first_list.len(),second_list.len()){
        (0,0) => Comparison::Equal,
        (0,_) => Comparison::Sublist,
        (_,0) => Comparison::Superlist,
        _ => check_comparison(first_list, second_list)
    };
    
    res
}

pub fn check_comparison(first_list: &[i32], second_list: &[i32]) -> Comparison{
    if first_list == second_list{
            return Comparison::Equal;
        }else if first_list.windows(second_list.len()).any(|arr| arr==second_list) {
            return Comparison::Superlist;
        }else if second_list.windows(first_list.len()).any(|arr| arr == first_list) {
            return Comparison::Sublist;
        }else {
            return Comparison::Unequal;
        }
}