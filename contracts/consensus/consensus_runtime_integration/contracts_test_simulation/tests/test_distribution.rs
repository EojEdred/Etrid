#[cfg(test)]
mod tests {
    use super::*;
    use consensus::distribution::{calculate_distribution_reward, DistributionInput};

    #[test]
    fn test_distribution_calculation() {
        let input = DistributionInput {
            stake: 100,
            coinage: 10,
            base_reward: 5.0,
        };
        let expected = 50.0;
        let result = calculate_distribution_reward(input);
        assert_eq!(result, expected);
    }
}