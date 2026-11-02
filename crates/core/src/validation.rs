use crate::error::CoreError;

pub fn validate_task_title(title: &str) -> Result<(), CoreError> {
    let length = title.chars().count();
    if !(1..=500).contains(&length) {
        return Err(CoreError::InvalidTaskTitleLength { length });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_titles_with_one_or_500_characters() {
        for character in ['a', 'я'] {
            for length in [1, 500] {
                let title = character.to_string().repeat(length);
                assert!(
                    validate_task_title(&title).is_ok(),
                    "expected a valid title with {length} copies of {character:?}"
                );
            }
        }
    }

    #[test]
    fn rejects_titles_with_zero_or_501_characters() {
        for character in ['a', 'я'] {
            for length in [0, 501] {
                let title = character.to_string().repeat(length);
                assert!(
                    matches!(
                        validate_task_title(&title),
                        Err(CoreError::InvalidTaskTitleLength { length: actual })
                            if actual == length
                    ),
                    "expected an invalid title length of {length} for {character:?}"
                );
            }
        }
    }
}
