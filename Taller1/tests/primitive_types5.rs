#[cfg(test)]
mod tests {
    #[test]
    fn test_primitive_types5() {
        let cat = ("Furry McFurson", 3.5);
        let name = cat.0;
        let age = cat.1;
    
        println!("{} is {} years old.", name, age);
    }
}