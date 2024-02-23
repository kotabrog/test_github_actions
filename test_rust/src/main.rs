use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let x: f64 = rng.gen();
    let y = 3.14;
    println!("Hello, world!! random num3: {}", x * y);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_unit() {
        assert_eq!(2 + 2, 4);
    }
}
