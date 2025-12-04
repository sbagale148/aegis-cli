use std::collections::HashMap;

/// Calculate Shannon entropy of a string
/// Higher entropy indicates more randomness (typical of secrets/keys)
pub fn calculate_shannon_entropy(data: &str) -> f64 {
    if data.is_empty() {
        return 0.0;
    }

    let mut frequency: HashMap<char, usize> = HashMap::new();
    let length = data.len() as f64;

    // Count character frequencies
    for ch in data.chars() {
        *frequency.entry(ch).or_insert(0) += 1;
    }

    // Calculate Shannon entropy: -Σ(p(x) * log2(p(x)))
    let entropy = frequency
        .values()
        .map(|&count| {
            let probability = count as f64 / length;
            if probability > 0.0 {
                probability * probability.log2()
            } else {
                0.0
            }
        })
        .sum::<f64>();

    -entropy
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_high_entropy_string() {
        // Random-looking string should have high entropy
        let random = "aB3xY9mK2pL8nQ4wR7tU6vI5oP1jH0gF";
        let entropy = calculate_shannon_entropy(random);
        assert!(entropy > 4.0, "Expected high entropy, got {}", entropy);
    }

    #[test]
    fn test_low_entropy_string() {
        // Repetitive string should have low entropy
        let repetitive = "aaaaaaaaaaaaaaa";
        let entropy = calculate_shannon_entropy(repetitive);
        assert!(entropy < 1.0, "Expected low entropy, got {}", entropy);
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(calculate_shannon_entropy(""), 0.0);
    }
}

