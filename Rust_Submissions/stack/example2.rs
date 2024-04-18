/*
Example 2: 1047. Remove All Adjacent Duplicates In String

You are given a string s. Continuously remove duplicates (two of the same character beside each other) until you 
can't anymore. Return the final string after this.

For example, given s = "abbaca", you can first remove the "bb" to get "aaca". Next, you can remove the "aa" to 
get "ca". This is the final answer.
*/

fn remove_duplicates(s: &str) -> String {
    let mut stack: Vec<char> = Vec::new();

    for c in s.chars() {
        if !stack.is_empty() && *stack.last().unwrap() == c {
                stack.pop();
        } else {
                stack.push(c);
        } 
    }

    stack.iter().collect()
}

fn main() {
    let s = "abbaca";
    println!("After removing duplicates: {}", remove_duplicates(s));
}