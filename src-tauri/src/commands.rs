use chrono::Utc;
use sqlx::SqlitePool;
use tauri::State;
use uuid::Uuid;

use crate::error::AppError;
use crate::models::*;

pub struct DbState(pub SqlitePool);

fn now() -> String {
    Utc::now().to_rfc3339()
}

fn new_id() -> String {
    Uuid::new_v4().to_string()
}

fn validate_color(color: &str) -> Result<String, AppError> {
    let c = color.trim().to_lowercase();
    // Accept #RGB or #RRGGBB
    let ok = (c.len() == 4 || c.len() == 7)
        && c.starts_with('#')
        && c.chars().skip(1).all(|ch| ch.is_ascii_hexdigit());
    if !ok {
        return Err(AppError::Message(
            "color must be #RGB or #RRGGBB".into(),
        ));
    }
    Ok(c)
}

// ── Boards ──────────────────────────────────────────────────────────

#[tauri::command]
pub async fn list_boards(db: State<'_, DbState>) -> Result<Vec<Board>, AppError> {
    let boards = sqlx::query_as::<_, Board>(
        "SELECT id, name, created_at, updated_at FROM boards ORDER BY updated_at DESC",
    )
    .fetch_all(&db.0)
    .await?;
    Ok(boards)
}

#[tauri::command]
pub async fn create_board(
    db: State<'_, DbState>,
    input: CreateBoardInput,
) -> Result<Board, AppError> {
    let name = input.name.trim().to_string();
    if name.is_empty() {
        return Err(AppError::Message("board name cannot be empty".into()));
    }

    let id = new_id();
    let ts = now();

    sqlx::query(
        "INSERT INTO boards (id, name, created_at, updated_at) VALUES (?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(&name)
    .bind(&ts)
    .bind(&ts)
    .execute(&db.0)
    .await?;

    // Default lists like Trello
    for (i, title) in ["To Do", "Doing", "Done"].iter().enumerate() {
        let list_id = new_id();
        sqlx::query(
            "INSERT INTO lists (id, board_id, title, position, created_at) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(&list_id)
        .bind(&id)
        .bind(*title)
        .bind(i as f64)
        .bind(&ts)
        .execute(&db.0)
        .await?;
    }

    // Default labels
    let defaults = [
        ("Bug", "#ef4444"),
        ("Feature", "#22c55e"),
        ("Melhoria", "#3b82f6"),
        ("Urgente", "#f97316"),
    ];
    for (label_name, color) in defaults {
        let label_id = new_id();
        sqlx::query(
            "INSERT INTO labels (id, board_id, name, color, created_at) VALUES (?, ?, ?, ?, ?)",
        )
        .bind(&label_id)
        .bind(&id)
        .bind(label_name)
        .bind(color)
        .bind(&ts)
        .execute(&db.0)
        .await?;
    }

    Ok(Board {
        id,
        name,
        created_at: ts.clone(),
        updated_at: ts,
    })
}

#[tauri::command]
pub async fn get_board(db: State<'_, DbState>, id: String) -> Result<BoardFull, AppError> {
    let board = sqlx::query_as::<_, Board>(
        "SELECT id, name, created_at, updated_at FROM boards WHERE id = ?",
    )
    .bind(&id)
    .fetch_optional(&db.0)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("board {id}")))?;

    let labels = fetch_board_labels(&db.0, &id).await?;

    let lists = sqlx::query_as::<_, List>(
        "SELECT id, board_id, title, position, created_at FROM lists WHERE board_id = ? ORDER BY position ASC",
    )
    .bind(&id)
    .fetch_all(&db.0)
    .await?;

    let mut lists_with_cards = Vec::with_capacity(lists.len());
    for list in lists {
        let rows = sqlx::query_as::<_, CardRow>(
            "SELECT id, list_id, title, description, position, created_at, updated_at \
             FROM cards WHERE list_id = ? ORDER BY position ASC",
        )
        .bind(&list.id)
        .fetch_all(&db.0)
        .await?;

        let mut cards = Vec::with_capacity(rows.len());
        for row in rows {
            cards.push(hydrate_card(&db.0, row).await?);
        }

        lists_with_cards.push(ListWithCards { list, cards });
    }

    Ok(BoardFull {
        board,
        labels,
        lists: lists_with_cards,
    })
}

#[tauri::command]
pub async fn rename_board(
    db: State<'_, DbState>,
    input: RenameBoardInput,
) -> Result<Board, AppError> {
    let name = input.name.trim().to_string();
    if name.is_empty() {
        return Err(AppError::Message("board name cannot be empty".into()));
    }
    let ts = now();

    let result = sqlx::query("UPDATE boards SET name = ?, updated_at = ? WHERE id = ?")
        .bind(&name)
        .bind(&ts)
        .bind(&input.id)
        .execute(&db.0)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("board {}", input.id)));
    }

    sqlx::query_as::<_, Board>(
        "SELECT id, name, created_at, updated_at FROM boards WHERE id = ?",
    )
    .bind(&input.id)
    .fetch_one(&db.0)
    .await
    .map_err(AppError::from)
}

#[tauri::command]
pub async fn delete_board(db: State<'_, DbState>, id: String) -> Result<(), AppError> {
    let result = sqlx::query("DELETE FROM boards WHERE id = ?")
        .bind(&id)
        .execute(&db.0)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("board {id}")));
    }
    Ok(())
}

// ── Lists ───────────────────────────────────────────────────────────

#[tauri::command]
pub async fn create_list(
    db: State<'_, DbState>,
    input: CreateListInput,
) -> Result<List, AppError> {
    let title = input.title.trim().to_string();
    if title.is_empty() {
        return Err(AppError::Message("list title cannot be empty".into()));
    }

    let max_pos: Option<f64> = sqlx::query_scalar(
        "SELECT MAX(position) FROM lists WHERE board_id = ?",
    )
    .bind(&input.board_id)
    .fetch_one(&db.0)
    .await?;

    let position = max_pos.map(|p| p + 1.0).unwrap_or(0.0);
    let id = new_id();
    let ts = now();

    sqlx::query(
        "INSERT INTO lists (id, board_id, title, position, created_at) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(&input.board_id)
    .bind(&title)
    .bind(position)
    .bind(&ts)
    .execute(&db.0)
    .await?;

    touch_board(&db.0, &input.board_id).await?;

    Ok(List {
        id,
        board_id: input.board_id,
        title,
        position,
        created_at: ts,
    })
}

#[tauri::command]
pub async fn rename_list(
    db: State<'_, DbState>,
    input: RenameListInput,
) -> Result<List, AppError> {
    let title = input.title.trim().to_string();
    if title.is_empty() {
        return Err(AppError::Message("list title cannot be empty".into()));
    }

    let result = sqlx::query("UPDATE lists SET title = ? WHERE id = ?")
        .bind(&title)
        .bind(&input.id)
        .execute(&db.0)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("list {}", input.id)));
    }

    let list = sqlx::query_as::<_, List>(
        "SELECT id, board_id, title, position, created_at FROM lists WHERE id = ?",
    )
    .bind(&input.id)
    .fetch_one(&db.0)
    .await?;

    touch_board(&db.0, &list.board_id).await?;
    Ok(list)
}

#[tauri::command]
pub async fn delete_list(db: State<'_, DbState>, id: String) -> Result<(), AppError> {
    let board_id: Option<String> =
        sqlx::query_scalar("SELECT board_id FROM lists WHERE id = ?")
            .bind(&id)
            .fetch_optional(&db.0)
            .await?;

    let Some(board_id) = board_id else {
        return Err(AppError::NotFound(format!("list {id}")));
    };

    sqlx::query("DELETE FROM lists WHERE id = ?")
        .bind(&id)
        .execute(&db.0)
        .await?;

    touch_board(&db.0, &board_id).await?;
    Ok(())
}

#[tauri::command]
pub async fn reorder_list(
    db: State<'_, DbState>,
    input: ReorderListInput,
) -> Result<List, AppError> {
    let result = sqlx::query("UPDATE lists SET position = ? WHERE id = ?")
        .bind(input.position)
        .bind(&input.id)
        .execute(&db.0)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("list {}", input.id)));
    }

    let list = sqlx::query_as::<_, List>(
        "SELECT id, board_id, title, position, created_at FROM lists WHERE id = ?",
    )
    .bind(&input.id)
    .fetch_one(&db.0)
    .await?;

    touch_board(&db.0, &list.board_id).await?;
    Ok(list)
}

// ── Cards ───────────────────────────────────────────────────────────

#[tauri::command]
pub async fn create_card(
    db: State<'_, DbState>,
    input: CreateCardInput,
) -> Result<Card, AppError> {
    let title = input.title.trim().to_string();
    if title.is_empty() {
        return Err(AppError::Message("card title cannot be empty".into()));
    }
    let description = input
        .description
        .unwrap_or_default()
        .trim()
        .to_string();

    let max_pos: Option<f64> =
        sqlx::query_scalar("SELECT MAX(position) FROM cards WHERE list_id = ?")
            .bind(&input.list_id)
            .fetch_one(&db.0)
            .await?;

    let position = max_pos.map(|p| p + 1.0).unwrap_or(0.0);
    let id = new_id();
    let ts = now();

    sqlx::query(
        "INSERT INTO cards (id, list_id, title, description, position, created_at, updated_at) \
         VALUES (?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(&input.list_id)
    .bind(&title)
    .bind(&description)
    .bind(position)
    .bind(&ts)
    .bind(&ts)
    .execute(&db.0)
    .await?;

    if let Ok(Some(board_id)) = sqlx::query_scalar::<_, String>(
        "SELECT board_id FROM lists WHERE id = ?",
    )
    .bind(&input.list_id)
    .fetch_optional(&db.0)
    .await
    {
        let _ = touch_board(&db.0, &board_id).await;
    }

    Ok(Card {
        id,
        list_id: input.list_id,
        title,
        description,
        position,
        created_at: ts.clone(),
        updated_at: ts,
        labels: vec![],
        checklist: vec![],
    })
}

#[tauri::command]
pub async fn update_card(
    db: State<'_, DbState>,
    input: UpdateCardInput,
) -> Result<Card, AppError> {
    let row = sqlx::query_as::<_, CardRow>(
        "SELECT id, list_id, title, description, position, created_at, updated_at \
         FROM cards WHERE id = ?",
    )
    .bind(&input.id)
    .fetch_optional(&db.0)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("card {}", input.id)))?;

    let mut title = row.title;
    let mut description = row.description;

    if let Some(t) = input.title {
        let t = t.trim().to_string();
        if t.is_empty() {
            return Err(AppError::Message("card title cannot be empty".into()));
        }
        title = t;
    }
    if let Some(d) = input.description {
        description = d;
    }
    let updated_at = now();

    sqlx::query("UPDATE cards SET title = ?, description = ?, updated_at = ? WHERE id = ?")
        .bind(&title)
        .bind(&description)
        .bind(&updated_at)
        .bind(&row.id)
        .execute(&db.0)
        .await?;

    let labels = fetch_card_labels(&db.0, &row.id).await?;
    let checklist = fetch_checklist(&db.0, &row.id).await?;
    Ok(Card {
        id: row.id,
        list_id: row.list_id,
        title,
        description,
        position: row.position,
        created_at: row.created_at,
        updated_at,
        labels,
        checklist,
    })
}

#[tauri::command]
pub async fn delete_card(db: State<'_, DbState>, id: String) -> Result<(), AppError> {
    let result = sqlx::query("DELETE FROM cards WHERE id = ?")
        .bind(&id)
        .execute(&db.0)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("card {id}")));
    }
    Ok(())
}

#[tauri::command]
pub async fn move_card(db: State<'_, DbState>, input: MoveCardInput) -> Result<Card, AppError> {
    let ts = now();
    let result = sqlx::query(
        "UPDATE cards SET list_id = ?, position = ?, updated_at = ? WHERE id = ?",
    )
    .bind(&input.list_id)
    .bind(input.position)
    .bind(&ts)
    .bind(&input.id)
    .execute(&db.0)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("card {}", input.id)));
    }

    let row = sqlx::query_as::<_, CardRow>(
        "SELECT id, list_id, title, description, position, created_at, updated_at \
         FROM cards WHERE id = ?",
    )
    .bind(&input.id)
    .fetch_one(&db.0)
    .await?;

    if let Ok(Some(board_id)) = sqlx::query_scalar::<_, String>(
        "SELECT board_id FROM lists WHERE id = ?",
    )
    .bind(&row.list_id)
    .fetch_optional(&db.0)
    .await
    {
        let _ = touch_board(&db.0, &board_id).await;
    }

    hydrate_card(&db.0, row).await
}

// ── Labels ──────────────────────────────────────────────────────────

#[tauri::command]
pub async fn list_labels(db: State<'_, DbState>, board_id: String) -> Result<Vec<Label>, AppError> {
    fetch_board_labels(&db.0, &board_id).await
}

#[tauri::command]
pub async fn create_label(
    db: State<'_, DbState>,
    input: CreateLabelInput,
) -> Result<Label, AppError> {
    let name = input.name.trim().to_string();
    if name.is_empty() {
        return Err(AppError::Message("label name cannot be empty".into()));
    }
    let color = validate_color(&input.color)?;

    let exists: Option<String> =
        sqlx::query_scalar("SELECT id FROM boards WHERE id = ?")
            .bind(&input.board_id)
            .fetch_optional(&db.0)
            .await?;
    if exists.is_none() {
        return Err(AppError::NotFound(format!("board {}", input.board_id)));
    }

    let id = new_id();
    let ts = now();

    sqlx::query(
        "INSERT INTO labels (id, board_id, name, color, created_at) VALUES (?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(&input.board_id)
    .bind(&name)
    .bind(&color)
    .bind(&ts)
    .execute(&db.0)
    .await?;

    touch_board(&db.0, &input.board_id).await?;

    Ok(Label {
        id,
        board_id: input.board_id,
        name,
        color,
        created_at: ts,
    })
}

#[tauri::command]
pub async fn update_label(
    db: State<'_, DbState>,
    input: UpdateLabelInput,
) -> Result<Label, AppError> {
    let mut label = sqlx::query_as::<_, Label>(
        "SELECT id, board_id, name, color, created_at FROM labels WHERE id = ?",
    )
    .bind(&input.id)
    .fetch_optional(&db.0)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("label {}", input.id)))?;

    if let Some(name) = input.name {
        let n = name.trim().to_string();
        if n.is_empty() {
            return Err(AppError::Message("label name cannot be empty".into()));
        }
        label.name = n;
    }
    if let Some(color) = input.color {
        label.color = validate_color(&color)?;
    }

    sqlx::query("UPDATE labels SET name = ?, color = ? WHERE id = ?")
        .bind(&label.name)
        .bind(&label.color)
        .bind(&label.id)
        .execute(&db.0)
        .await?;

    touch_board(&db.0, &label.board_id).await?;
    Ok(label)
}

#[tauri::command]
pub async fn delete_label(db: State<'_, DbState>, id: String) -> Result<(), AppError> {
    let board_id: Option<String> =
        sqlx::query_scalar("SELECT board_id FROM labels WHERE id = ?")
            .bind(&id)
            .fetch_optional(&db.0)
            .await?;

    let Some(board_id) = board_id else {
        return Err(AppError::NotFound(format!("label {id}")));
    };

    sqlx::query("DELETE FROM labels WHERE id = ?")
        .bind(&id)
        .execute(&db.0)
        .await?;

    touch_board(&db.0, &board_id).await?;
    Ok(())
}

#[tauri::command]
pub async fn set_card_labels(
    db: State<'_, DbState>,
    input: SetCardLabelsInput,
) -> Result<Card, AppError> {
    let row = sqlx::query_as::<_, CardRow>(
        "SELECT id, list_id, title, description, position, created_at, updated_at \
         FROM cards WHERE id = ?",
    )
    .bind(&input.card_id)
    .fetch_optional(&db.0)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("card {}", input.card_id)))?;

    let board_id: String = sqlx::query_scalar(
        "SELECT board_id FROM lists WHERE id = ?",
    )
    .bind(&row.list_id)
    .fetch_optional(&db.0)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("list {}", row.list_id)))?;

    // Validate all labels belong to this board
    for label_id in &input.label_ids {
        let ok: Option<String> = sqlx::query_scalar(
            "SELECT id FROM labels WHERE id = ? AND board_id = ?",
        )
        .bind(label_id)
        .bind(&board_id)
        .fetch_optional(&db.0)
        .await?;
        if ok.is_none() {
            return Err(AppError::NotFound(format!("label {label_id}")));
        }
    }

    sqlx::query("DELETE FROM card_labels WHERE card_id = ?")
        .bind(&input.card_id)
        .execute(&db.0)
        .await?;

    for label_id in &input.label_ids {
        sqlx::query("INSERT INTO card_labels (card_id, label_id) VALUES (?, ?)")
            .bind(&input.card_id)
            .bind(label_id)
            .execute(&db.0)
            .await?;
    }

    let ts = now();
    sqlx::query("UPDATE cards SET updated_at = ? WHERE id = ?")
        .bind(&ts)
        .bind(&input.card_id)
        .execute(&db.0)
        .await?;

    touch_board(&db.0, &board_id).await?;

    let labels = fetch_card_labels(&db.0, &input.card_id).await?;
    let checklist = fetch_checklist(&db.0, &input.card_id).await?;
    Ok(Card {
        id: row.id,
        list_id: row.list_id,
        title: row.title,
        description: row.description,
        position: row.position,
        created_at: row.created_at,
        updated_at: ts,
        labels,
        checklist,
    })
}

// ── Checklist ───────────────────────────────────────────────────────

#[tauri::command]
pub async fn create_checklist_item(
    db: State<'_, DbState>,
    input: CreateChecklistItemInput,
) -> Result<ChecklistItem, AppError> {
    let title = input.title.trim().to_string();
    if title.is_empty() {
        return Err(AppError::Message("checklist item title cannot be empty".into()));
    }

    let card_exists: Option<String> =
        sqlx::query_scalar("SELECT id FROM cards WHERE id = ?")
            .bind(&input.card_id)
            .fetch_optional(&db.0)
            .await?;
    if card_exists.is_none() {
        return Err(AppError::NotFound(format!("card {}", input.card_id)));
    }

    let max_pos: Option<f64> = sqlx::query_scalar(
        "SELECT MAX(position) FROM checklist_items WHERE card_id = ?",
    )
    .bind(&input.card_id)
    .fetch_one(&db.0)
    .await?;

    let position = max_pos.map(|p| p + 1.0).unwrap_or(0.0);
    let id = new_id();
    let ts = now();

    sqlx::query(
        "INSERT INTO checklist_items (id, card_id, title, done, position, created_at) \
         VALUES (?, ?, ?, 0, ?, ?)",
    )
    .bind(&id)
    .bind(&input.card_id)
    .bind(&title)
    .bind(position)
    .bind(&ts)
    .execute(&db.0)
    .await?;

    touch_card_and_board(&db.0, &input.card_id).await?;

    Ok(ChecklistItem {
        id,
        card_id: input.card_id,
        title,
        done: false,
        position,
        created_at: ts,
    })
}

#[tauri::command]
pub async fn update_checklist_item(
    db: State<'_, DbState>,
    input: UpdateChecklistItemInput,
) -> Result<ChecklistItem, AppError> {
    let mut item = sqlx::query_as::<_, ChecklistItem>(
        "SELECT id, card_id, title, done, position, created_at \
         FROM checklist_items WHERE id = ?",
    )
    .bind(&input.id)
    .fetch_optional(&db.0)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("checklist item {}", input.id)))?;

    if let Some(title) = input.title {
        let t = title.trim().to_string();
        if t.is_empty() {
            return Err(AppError::Message("checklist item title cannot be empty".into()));
        }
        item.title = t;
    }
    if let Some(done) = input.done {
        item.done = done;
    }

    sqlx::query("UPDATE checklist_items SET title = ?, done = ? WHERE id = ?")
        .bind(&item.title)
        .bind(item.done)
        .bind(&item.id)
        .execute(&db.0)
        .await?;

    touch_card_and_board(&db.0, &item.card_id).await?;
    Ok(item)
}

#[tauri::command]
pub async fn delete_checklist_item(db: State<'_, DbState>, id: String) -> Result<(), AppError> {
    let card_id: Option<String> =
        sqlx::query_scalar("SELECT card_id FROM checklist_items WHERE id = ?")
            .bind(&id)
            .fetch_optional(&db.0)
            .await?;

    let Some(card_id) = card_id else {
        return Err(AppError::NotFound(format!("checklist item {id}")));
    };

    sqlx::query("DELETE FROM checklist_items WHERE id = ?")
        .bind(&id)
        .execute(&db.0)
        .await?;

    touch_card_and_board(&db.0, &card_id).await?;
    Ok(())
}

// ── Helpers ─────────────────────────────────────────────────────────

async fn touch_board(pool: &SqlitePool, board_id: &str) -> Result<(), AppError> {
    sqlx::query("UPDATE boards SET updated_at = ? WHERE id = ?")
        .bind(now())
        .bind(board_id)
        .execute(pool)
        .await?;
    Ok(())
}

async fn fetch_board_labels(pool: &SqlitePool, board_id: &str) -> Result<Vec<Label>, AppError> {
    let labels = sqlx::query_as::<_, Label>(
        "SELECT id, board_id, name, color, created_at FROM labels \
         WHERE board_id = ? ORDER BY name ASC",
    )
    .bind(board_id)
    .fetch_all(pool)
    .await?;
    Ok(labels)
}

async fn fetch_card_labels(pool: &SqlitePool, card_id: &str) -> Result<Vec<Label>, AppError> {
    let labels = sqlx::query_as::<_, Label>(
        "SELECT l.id, l.board_id, l.name, l.color, l.created_at \
         FROM labels l \
         INNER JOIN card_labels cl ON cl.label_id = l.id \
         WHERE cl.card_id = ? \
         ORDER BY l.name ASC",
    )
    .bind(card_id)
    .fetch_all(pool)
    .await?;
    Ok(labels)
}

async fn fetch_checklist(
    pool: &SqlitePool,
    card_id: &str,
) -> Result<Vec<ChecklistItem>, AppError> {
    let items = sqlx::query_as::<_, ChecklistItem>(
        "SELECT id, card_id, title, done, position, created_at \
         FROM checklist_items WHERE card_id = ? ORDER BY position ASC",
    )
    .bind(card_id)
    .fetch_all(pool)
    .await?;
    Ok(items)
}

async fn hydrate_card(pool: &SqlitePool, row: CardRow) -> Result<Card, AppError> {
    let labels = fetch_card_labels(pool, &row.id).await?;
    let checklist = fetch_checklist(pool, &row.id).await?;
    Ok(Card::from_row(row, labels, checklist))
}

async fn touch_card_and_board(pool: &SqlitePool, card_id: &str) -> Result<(), AppError> {
    let ts = now();
    sqlx::query("UPDATE cards SET updated_at = ? WHERE id = ?")
        .bind(&ts)
        .bind(card_id)
        .execute(pool)
        .await?;

    if let Some(board_id) = sqlx::query_scalar::<_, String>(
        "SELECT l.board_id FROM lists l \
         INNER JOIN cards c ON c.list_id = l.id \
         WHERE c.id = ?",
    )
    .bind(card_id)
    .fetch_optional(pool)
    .await?
    {
        touch_board(pool, &board_id).await?;
    }
    Ok(())
}
