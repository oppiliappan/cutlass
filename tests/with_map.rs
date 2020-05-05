#![feature(type_alias_impl_trait)]

pub mod helper_functions {
    use cutlass::curry;

    #[curry]
    pub fn product(x: u32, y: u32) -> u32 {
        return x * y;
    }
    #[curry]
    pub fn add(x: u32, y: u32) -> u32 {
        x + y
    }
}

#[cfg(test)]
mod tests {
    use super::helper_functions::{add, product};
    #[test]
    fn test_product() {
        assert_eq!(
            (1..=3).map(product(5)).collect::<Vec<u32>>(),
            vec![5, 10, 15]
        );
    }

    #[test]
    fn test_add() {
        let increment = add(1);
        let v: Vec<u32> = (1..=3).map(increment).collect();
        assert_eq!(v, vec![2, 3, 4]);
    }
}
