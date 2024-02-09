/*
Example 2: 2351. First Letter to Appear Twice

Given a string s, return the first character to appear twice. It is guaranteed that the input will have a 
duplicate character.

/**
 * @param {string} s
 * @return {character}
 */
var repeatedCharacter = function(s) {
    let seen = new Set();
    for (const c of s) {
        if (seen.has(c)) {
            return c;
        }
        
        seen.add(c);
    }
    
    return " ";
};

/*
This improves our time complexity to O(n) as each for loop iteration now runs in constant time.

The space complexity is a more interesting topic of discussion. Many people will argue that the space 
complexity is O(1)O(1) because the input can only have characters from the English alphabet, which is bounded 
by a constant (26). This is very common with string problems and technically correct. In an interview setting, 
this is probably a safe answer, but you should also note that the space complexity could be O(m)O(m), where m
is the number of allowable characters in the input. This is a more general answer and also technically correct.

*/