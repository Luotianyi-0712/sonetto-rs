use sonettobuf;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct UserCommandPostInfo {
    pub user_id: i64,
    pub paper: i32,
    pub catch_num: i32,
}

#[derive(Debug, Clone)]
pub struct CommandPostEventInfo {
    pub event_id: i32,
    pub state: i32,
    pub hero_ids: Vec<i32>,
    pub start_time: u64,
    pub end_time: u64,
    pub is_read: bool,
}

impl From<CommandPostEventInfo> for sonettobuf::CommandPostEventInfo {
    fn from(e: CommandPostEventInfo) -> Self {
        sonettobuf::CommandPostEventInfo {
            id: Some(e.event_id),
            state: Some(e.state),
            hero_ids: e.hero_ids,
            start_time: Some(e.start_time),
            end_time: Some(e.end_time),
            read: Some(e.is_read),
        }
    }
}

#[derive(Debug, Clone, FromRow)]
pub struct CommandPostTask {
    pub task_id: i32,
    pub progress: i32,
    pub has_finished: bool,
    pub finish_count: i32,
    pub task_type: i32,
    pub expiry_time: i32,
}

impl From<CommandPostTask> for sonettobuf::Task {
    fn from(t: CommandPostTask) -> Self {
        sonettobuf::Task {
            id: t.task_id,
            progress: t.progress,
            has_finished: t.has_finished,
            finish_count: Some(t.finish_count),
            r#type: Some(t.task_type),
            expiry_time: Some(t.expiry_time),
        }
    }
}
