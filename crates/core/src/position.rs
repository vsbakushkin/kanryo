use fractional_index::FractionalIndex;

use crate::error::CoreError;

pub fn first() -> String {
    FractionalIndex::default().to_string()
}

pub fn after(a: &str) -> Result<String, CoreError> {
    let index = FractionalIndex::from_string(a).map_err(|_| CoreError::InvalidPosition {
        value: a.to_string(),
    })?;
    Ok(FractionalIndex::new_after(&index).to_string())
}

pub fn before(b: &str) -> Result<String, CoreError> {
    let index = FractionalIndex::from_string(b).map_err(|_| CoreError::InvalidPosition {
        value: b.to_string(),
    })?;
    Ok(FractionalIndex::new_before(&index).to_string())
}

pub fn between(a: &str, b: &str) -> Result<String, CoreError> {
    let lower = FractionalIndex::from_string(a).map_err(|_| CoreError::InvalidPosition {
        value: a.to_string(),
    })?;
    let upper = FractionalIndex::from_string(b).map_err(|_| CoreError::InvalidPosition {
        value: b.to_string(),
    })?;
    let index = FractionalIndex::new_between(&lower, &upper).ok_or_else(|| {
        CoreError::InvalidPositionBounds {
            lower: a.to_string(),
            upper: b.to_string(),
        }
    })?;

    Ok(index.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positions_are_ordered_and_invalid_positions_are_rejected() {
        let middle = first();
        let left = before(&middle).expect("before should accept a valid position");
        let right = after(&middle).expect("after should accept a valid position");

        assert!(left < middle);
        assert!(middle < right);

        assert!(matches!(
            before("not-a-position"),
            Err(CoreError::InvalidPosition { .. })
        ));
        assert!(matches!(
            after("not-a-position"),
            Err(CoreError::InvalidPosition { .. })
        ));

        let a = first();
        let b = after(&a).expect("after should accept a valid position");
        let inserted = between(&a, &b).expect("between should accept ordered valid bounds");

        assert!(a < inserted);
        assert!(inserted < b);

        assert!(matches!(
            between(&a, &a),
            Err(CoreError::InvalidPositionBounds { .. })
        ));
        assert!(matches!(
            between(&b, &a),
            Err(CoreError::InvalidPositionBounds { .. })
        ));
        assert!(matches!(
            between("not-a-position", &b),
            Err(CoreError::InvalidPosition { .. })
        ));
        assert!(matches!(
            between(&a, "not-a-position"),
            Err(CoreError::InvalidPosition { .. })
        ));
    }

    #[test]
    fn repeated_insertions_preserve_strict_order() {
        let a = first();
        let b = after(&a).expect("after should accept a valid position");
        let mut positions: Vec<String> = vec![a.clone(), b.clone()];

        for _ in 0..1000 {
            let inserted = between(&positions[0], &positions[1])
                .expect("between should accept ordered valid bounds");
            assert!(positions[0] < inserted);
            assert!(inserted < positions[1]);
            positions.insert(1, inserted);
        }

        assert_eq!(positions.len(), 1002);
        assert_eq!(positions.first(), Some(&a));
        assert_eq!(positions.last(), Some(&b));
        assert!(positions.windows(2).all(|pair| pair[0] < pair[1]));
    }
}
