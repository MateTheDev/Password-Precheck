mod compare_pass;
use compare_pass::{hash, comparison};

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test1() {
        let result = hash("hello");
        assert_eq!(result, "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824");
    }

    #[test]
    fn test2_true() {
        let result = comparison(&hash("hello"), &hash("hello"));
        assert_eq!(result, true);
    }
    #[test]

    fn test2_false() {
        let result = comparison(&hash("hello"), &hash("helo"));
        assert_eq!(result, false);
    }
}