use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Board {
    pub id: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct List {
    pub id: String,
    pub board_id: String,
    pub title: String,
    pub position: f64,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Label {
    pub id: String,
    pub board_id: String,
    pub name: String,
    pub color: String,
    pub created_at: String,
}

/// Row shape for SQL (no nested fields).
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct CardRow {
    pub id: String,
    pub list_id: String,
    pub title: String,
    pub description: String,
    pub position: f64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ChecklistItem {
    pub id: String,
    pub card_id: String,
    pub title: String,
    /// SQLite INTEGER 0/1
    pub done: bool,
    pub position: f64,
    pub created_at: String,
}

/// Card returned to the frontend (includes labels + checklist).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Card {
    pub id: String,
    pub list_id: String,
    pub title: String,
    pub description: String,
    pub position: f64,
    pub created_at: String,
    pub updated_at: String,
    pub labels: Vec<Label>,
    pub checklist: Vec<ChecklistItem>,
}

impl Card {
    pub fn from_row(
        row: CardRow,
        labels: Vec<Label>,
        checklist: Vec<ChecklistItem>,
    ) -> Self {
        Self {
            id: row.id,
            list_id: row.list_id,
            title: row.title,
            description: row.description,
            position: row.position,
            created_at: row.created_at,
            updated_at: row.updated_at,
            labels,
            checklist,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardFull {
    pub board: Board,
    pub labels: Vec<Label>,
    pub lists: Vec<ListWithCards>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListWithCards {
    #[serde(flatten)]
    pub list: List,
    pub cards: Vec<Card>,
}

#[derive(Debug, Deserialize)]
pub struct CreateBoardInput {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct RenameBoardInput {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateListInput {
    pub board_id: String,
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct RenameListInput {
    pub id: String,
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct ReorderListInput {
    pub id: String,
    pub position: f64,
}

#[derive(Debug, Deserialize)]
pub struct CreateCardInput {
    pub list_id: String,
    pub title: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateCardInput {
    pub id: String,
    pub title: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MoveCardInput {
    pub id: String,
    pub list_id: String,
    pub position: f64,
}

#[derive(Debug, Deserialize)]
pub struct CreateLabelInput {
    pub board_id: String,
    pub name: String,
    pub color: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateLabelInput {
    pub id: String,
    pub name: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SetCardLabelsInput {
    pub card_id: String,
    pub label_ids: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateChecklistItemInput {
    pub card_id: String,
    pub title: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateChecklistItemInput {
    pub id: String,
    pub title: Option<String>,
    pub done: Option<bool>,
}
