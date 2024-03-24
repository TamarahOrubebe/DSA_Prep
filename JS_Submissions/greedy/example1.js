/*
Example 1: 2126. Destroying Asteroids

You are given an integer array asteroids and an integer mass representing the mass of a planet. The planet will 
collide with the asteroids one by one - you can choose the order. If the mass of the planet is less than the 
mass of an asteroid, the planet is destroyed. Otherwise, the planet gains the mass of the asteroid. Can you 
destroy all the asteroids?
*/

/**
 * @param {number} mass
 * @param {number[]} asteroids
 * @return {boolean}
 */
var asteroidsDestroyed = function(mass, asteroids) {
    asteroids.sort((a, b) => a - b);
    for (const asteroid of asteroids) {
        if (asteroid > mass) {
            return false;
        }
        
        mass += asteroid;
    }
    
    return true;
};

/*
This algorithm has a time complexity of O(n \cdot \log{}n)O(n⋅logn) due to the sort, where nn is the number of 
asteroids. The amount of space used is dependent on the language you are using. For example, Python implements 
timsort which uses up to O(n)O(n) space, but a language that uses quicksort uses O(logn) space.


*/