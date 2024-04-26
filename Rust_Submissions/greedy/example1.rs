/*
Example 1: 2126. Destroying Asteroids

You are given an integer array asteroids and an integer mass representing the mass of a planet. The planet will 
collide with the asteroids one by one - you can choose the order. If the mass of the planet is less than the 
mass of an asteroid, the planet is destroyed. Otherwise, the planet gains the mass of the asteroid. Can you 
destroy all the asteroids?
*/

fn asteroids_destroyed(mut mass: i32, mut asteroids: Vec<i32>) -> bool {
    asteroids.sort();
    for asteroid in asteroids {
        if asteroid > mass {
            return false;
        }
        mass += asteroid;
    }
    true
}
fn main() {
    let elements = vec![1, 5, 3, 2, 4];
    println!("{:#?}", asteroids_destroyed(2, elements));
}

/*
 This algorithm has a time complexity of O(n⋅logn) due to the sort, where nn is the number of asteroids. The 
 amount of space used is dependent on the language you are using. For example, Python implements timsort which 
 uses up to O(n) space, but a language that uses quicksort uses O(logn) space.
*/