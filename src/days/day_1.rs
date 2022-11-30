use crate::utils::utils::hello_world;

pub fn solve() {
    hello_world();
}

#[allow(dead_code)]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
}
