use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use crate::ids::{
    ChecklistItemId, ColumnId, JournalEntryId, LabelId, MilestoneId, ProjectId, TaskId,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Priority {
    P1,
    P2,
    P3,
    P4,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProjectStatus {
    Idea,
    Active,
    Paused,
    Shipped,
    Abandoned,
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
