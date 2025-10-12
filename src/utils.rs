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
}
