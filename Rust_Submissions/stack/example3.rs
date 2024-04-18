/*
Example 3: 844. Backspace String Compare

Given two strings s and t, return true if they are equal when both are typed into empty text editors. '#' means a 
backspace character.

For example, given s = "ab#c" and t = "ad#c", return true. Because of the backspace, the strings are both equal to "ac".
*/

fn back_space_compare(s: &str, t: &str) -> bool {
    fn build(s: &str) -> String {
        let mut stack: Vec<char> = Vec::new();
        
         for c in s.chars() {
            if !stack.is_empty() && c == '#' {
                    stack.pop();
            } else {
                    stack.push(c);
            } 
        }
        
         stack.iter().collect()
    }
    

   
    build(s) == build(t)
   
}

fn main() {
    let s = "ab#c";
    let t = "ad#c";
    println!("Backspace compare of {s} and {t}: {}", back_space_compare(s, t));
}

/*
this approach has a time and space complexity linear with the input sizes, because our stack implementations are 
efficient.
*/