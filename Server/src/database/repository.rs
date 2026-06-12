use sqlx::SqlitePool;

use crate::database::models::{Account, Character, NewCharacterInput, UserItem, UserSkill};

/// 账号仓库
pub struct AccountRepository {
    pool: SqlitePool,
}

impl AccountRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// 根据用户名查找账号
    pub async fn find_by_username(&self, username: &str) -> Result<Option<Account>, sqlx::Error> {
        sqlx::query_as::<_, Account>("SELECT * FROM accounts WHERE username = ?")
            .bind(username)
            .fetch_optional(&self.pool)
            .await
    }

    /// 创建新账号，返回新记录的 id
    pub async fn create(&self, username: &str, password_hash: &str) -> Result<i64, sqlx::Error> {
        let result = sqlx::query("INSERT INTO accounts (username, password) VALUES (?, ?)")
            .bind(username)
            .bind(password_hash)
            .execute(&self.pool)
            .await?;
        Ok(result.last_insert_rowid())
    }

    /// 修改密码
    pub async fn update_password(&self, account_id: i64, new_hash: &str) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(
            "UPDATE accounts SET password = ?, updated_at = datetime('now') WHERE id = ?",
        )
        .bind(new_hash)
        .bind(account_id)
        .execute(&self.pool)
        .await?;
        Ok(result.rows_affected() > 0)
    }
}

/// 角色仓库
pub struct CharacterRepository {
    pool: SqlitePool,
}

impl CharacterRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// 查找某账号的所有角色（未被删除的）
    pub async fn find_by_account(&self, account_id: i64) -> Result<Vec<Character>, sqlx::Error> {
        sqlx::query_as::<_, Character>(
            "SELECT * FROM characters WHERE account_id = ? AND deleted = 0 ORDER BY id",
        )
        .bind(account_id)
        .fetch_all(&self.pool)
        .await
    }

    /// 统计某账号的角色数量（未被删除的）
    pub async fn count_by_account(&self, account_id: i64) -> Result<i32, sqlx::Error> {
        let row: (i32,) = sqlx::query_as(
            "SELECT COUNT(*) FROM characters WHERE account_id = ? AND deleted = 0",
        )
        .bind(account_id)
        .fetch_one(&self.pool)
        .await?;
        Ok(row.0)
    }

    /// 创建新角色，返回新记录的 id
    pub async fn create(&self, input: &NewCharacterInput) -> Result<i64, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO characters (account_id, name, class, gender) VALUES (?, ?, ?, ?)",
        )
        .bind(input.account_id)
        .bind(&input.name)
        .bind(input.class)
        .bind(input.gender)
        .execute(&self.pool)
        .await?;
        Ok(result.last_insert_rowid())
    }

    /// 软删除角色
    pub async fn soft_delete(&self, char_id: i64, account_id: i64) -> Result<bool, sqlx::Error> {
        let result = sqlx::query(
            "UPDATE characters SET deleted = 1, updated_at = datetime('now') WHERE id = ? AND account_id = ?",
        )
        .bind(char_id)
        .bind(account_id)
        .execute(&self.pool)
        .await?;
        Ok(result.rows_affected() > 0)
    }

    /// 根据 id 查找角色
    pub async fn find_by_id(&self, char_id: i64) -> Result<Option<Character>, sqlx::Error> {
        sqlx::query_as::<_, Character>(
            "SELECT * FROM characters WHERE id = ? AND deleted = 0",
        )
        .bind(char_id)
        .fetch_optional(&self.pool)
        .await
    }
}

/// 背包仓库
pub struct InventoryRepository {
    pool: SqlitePool,
}

impl InventoryRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// 获取角色背包中的所有物品
    pub async fn get_inventory_items(&self, character_id: i64) -> Result<Vec<UserItem>, sqlx::Error> {
        sqlx::query_as::<_, UserItem>(
            "SELECT * FROM user_items WHERE character_id = ? ORDER BY slot",
        )
        .bind(character_id)
        .fetch_all(&self.pool)
        .await
    }

    /// 添加物品到背包，返回新记录的 id
    pub async fn add_inventory_item(
        &self,
        character_id: i64,
        item_id: i32,
        slot: i32,
        count: i32,
    ) -> Result<i64, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO user_items (character_id, item_id, slot, count) VALUES (?, ?, ?, ?)",
        )
        .bind(character_id)
        .bind(item_id)
        .bind(slot)
        .bind(count)
        .execute(&self.pool)
        .await?;
        Ok(result.last_insert_rowid())
    }

    /// 从背包移除物品
    pub async fn remove_inventory_item(&self, item_id: i64) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM user_items WHERE id = ?")
            .bind(item_id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    /// 更新物品槽位
    pub async fn update_item_slot(&self, item_id: i64, new_slot: i32) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("UPDATE user_items SET slot = ? WHERE id = ?")
            .bind(new_slot)
            .bind(item_id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    /// 清空角色背包
    pub async fn clear_inventory(&self, character_id: i64) -> Result<u64, sqlx::Error> {
        let result = sqlx::query("DELETE FROM user_items WHERE character_id = ?")
            .bind(character_id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected())
    }
}

// ============================================
// SkillRepository
// ============================================

/// 技能仓库
pub struct SkillRepository {
    pool: SqlitePool,
}

impl SkillRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_skill(&self, skill: &UserSkill) -> Result<UserSkill, sqlx::Error> {
        let result = sqlx::query(
            "INSERT INTO user_skills (character_id, spell_id, level, proficiency) VALUES (?, ?, ?, ?)"
        )
        .bind(skill.character_id)
        .bind(skill.spell_id)
        .bind(skill.level)
        .bind(skill.proficiency)
        .execute(&self.pool)
        .await?;

        Ok(UserSkill {
            id: result.last_insert_rowid(),
            character_id: skill.character_id,
            spell_id: skill.spell_id,
            level: skill.level,
            proficiency: skill.proficiency,
        })
    }

    pub async fn get_skills_by_character(&self, character_id: i64) -> Result<Vec<UserSkill>, sqlx::Error> {
        sqlx::query_as::<_, UserSkill>(
            "SELECT * FROM user_skills WHERE character_id = ?"
        )
        .bind(character_id)
        .fetch_all(&self.pool)
        .await
    }

    pub async fn update_proficiency(&self, skill_id: i64, level: i32, proficiency: i32) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE user_skills SET level = ?, proficiency = ? WHERE id = ?"
        )
        .bind(level)
        .bind(proficiency)
        .bind(skill_id)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    pub async fn delete_skill(&self, skill_id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM user_skills WHERE id = ?")
            .bind(skill_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

// ============================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_helpers::{setup_test_db, teardown, create_test_account, create_test_character};

    // ============================================
    // SkillRepository CRUD 测试
    // ============================================

    /// 插入一条 UserSkill 记录，验证 id 被正确赋值
    #[tokio::test]
    async fn test_create_user_skill() {
        let (pool, path) = setup_test_db("skill_create").await;
        let acc_repo = AccountRepository::new(pool.clone());
        let char_repo = CharacterRepository::new(pool.clone());
        let skill_repo = SkillRepository::new(pool.clone());

        let account_id = create_test_account(&acc_repo).await;
        let char_id = create_test_character(&char_repo, account_id, "SkillHero").await;

        let skill = UserSkill {
            id: 0,
            character_id: char_id,
            spell_id: 101,
            level: 1,
            proficiency: 0,
        };

        let result = skill_repo.create_skill(&skill).await.unwrap();
        assert!(result.id > 0, "创建后应自动分配 ID");
        assert_eq!(result.character_id, char_id);
        assert_eq!(result.spell_id, 101);
        assert_eq!(result.level, 1);
        assert_eq!(result.proficiency, 0);

        teardown(pool, &path).await;
    }

    /// 为一个角色插入多个技能，验证 get_skills_by_character 返回正确数量和内容
    #[tokio::test]
    async fn test_get_user_skills() {
        let (pool, path) = setup_test_db("skill_get").await;
        let acc_repo = AccountRepository::new(pool.clone());
        let char_repo = CharacterRepository::new(pool.clone());
        let skill_repo = SkillRepository::new(pool.clone());

        let account_id = create_test_account(&acc_repo).await;
        let char_id = create_test_character(&char_repo, account_id, "SkillMage").await;

        // 插入多个技能
        let test_skills = [(101i32, 1i32), (102, 2), (103, 3)];
        for (spell_id, level) in &test_skills {
            let skill = UserSkill {
                id: 0,
                character_id: char_id,
                spell_id: *spell_id,
                level: *level,
                proficiency: 0,
            };
            skill_repo.create_skill(&skill).await.unwrap();
        }

        let skills = skill_repo.get_skills_by_character(char_id).await.unwrap();
        assert_eq!(skills.len(), 3, "应返回 3 个技能");

        // 验证技能数据正确
        let spell_ids: Vec<i32> = skills.iter().map(|s| s.spell_id).collect();
        assert!(spell_ids.contains(&101));
        assert!(spell_ids.contains(&102));
        assert!(spell_ids.contains(&103));

        // 查询不存在角色的技能
        let empty = skill_repo.get_skills_by_character(99999).await.unwrap();
        assert!(empty.is_empty(), "不存在角色的技能列表应为空");

        teardown(pool, &path).await;
    }

    /// 更新熟练度，验证更新成功
    #[tokio::test]
    async fn test_update_skill_proficiency() {
        let (pool, path) = setup_test_db("skill_upd").await;
        let acc_repo = AccountRepository::new(pool.clone());
        let char_repo = CharacterRepository::new(pool.clone());
        let skill_repo = SkillRepository::new(pool.clone());

        let account_id = create_test_account(&acc_repo).await;
        let char_id = create_test_character(&char_repo, account_id, "SkillPro").await;

        let skill = UserSkill {
            id: 0,
            character_id: char_id,
            spell_id: 101,
            level: 1,
            proficiency: 100,
        };
        let created = skill_repo.create_skill(&skill).await.unwrap();

        // 更新熟练度和等级
        skill_repo.update_proficiency(created.id, 2, 500).await.unwrap();

        let skills = skill_repo.get_skills_by_character(char_id).await.unwrap();
        let updated = skills.iter().find(|s| s.spell_id == 101).unwrap();
        assert_eq!(updated.level, 2, "等级应更新为 2");
        assert_eq!(updated.proficiency, 500, "熟练度应更新为 500");

        teardown(pool, &path).await;
    }

    /// 删除技能记录，验证删除后查询为空
    #[tokio::test]
    async fn test_delete_user_skill() {
        let (pool, path) = setup_test_db("skill_del").await;
        let acc_repo = AccountRepository::new(pool.clone());
        let char_repo = CharacterRepository::new(pool.clone());
        let skill_repo = SkillRepository::new(pool.clone());

        let account_id = create_test_account(&acc_repo).await;
        let char_id = create_test_character(&char_repo, account_id, "SkillDel").await;

        let skill = UserSkill {
            id: 0,
            character_id: char_id,
            spell_id: 101,
            level: 1,
            proficiency: 0,
        };
        let created = skill_repo.create_skill(&skill).await.unwrap();

        // 删除技能
        skill_repo.delete_skill(created.id).await.unwrap();

        let skills = skill_repo.get_skills_by_character(char_id).await.unwrap();
        assert!(skills.is_empty(), "删除后技能列表应为空");

        teardown(pool, &path).await;
    }
}
