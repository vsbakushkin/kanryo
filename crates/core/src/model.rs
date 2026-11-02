use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use crate::error::CoreError;
use crate::ids::{
    ChecklistItemId, ColumnId, JournalEntryId, LabelId, MilestoneId, ProjectId, TaskId,
};
use crate::validation;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Priority {
    P1,
    P2,
    P3,
    P4,
}

impl TryFrom<i16> for Priority {
    type Error = CoreError;

    fn try_from(value: i16) -> Result<Self, CoreError> {
        match value {
            1 => Ok(Priority::P1),
            2 => Ok(Priority::P2),
            3 => Ok(Priority::P3),
            4 => Ok(Priority::P4),
            _ => Err(CoreError::InvalidPriority { value }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProjectStatus {
    Idea,
    Active,
    Paused,
    Shipped,
    Abandoned,
}

impl TryFrom<&str> for ProjectStatus {
    type Error = CoreError;

    fn try_from(value: &str) -> Result<Self, CoreError> {
        match value {
            "idea" => Ok(ProjectStatus::Idea),
            "active" => Ok(ProjectStatus::Active),
            "paused" => Ok(ProjectStatus::Paused),
            "shipped" => Ok(ProjectStatus::Shipped),
            "abandoned" => Ok(ProjectStatus::Abandoned),
            _ => Err(CoreError::InvalidProjectStatus {
                value: value.to_string(),
            }),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EntityKind {
    Project,
    Milestone,
    JournalEntry,
    Column,
    Task,
    ChecklistItem,
    Label,
    TaskLabel,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Project {
    id: ProjectId,
    name: String,
    key: String,
    description: String,
    color: String,
    status: ProjectStatus,
    next_step: String,
    repo_url: Option<String>,
    position: String,
    is_inbox: bool,
    deleted: bool,
    created_at: DateTime<Utc>,
}

impl Project {
    pub fn new(
        id: ProjectId,
        name: String,
        key: String,
        status: &str,
        position: String,
        created_at: DateTime<Utc>,
    ) -> Result<Self, CoreError> {
        Ok(Self {
            id,
            name,
            key,
            description: String::new(),
            color: "gray".to_string(),
            status: ProjectStatus::try_from(status)?,
            next_step: String::new(),
            repo_url: None,
            position,
            is_inbox: false,
            deleted: false,
            created_at,
        })
    }

    pub fn set_status(&mut self, value: &str) -> Result<(), CoreError> {
        let project_status = ProjectStatus::try_from(value)?;
        self.status = project_status;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Milestone {
    id: MilestoneId,
    project_id: ProjectId,
    name: String,
    description: String,
    target_date: Option<NaiveDate>,
    status: String,
    reached_at: Option<DateTime<Utc>>,
    position: String,
    deleted: bool,
    created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JournalEntry {
    id: JournalEntryId,
    project_id: ProjectId,
    entry_date: NaiveDate,
    body: String,
    kind: String,
    deleted: bool,
    created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Column {
    id: ColumnId,
    project_id: ProjectId,
    name: String,
    position: String,
    is_done: bool,
    deleted: bool,
    created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Task {
    id: TaskId,
    number: Option<i64>,
    project_id: ProjectId,
    column_id: Option<ColumnId>,
    milestone_id: Option<MilestoneId>,
    parent_id: Option<TaskId>,
    title: String,
    notes: String,
    priority: Priority,
    completed_at: Option<DateTime<Utc>>,
    position: String,
    deleted: bool,
    created_at: DateTime<Utc>,
}

impl Task {
    pub fn new(
        id: TaskId,
        project_id: ProjectId,
        title: String,
        priority: i16,
        position: String,
        created_at: DateTime<Utc>,
    ) -> Result<Self, CoreError> {
        validation::validate_task_title(&title)?;
        Ok(Self {
            id,
            number: None,
            project_id,
            column_id: None,
            milestone_id: None,
            parent_id: None,
            title,
            notes: String::new(),
            priority: Priority::try_from(priority)?,
            completed_at: None,
            position,
            deleted: false,
            created_at,
        })
    }

    pub fn set_title(&mut self, title: String) -> Result<(), CoreError> {
        validation::validate_task_title(&title)?;
        self.title = title;
        Ok(())
    }

    pub fn set_priority(&mut self, priority: i16) -> Result<(), CoreError> {
        let priority_enum = Priority::try_from(priority)?;
        self.priority = priority_enum;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChecklistItem {
    id: ChecklistItemId,
    task_id: TaskId,
    text: String,
    done: bool,
    position: String,
    deleted: bool,
    created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Label {
    id: LabelId,
    name: String,
    color: String,
    deleted: bool,
    created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TaskLabel {
    task_id: TaskId,
    label_id: LabelId,
    deleted: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_task() -> Task {
        Task::new(
            TaskId::new_v7(1_700_000_000_000, [1; 10]),
            ProjectId::new_v7(1_700_000_000_000, [2; 10]),
            "Original title".to_string(),
            4,
            "a".to_string(),
            DateTime::parse_from_rfc3339("2025-01-01T00:00:00Z")
                .unwrap()
                .with_timezone(&Utc),
        )
        .unwrap()
    }

    fn test_project() -> Project {
        Project::new(
            ProjectId::new_v7(1_700_000_000_000, [2; 10]),
            "Test project".to_string(),
            "TEST".to_string(),
            "idea",
            "a".to_string(),
            DateTime::parse_from_rfc3339("2025-01-01T00:00:00Z")
                .unwrap()
                .with_timezone(&Utc),
        )
        .unwrap()
    }

    #[test]
    fn task_new_rejects_invalid_values() {
        for (title, priority, expected_error) in [
            ("", 4, validation::validate_task_title("").unwrap_err()),
            ("Valid title", 5, CoreError::InvalidPriority { value: 5 }),
        ] {
            let error = Task::new(
                TaskId::new_v7(1_700_000_000_000, [1; 10]),
                ProjectId::new_v7(1_700_000_000_000, [2; 10]),
                title.to_string(),
                priority,
                "a".to_string(),
                DateTime::parse_from_rfc3339("2025-01-01T00:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc),
            )
            .unwrap_err();
            assert_eq!(
                std::mem::discriminant(&error),
                std::mem::discriminant(&expected_error),
            );
            if let CoreError::InvalidPriority { value } = error {
                assert_eq!(value, priority);
            }
        }
    }

    #[test]
    fn task_set_title_updates_value_and_preserves_entity_on_error() {
        let mut task = test_task();

        task.set_title("Updated title".to_string()).unwrap();
        assert_eq!(task.title, "Updated title");

        let saved = task.clone();
        let expected_error = validation::validate_task_title("").unwrap_err();
        let error = task.set_title(String::new()).unwrap_err();
        assert_eq!(
            std::mem::discriminant(&error),
            std::mem::discriminant(&expected_error),
        );
        assert_eq!(task, saved);
    }

    #[test]
    fn task_set_priority_updates_value_and_preserves_entity_on_error() {
        let mut task = test_task();

        task.set_priority(1).unwrap();
        assert_eq!(task.priority, Priority::P1);

        let saved = task.clone();
        assert!(matches!(
            task.set_priority(5),
            Err(CoreError::InvalidPriority { value: 5 })
        ));
        assert_eq!(task, saved);
    }

    #[test]
    fn project_set_status_updates_value_and_preserves_entity_on_error() {
        assert!(matches!(
            Project::new(
                ProjectId::new_v7(1_700_000_000_000, [2; 10]),
                "Test project".to_string(),
                "TEST".to_string(),
                "unknown",
                "a".to_string(),
                DateTime::parse_from_rfc3339("2025-01-01T00:00:00Z")
                    .unwrap()
                    .with_timezone(&Utc),
            ),
            Err(CoreError::InvalidProjectStatus { value }) if value == "unknown"
        ));

        let mut project = test_project();

        project.set_status("active").unwrap();
        assert_eq!(project.status, ProjectStatus::Active);

        let saved = project.clone();
        assert!(matches!(
            project.set_status("unknown"),
            Err(CoreError::InvalidProjectStatus { value }) if value == "unknown"
        ));
        assert_eq!(project, saved);
    }

    #[test]
    fn priority_try_from_valid_values() {
        for (value, expected) in [
            (1, Priority::P1),
            (2, Priority::P2),
            (3, Priority::P3),
            (4, Priority::P4),
        ] {
            assert_eq!(Priority::try_from(value).unwrap(), expected);
        }
    }

    #[test]
    fn priority_try_from_invalid_values() {
        for value in [0, 5, -1] {
            assert!(matches!(
                Priority::try_from(value),
                Err(CoreError::InvalidPriority { value: invalid }) if invalid == value
            ));
        }
    }

    #[test]
    fn project_status_try_from_valid_values() {
        for (value, expected) in [
            ("idea", ProjectStatus::Idea),
            ("active", ProjectStatus::Active),
            ("paused", ProjectStatus::Paused),
            ("shipped", ProjectStatus::Shipped),
            ("abandoned", ProjectStatus::Abandoned),
        ] {
            assert_eq!(ProjectStatus::try_from(value).unwrap(), expected);
        }
    }

    #[test]
    fn project_status_try_from_invalid_values() {
        for value in ["", "unknown", "Active", " active", "active "] {
            assert!(matches!(
                ProjectStatus::try_from(value),
                Err(CoreError::InvalidProjectStatus { value: invalid }) if invalid == value
            ));
        }
    }
}
