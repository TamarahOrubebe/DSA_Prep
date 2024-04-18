/*
Example 1: 20. Valid Parentheses

Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string 
is valid. The string is valid if all open brackets are closed by the same type of closing bracket in the correct 
order, and each closing bracket closes exactly one open bracket.

For example, s = "({})" and s = "(){}[]" are valid, but s = "(]" and s = "({)}" are not valid.
*/

fn is_valid(s: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    let matching: std::collections::HashMap<char, char> = [
        ('(', ')'),
        ('[', ']'),
        ('{', '}')
    ].iter().cloned().collect();
    
    for c in s.chars() {
        if matching.contains_key(&c) {
            stack.push(c);
        } else {
            if stack.is_empty() {
                return false;
            } else {
                let recent_opening = stack.pop().unwrap();
                if let Some(closing) = matching.get(&recent_opening) {
                    if *closing != c {
                        return false;
                    }
                }
            }
        }
    }

    stack.is_empty()
}

fn main() {
    let s = "{[()]}";
    println!("Is '{}' valid? {}", s, is_valid(s));

    let s = "{[(])}";
    println!("Is '{}' valid? {}", s, is_valid(s));

    let s = "{{{{";
    println!("Is '{}' valid? {}", s, is_valid(s));
}

/*
Because the stack's push and pop operations are O(1), this gives us a time complexity of O(n), where n is the 
size of the input array. This is because each element can only be pushed or popped once. The space complexity is 
also O(n) because the stack's size can grow linearly with the input size.

The key to recognizing the stack solution for this problem is seeing that the problem follows a LIFO nature, 
where the last (most recent) opening bracket is the first to be closed.
*/