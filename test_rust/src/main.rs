use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let x: f64 = rng.gen();
    println!("Hello, world!: {}", x);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_unit() {
        assert_eq!(2 + 2, 4);
    }
}
