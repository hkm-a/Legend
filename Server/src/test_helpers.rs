/// 测试辅助函数 — 仅在测试模式下编译

use crate::database::models::NewCharacterInput;
use crate::database::repository::{AccountRepository, CharacterRepository};
use sqlx::sqlite::SqlitePool;
use crate::database::init_db;

/// 创建测试用数据库，返回 (pool, path)
pub async fn setup_test_db(name: &str) -> (SqlitePool, String) {
    let path = format!("./data/test_{}.db", name);
    let _ = std::fs::remove_file(&path);
    let pool = init_db(&path).await.unwrap();
    (pool, path)
}

/// 清理测试数据库文件
pub async fn teardown(pool: SqlitePool, path: &str) {
    pool.close().await;
    let _ = std::fs::remove_file(path);
}

/// 创建测试账号，返回 account_id
pub async fn create_test_account(repo: &AccountRepository) -> i64 {
    repo.create("test_user", "$argon2hash_v1").await.unwrap()
}

/// 创建测试角色，返回 character_id
pub async fn create_test_character(
    char_repo: &CharacterRepository,
    account_id: i64,
    name: &str,
) -> i64 {
    let input = NewCharacterInput {
        account_id,
        name: name.to_string(),
        class: 0,
        gender: 0,
    };
    char_repo.create(&input).await.unwrap()
}
