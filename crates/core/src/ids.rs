use serde::{Deserialize, Serialize};
use uuid::{Uuid, uuid};

/// The shared Inbox project ID used by every device.
/// Changing it after data has been created requires a migration.
pub const INBOX_ID: ProjectId = ProjectId(uuid!("01a1d56b-3cab-71f9-828c-e74c9e652a46"));

// Stable namespace for label IDs derived from normalized names.
// Changing it would assign new IDs to existing labels.
const LABEL_NAMESPACE: Uuid = uuid!("2a129103-0af8-4bfe-8575-602dc601a0c2");

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectId(Uuid);

impl ProjectId {
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }

    pub fn new_v7(unix_ms: u64, random_bytes: [u8; 10]) -> Self {
        Self(uuid_v7_from_parts(unix_ms, random_bytes))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MilestoneId(Uuid);

impl MilestoneId {
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }

    pub fn new_v7(unix_ms: u64, random_bytes: [u8; 10]) -> Self {
        Self(uuid_v7_from_parts(unix_ms, random_bytes))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JournalEntryId(Uuid);

impl JournalEntryId {
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }

    pub fn new_v7(unix_ms: u64, random_bytes: [u8; 10]) -> Self {
        Self(uuid_v7_from_parts(unix_ms, random_bytes))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ColumnId(Uuid);

impl ColumnId {
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }

    pub fn new_v7(unix_ms: u64, random_bytes: [u8; 10]) -> Self {
        Self(uuid_v7_from_parts(unix_ms, random_bytes))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskId(Uuid);

impl TaskId {
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }

    pub fn new_v7(unix_ms: u64, random_bytes: [u8; 10]) -> Self {
        Self(uuid_v7_from_parts(unix_ms, random_bytes))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChecklistItemId(Uuid);

impl ChecklistItemId {
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }

    pub fn new_v7(unix_ms: u64, random_bytes: [u8; 10]) -> Self {
        Self(uuid_v7_from_parts(unix_ms, random_bytes))
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LabelId(Uuid);

impl LabelId {
    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

pub fn label_id(name: &str) -> LabelId {
    let normalized_name = name.trim().to_lowercase();
    let uuid = Uuid::new_v5(&LABEL_NAMESPACE, normalized_name.as_bytes());
    LabelId::from_uuid(uuid)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskLabelKey {
    pub task_id: TaskId,
    pub label_id: LabelId,
}

fn uuid_v7_from_parts(unix_ms: u64, random_bytes: [u8; 10]) -> Uuid {
    uuid::Builder::from_unix_timestamp_millis(unix_ms, &random_bytes).into_uuid()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_uuid_v7_with_expected_timestamp_and_variant() {
        let unix_ms = 1_700_000_000_123_u64;
        let random_bytes = [0xab; 10];

        let uuid = uuid_v7_from_parts(unix_ms, random_bytes);

        assert_eq!(uuid.get_version_num(), 7);
        assert_eq!(uuid.get_variant(), uuid::Variant::RFC4122);
        assert_eq!(&uuid.as_bytes()[..6], &unix_ms.to_be_bytes()[2..]);
        assert_eq!(uuid, uuid_v7_from_parts(unix_ms, random_bytes));
        assert_ne!(uuid, uuid_v7_from_parts(unix_ms + 1, random_bytes));
        assert_ne!(uuid, uuid_v7_from_parts(unix_ms, [0xcd; 10]));
    }

    #[test]
    fn id_constructors_generate_expected_uuid_v7() {
        let unix_ms = 1_700_000_000_123;
        let random_bytes = [0xab; 10];
        let expected = uuid_v7_from_parts(unix_ms, random_bytes);

        let generated = [
            ProjectId::new_v7(unix_ms, random_bytes).as_uuid(),
            MilestoneId::new_v7(unix_ms, random_bytes).as_uuid(),
            JournalEntryId::new_v7(unix_ms, random_bytes).as_uuid(),
            ColumnId::new_v7(unix_ms, random_bytes).as_uuid(),
            TaskId::new_v7(unix_ms, random_bytes).as_uuid(),
            ChecklistItemId::new_v7(unix_ms, random_bytes).as_uuid(),
        ];

        for uuid in generated {
            assert_eq!(uuid, expected);
        }
    }

    #[test]
    fn label_and_inbox_identities_remain_stable() {
        for name in ["Bug", " bug ", "BUG"] {
            assert_eq!(
                label_id(name).as_uuid(),
                uuid!("38e67d22-6391-5238-843a-ad1e49d71eed")
            );
        }

        assert_ne!(label_id("bug"), label_id("feature"));
        assert_eq!(
            INBOX_ID.as_uuid(),
            uuid!("01a1d56b-3cab-71f9-828c-e74c9e652a46")
        );
    }
}
