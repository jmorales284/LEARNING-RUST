#[cfg(test)]
mod tests {
    #[test]
    fn test_primitive_types4() {
        let a = [1, 2, 3, 4];
    
        let nice_slice = &a[1..4];

        print!("nice_slice: {:?}\n", nice_slice);
    
        assert_eq!([2, 3, 4], nice_slice)
    }
}