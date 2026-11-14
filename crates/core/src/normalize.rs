use std::cmp::Ordering;

use uuid::Uuid;

pub fn compare_position_and_id(
    left_position: &str,
    left_id: Uuid,
    right_position: &str,
    right_id: Uuid,
) -> Ordering {
    left_position
        .cmp(right_position)
        .then_with(|| left_id.cmp(&right_id))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_and_id_ordering() {
        let low_id = Uuid::from_u128(1);
        let high_id = Uuid::from_u128(2);
        let cases = [
            ("a", high_id, "b", low_id, Ordering::Less),
            ("b", low_id, "a", high_id, Ordering::Greater),
            ("a", low_id, "a", high_id, Ordering::Less),
            ("a", high_id, "a", low_id, Ordering::Greater),
            ("a", low_id, "a", low_id, Ordering::Equal),
        ];

        for (left_position, left_id, right_position, right_id, expected) in cases {
            assert_eq!(
                compare_position_and_id(left_position, left_id, right_position, right_id),
                expected,
                "comparing ({left_position}, {left_id}) with ({right_position}, {right_id})"
            );
        }
    }
}
