use crate::change::{Change, FieldChange};
use crate::error::CoreError;
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Record {
    state: Change,
}

pub fn merge(current: Option<&Record>, change: &Change) -> Result<Record, CoreError> {
    match current {
        None => Ok(Record {
            state: change.clone(),
        }),
        Some(record) => match (&record.state, change) {
            (
                Change::Task {
                    id: current_id,
                    fields: current_fields,
                },
                Change::Task {
                    id: incoming_id,
                    fields: incoming_fields,
                },
            ) => {
                if current_id != incoming_id {
                    return Err(CoreError::MergeIdMismatch);
                }

                let merged_fields = merge_fields(current_fields, incoming_fields);

                Ok(Record {
                    state: Change::Task {
                        id: current_id.clone(),
                        fields: merged_fields,
                    },
                })
            }
            (
                Change::Project {
                    id: current_id,
                    fields: current_fields,
                },
                Change::Project {
                    id: incoming_id,
                    fields: incoming_fields,
                },
            ) => {
                if current_id != incoming_id {
                    return Err(CoreError::MergeIdMismatch);
                }

                let merged_fields = merge_fields(current_fields, incoming_fields);

                Ok(Record {
                    state: Change::Project {
                        id: current_id.clone(),
                        fields: merged_fields,
                    },
                })
            }
            (
                Change::Milestone {
                    id: current_id,
                    fields: current_fields,
                },
                Change::Milestone {
                    id: incoming_id,
                    fields: incoming_fields,
                },
            ) => {
                if current_id != incoming_id {
                    return Err(CoreError::MergeIdMismatch);
                }

                let merged_fields = merge_fields(current_fields, incoming_fields);

                Ok(Record {
                    state: Change::Milestone {
                        id: current_id.clone(),
                        fields: merged_fields,
                    },
                })
            }
            (
                Change::JournalEntry {
                    id: current_id,
                    fields: current_fields,
                },
                Change::JournalEntry {
                    id: incoming_id,
                    fields: incoming_fields,
                },
            ) => {
                if current_id != incoming_id {
                    return Err(CoreError::MergeIdMismatch);
                }

                let merged_fields = merge_fields(current_fields, incoming_fields);

                Ok(Record {
                    state: Change::JournalEntry {
                        id: current_id.clone(),
                        fields: merged_fields,
                    },
                })
            }
            (
                Change::Column {
                    id: current_id,
                    fields: current_fields,
                },
                Change::Column {
                    id: incoming_id,
                    fields: incoming_fields,
                },
            ) => {
                if current_id != incoming_id {
                    return Err(CoreError::MergeIdMismatch);
                }

                let merged_fields = merge_fields(current_fields, incoming_fields);

                Ok(Record {
                    state: Change::Column {
                        id: current_id.clone(),
                        fields: merged_fields,
                    },
                })
            }
            (
                Change::ChecklistItem {
                    id: current_id,
                    fields: current_fields,
                },
                Change::ChecklistItem {
                    id: incoming_id,
                    fields: incoming_fields,
                },
            ) => {
                if current_id != incoming_id {
                    return Err(CoreError::MergeIdMismatch);
                }

                let merged_fields = merge_fields(current_fields, incoming_fields);

                Ok(Record {
                    state: Change::ChecklistItem {
                        id: current_id.clone(),
                        fields: merged_fields,
                    },
                })
            }
            (
                Change::Label {
                    id: current_id,
                    fields: current_fields,
                },
                Change::Label {
                    id: incoming_id,
                    fields: incoming_fields,
                },
            ) => {
                if current_id != incoming_id {
                    return Err(CoreError::MergeIdMismatch);
                }

                let merged_fields = merge_fields(current_fields, incoming_fields);

                Ok(Record {
                    state: Change::Label {
                        id: current_id.clone(),
                        fields: merged_fields,
                    },
                })
            }
            (
                Change::TaskLabel {
                    id: current_id,
                    fields: current_fields,
                },
                Change::TaskLabel {
                    id: incoming_id,
                    fields: incoming_fields,
                },
            ) => {
                if current_id != incoming_id {
                    return Err(CoreError::MergeIdMismatch);
                }

                let merged_fields = merge_fields(current_fields, incoming_fields);

                Ok(Record {
                    state: Change::TaskLabel {
                        id: current_id.clone(),
                        fields: merged_fields,
                    },
                })
            }

            _ => Err(CoreError::MergeEntityMismatch),
        },
    }
}

fn merge_fields<F: Ord + Copy>(
    current: &BTreeMap<F, FieldChange>,
    incoming: &BTreeMap<F, FieldChange>,
) -> BTreeMap<F, FieldChange> {
    let mut merged = current.clone();
    for (field, incoming_change) in incoming {
        let should_apply = match merged.get(field) {
            None => true,
            Some(current_change) => incoming_change.hlc > current_change.hlc,
        };
        if should_apply {
            merged.insert(*field, incoming_change.clone());
        }
    }
    merged
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::change::{FieldChange, TaskField};
    use crate::ids::{LabelId, ProjectId, TaskId, TaskLabelKey};
    use serde_json::json;
    use std::collections::BTreeMap;

    #[test]
    fn creates_partial_record_from_change() {
        let change = Change::Task {
            id: TaskId::new_v7(1_700_000_000_000, [1; 10]),
            fields: BTreeMap::from([(
                TaskField::Title,
                FieldChange {
                    v: serde_json::json!("Новый заголовок"),
                    hlc: "0001761650000000-0000-a1b2c3d4".parse().unwrap(),
                },
            )]),
        };

        let record = merge(None, &change).unwrap();

        assert_eq!(record.state, change);
    }

    #[test]
    fn creates_and_merges_every_entity_kind_without_mutating_current() {
        let task_id = "0193f1c2-7a4e-7c3b-9a10-5b2f8e1d4c77";
        let label_id = "33333333-3333-5333-8333-333333333333";
        let cases = [
            (
                "project",
                "name",
                json!("Original project"),
                json!("Updated project"),
            ),
            (
                "milestone",
                "name",
                json!("Original milestone"),
                json!("Updated milestone"),
            ),
            (
                "journal_entry",
                "body",
                json!("Original entry"),
                json!("Updated entry"),
            ),
            (
                "column",
                "name",
                json!("Original column"),
                json!("Updated column"),
            ),
            (
                "task",
                "title",
                json!("Original title"),
                json!("Updated title"),
            ),
            (
                "checklist_item",
                "text",
                json!("Original item"),
                json!("Updated item"),
            ),
            ("label", "color", json!("gray"), json!("blue")),
            ("task_label", "deleted", json!(false), json!(true)),
        ];

        for (entity, field, original_value, updated_value) in cases {
            let id = match entity {
                "task_label" => json!({ "task_id": task_id, "label_id": label_id }),
                "label" => json!(label_id),
                _ => json!(task_id),
            };
            let original: Change = serde_json::from_value(json!({
                "entity": entity,
                "id": id,
                "fields": {
                    (field): {
                        "v": original_value,
                        "hlc": "0001761650000000-0001-a1b2c3d4"
                    }
                }
            }))
            .expect("valid initial change fixture");
            let incoming: Change = serde_json::from_value(json!({
                "entity": entity,
                "id": id,
                "fields": {
                    (field): {
                        "v": updated_value,
                        "hlc": "0001761650000000-0002-a1b2c3d4"
                    }
                }
            }))
            .expect("valid incoming change fixture");

            let current = merge(None, &original).expect("create a partial record");
            assert_eq!(
                current.state, original,
                "{entity}: creation must preserve fields"
            );
            let before = current.clone();

            let merged = merge(Some(&current), &incoming).expect("merge matching records");

            assert_eq!(merged.state, incoming, "{entity}: newer field must win");
            assert_eq!(current, before, "{entity}: original record changed");
        }
    }

    #[test]
    fn rejects_mismatched_ids_and_entity_kinds_without_mutating_current() {
        let task_id = TaskId::new_v7(1_700_000_000_000, [1; 10]);
        let other_task_id = TaskId::new_v7(1_700_000_000_000, [2; 10]);
        let label_id = LabelId::from_uuid(
            uuid::Uuid::parse_str("33333333-3333-5333-8333-333333333333").unwrap(),
        );
        let other_label_id = LabelId::from_uuid(
            uuid::Uuid::parse_str("44444444-4444-5444-8444-444444444444").unwrap(),
        );
        let task = Change::Task {
            id: task_id.clone(),
            fields: BTreeMap::new(),
        };
        let task_label = |task_id, label_id| Change::TaskLabel {
            id: TaskLabelKey { task_id, label_id },
            fields: BTreeMap::new(),
        };
        let cases = [
            (
                "different task IDs",
                task.clone(),
                Change::Task {
                    id: other_task_id.clone(),
                    fields: BTreeMap::new(),
                },
            ),
            (
                "same task but different labels",
                task_label(task_id.clone(), label_id.clone()),
                task_label(task_id.clone(), other_label_id),
            ),
            (
                "same label but different tasks",
                task_label(task_id.clone(), label_id.clone()),
                task_label(other_task_id, label_id),
            ),
        ];

        for (name, original, incoming) in cases {
            let current = merge(None, &original).expect("create a partial record");
            let before = current.clone();

            assert!(
                matches!(
                    merge(Some(&current), &incoming),
                    Err(CoreError::MergeIdMismatch)
                ),
                "{name}: expected an ID mismatch"
            );
            assert_eq!(current, before, "{name}: original record changed");
        }

        // Equal UUID values do not make a task and a project the same record.
        let project = Change::Project {
            id: ProjectId::from_uuid(task_id.as_uuid()),
            fields: BTreeMap::new(),
        };
        let current = merge(None, &task).expect("create a task record");
        let before = current.clone();

        assert!(matches!(
            merge(Some(&current), &project),
            Err(CoreError::MergeEntityMismatch)
        ));
        assert_eq!(
            current, before,
            "entity mismatch must preserve the original record"
        );
    }

    #[test]
    fn merges_title_by_hlc_without_losing_notes_or_mutating_current() {
        let field_change = |value: &str, counter: &str| FieldChange {
            v: serde_json::json!(value),
            hlc: format!("0001761650000000-{counter}-a1b2c3d4")
                .parse()
                .unwrap(),
        };
        let cases = [
            (
                "missing title",
                None,
                field_change("Incoming title", "0004"),
                field_change("Incoming title", "0004"),
            ),
            (
                "newer counter",
                Some(field_change("Saved title", "0003")),
                field_change("Incoming title", "0004"),
                field_change("Incoming title", "0004"),
            ),
            (
                "older counter",
                Some(field_change("Saved title", "0003")),
                field_change("Incoming title", "0002"),
                field_change("Saved title", "0003"),
            ),
            (
                "equal counter with different value",
                Some(field_change("Saved title", "0003")),
                field_change("Different title", "0003"),
                field_change("Saved title", "0003"),
            ),
        ];

        for (name, saved_title, incoming_title, expected_title) in cases {
            let id = TaskId::new_v7(1_700_000_000_000, [1; 10]);
            let notes = field_change("Saved notes", "0003");
            let mut fields = BTreeMap::from([(TaskField::Notes, notes.clone())]);
            if let Some(title) = saved_title {
                fields.insert(TaskField::Title, title);
            }
            let current = Record {
                state: Change::Task {
                    id: id.clone(),
                    fields,
                },
            };
            let before = current.clone();
            let change = Change::Task {
                id,
                fields: BTreeMap::from([(TaskField::Title, incoming_title)]),
            };

            let merged = merge(Some(&current), &change).unwrap();

            match &merged.state {
                Change::Task { fields, .. } => {
                    assert_eq!(
                        fields.get(&TaskField::Title),
                        Some(&expected_title),
                        "{name}"
                    );
                    assert_eq!(fields.get(&TaskField::Notes), Some(&notes), "{name}");
                }
                _ => panic!("{name}: expected a task record"),
            }
            assert_eq!(current, before, "{name}: original record changed");
        }
    }
}
