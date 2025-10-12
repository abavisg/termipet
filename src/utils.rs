use rand::Rng;

/// Caps a stat value between min and max bounds
/// Useful for ensuring pet stats stay within valid ranges (0-100)
pub fn cap_stat(value: i32, min: u8, max: u8) -> u8 {
    if value < min as i32 {
        min
    } else if value > max as i32 {
        max
    } else {
        value as u8
    }
}

/// Returns true with the given probability (0.0 to 1.0)
/// Used for probabilistic pet behaviors like potty relief during walks
pub fn random_bool(probability: f32) -> bool {
    let mut rng = rand::thread_rng();
    rng.r#gen::<f32>() < probability
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cap_stat_within_range() {
        // Given: a value within range
        let result = cap_stat(50, 0, 100);

        // Then: value should remain unchanged
        assert_eq!(result, 50);
    }

    #[test]
    fn test_cap_stat_above_max() {
        // Given: a value above max
        let result = cap_stat(120, 0, 100);

        // Then: should be capped at max
        assert_eq!(result, 100);
    }

    #[test]
    fn test_cap_stat_below_min() {
        // Given: a value below min
        let result = cap_stat(-10, 0, 100);

        // Then: should be capped at min
        assert_eq!(result, 0);
    }

    #[test]
    fn test_cap_stat_at_boundaries() {
        // Test exact boundary values
        assert_eq!(cap_stat(0, 0, 100), 0);
        assert_eq!(cap_stat(100, 0, 100), 100);
    }

    #[test]
    fn test_cap_stat_just_outside_boundaries() {
        // Test values just outside boundaries
        assert_eq!(cap_stat(-1, 0, 100), 0);
        assert_eq!(cap_stat(101, 0, 100), 100);
    }

    #[test]
    fn test_random_bool_probability_distribution() {
        // Given: a probability of 0.8 (80%)
        let probability = 0.8;
        let iterations = 1000;
        let mut true_count = 0;

        // When: calling random_bool many times
        for _ in 0..iterations {
            if random_bool(probability) {
                true_count += 1;
            }
        }

        // Then: approximately 80% should be true (within reasonable tolerance)
        let actual_probability = true_count as f32 / iterations as f32;
        // Allow 10% tolerance (0.7 to 0.9 range for 0.8 expected)
        assert!(
            actual_probability >= 0.7 && actual_probability <= 0.9,
            "Expected ~0.8, got {}",
            actual_probability
        );
    }

    #[test]
    fn test_random_bool_always_true() {
        // Given: probability of 1.0
        // Then: should always return true
        for _ in 0..100 {
            assert!(random_bool(1.0));
        }
    }

    #[test]
    fn test_random_bool_never_true() {
        // Given: probability of 0.0
        // Then: should always return false
        for _ in 0..100 {
            assert!(!random_bool(0.0));
        }
    }
}
