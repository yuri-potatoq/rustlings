// hashmap1.rs
// A basket of fruits in the form of a hash map needs to be defined.
// The key represents the name of the fruit and the value represents
// how many of that particular fruit is in the basket. You have to put
// at least three different types of fruits (e.g apple, banana, mango)
// in the basket and the total count of all the fruits should be at
// least five.
//
// Make me compile and pass the tests!
//
// Execute the command `rustlings hint hashmap1` if you need
// hints.

// I AM DONE

use std::collections::HashMap;


struct FruitBasket <K, V> {
    fruits: HashMap<K, V>,
}

impl <K, V> FruitBasket <K, V> 
    where K: Eq + std::hash::Hash
{
    fn create() -> Self {
        Self {
            fruits: HashMap::<K, V>::new()
        }
    }

    fn populate(&mut self, name: K, amount: V) -> () {
        self.fruits.insert(name, amount);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let mut fruits_basket: FruitBasket::<String, u32> = 
            FruitBasket::create();
        
        let fruits = [
            ("banana".to_owned(), 2),
            ("apple".to_owned(), 3),
            ("avocado".to_owned(), 1),
        ];

        for (name, amount) in fruits.iter() {
            fruits_basket.populate(name.clone(), *amount);
        }

        assert!(fruits_basket.fruits.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let mut fruits_basket = 
            FruitBasket::<String, u32>::create();

        for i in 0..4 {
            fruits_basket.populate(format!("banana{}", i), 2);
        }

        println!("counter {}", fruits_basket.fruits.len());
        assert!(fruits_basket.fruits.values().sum::<u32>() >= 5);
    }
}
