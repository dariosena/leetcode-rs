pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();

        for c in s.chars() {
            match c {
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                '{' => stack.push('}'),
                ')' | ']' | '}' => {
                    if stack.is_empty() {
                        return false;
                    }

                    if let Some(pair) = stack.pop() {
                        if pair != c {
                            return false;
                        }
                    }
                }
                _ => return false,
            }
        }

        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_case_1() {
        assert!(Solution::is_valid("()".to_string()));
    }

    #[test]
    fn test_case_2() {
        assert!(!Solution::is_valid("(]".to_string()));
    }

    #[test]
    fn test_case_3() {
        assert!(Solution::is_valid("()[]{}".to_string()));
    }

    #[test]
    fn test_case_4() {
        assert!(Solution::is_valid("{[]}".to_string()));
    }

    #[test]
    fn test_case_5() {
        assert!(Solution::is_valid("([])".to_string()));
    }

    #[test]
    fn test_case_6() {
        assert!(!Solution::is_valid("]".to_string()));
    }
}
