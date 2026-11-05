use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    hlc::Hlc,
    ids::{
        ChecklistItemId, ColumnId, JournalEntryId, LabelId, MilestoneId, ProjectId, TaskId,
        TaskLabelKey,
    },
};

pub type FieldValue = Value;
pub type Clocks<F> = BTreeMap<F, Hlc>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskField {
    Number,
    ProjectId,
    ColumnId,
    MilestoneId,
    ParentId,
    Title,
    Notes,
    Priority,
    CompletedAt,
    Position,
    Deleted,
    CreatedAt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectField {
    Name,
    Key,
    Description,
    Color,
    Status,
    NextStep,
    RepoUrl,
    Position,
    IsInbox,
    Deleted,
    CreatedAt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MilestoneField {
    ProjectId,
    Name,
    Description,
    TargetDate,
    Status,
    ReachedAt,
    Position,
    Deleted,
    CreatedAt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum JournalEntryField {
    ProjectId,
    EntryDate,
    Body,
    Kind,
    Deleted,
    CreatedAt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ColumnField {
    ProjectId,
    Name,
    Position,
    IsDone,
    Deleted,
    CreatedAt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChecklistItemField {
    TaskId,
    Text,
    Done,
    Position,
    Deleted,
    CreatedAt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LabelField {
    Name,
    Color,
    Deleted,
    CreatedAt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskLabelField {
    Deleted,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FieldChange {
    pub v: FieldValue,
    pub hlc: Hlc,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "entity", rename_all = "snake_case")]
pub enum Change {
    Project {
        id: ProjectId,
        fields: BTreeMap<ProjectField, FieldChange>,
    },
    Milestone {
        id: MilestoneId,
        fields: BTreeMap<MilestoneField, FieldChange>,
    },
    JournalEntry {
        id: JournalEntryId,
        fields: BTreeMap<JournalEntryField, FieldChange>,
    },
    Column {
        id: ColumnId,
        fields: BTreeMap<ColumnField, FieldChange>,
    },
    Task {
        id: TaskId,
        fields: BTreeMap<TaskField, FieldChange>,
    },
    ChecklistItem {
        id: ChecklistItemId,
        fields: BTreeMap<ChecklistItemField, FieldChange>,
    },
    Label {
        id: LabelId,
        fields: BTreeMap<LabelField, FieldChange>,
    },
    TaskLabel {
        id: TaskLabelKey,
        fields: BTreeMap<TaskLabelField, FieldChange>,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn task_change_has_expected_json_and_round_trips() {
        let task_id = "11111111-1111-4111-8111-111111111111";
        let column_id = "22222222-2222-4222-8222-222222222222";
        let hlc: Hlc =
            serde_json::from_value(json!("0001761650000000-0000-a1b2c3d4")).expect("valid HLC");
        let change = Change::Task {
            id: serde_json::from_value(json!(task_id)).expect("valid task ID"),
            fields: BTreeMap::from([
                (
                    TaskField::Title,
                    FieldChange {
                        v: json!("Написать ТЗ"),
                        hlc,
                    },
                ),
                (
                    TaskField::ColumnId,
                    FieldChange {
                        v: json!(column_id),
                        hlc,
                    },
                ),
            ]),
        };
        let expected: Value =
            serde_json::from_str(include_str!("../tests/fixtures/task_change.json"))
                .expect("valid task change snapshot");

        let actual = serde_json::to_value(&change).expect("serialize task change");
        assert_eq!(actual, expected);
        let restored: Change = serde_json::from_value(actual).expect("deserialize task change");
        assert_eq!(restored, change);
    }

    #[test]
    fn task_label_change_has_expected_json_and_round_trips() {
        let task_id = "11111111-1111-4111-8111-111111111111";
        let label_id = "33333333-3333-4333-8333-333333333333";
        let hlc: Hlc =
            serde_json::from_value(json!("0001761650000000-0000-a1b2c3d4")).expect("valid HLC");
        let change = Change::TaskLabel {
            id: serde_json::from_value(json!({
                "task_id": task_id,
                "label_id": label_id
            }))
            .expect("valid task-label key"),
            fields: BTreeMap::from([(
                TaskLabelField::Deleted,
                FieldChange {
                    v: json!(true),
                    hlc,
                },
            )]),
        };
        let expected: Value =
            serde_json::from_str(include_str!("../tests/fixtures/task_label_change.json"))
                .expect("valid task-label change snapshot");

        let actual = serde_json::to_value(&change).expect("serialize task-label change");
        assert_eq!(actual, expected);
        let restored: Change =
            serde_json::from_value(actual).expect("deserialize task-label change");
        assert_eq!(restored, change);
    }
}
