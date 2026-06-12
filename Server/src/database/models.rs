use sqlx::FromRow;

/// 账号数据库模型
#[derive(Debug, Clone, FromRow)]
pub struct Account {
    pub id: i64,
    pub username: String,
    pub password: String,
    pub email: String,
    pub banned: i32,
    pub ban_reason: String,
    pub created_at: String,
    pub updated_at: String,
}

/// 角色数据库模型
#[derive(Debug, Clone, FromRow)]
pub struct Character {
    pub id: i64,
    pub account_id: i64,
    pub name: String,
    pub class: i32,
    pub gender: i32,
    pub level: i32,
    pub experience: i64,
    pub map_id: i32,
    pub location_x: i32,
    pub location_y: i32,
    pub direction: i32,
    pub hp: i32,
    pub mp: i32,
    pub gold: i64,
    pub deleted: i32,
    pub created_at: String,
    pub updated_at: String,
}

/// 用于网络传输的角色信息（非DB映射）
#[derive(Debug, Clone)]
pub struct CharacterInfo {
    pub index: u32,
    pub name: String,
    pub class: u8,
    pub gender: u8,
    pub level: u16,
    pub hp: u32,
    pub mp: u32,
    pub max_hp: u32,
    pub max_mp: u32,
}

/// 创建角色入参
#[derive(Debug, Clone)]
pub struct NewCharacterInput {
    pub account_id: i64,
    pub name: String,
    pub class: i32,
    pub gender: i32,
}

/// 用户背包物品
#[derive(Debug, Clone, FromRow)]
pub struct UserItem {
    pub id: i64,
    pub character_id: i64,
    pub item_id: i32,
    pub slot: i32,
    pub count: i32,
    pub durability: i32,
    pub max_durability: i32,
}

/// 用户技能数据库模型
#[derive(Debug, Clone, FromRow)]
pub struct UserSkill {
    pub id: i64,
    pub character_id: i64,
    pub spell_id: i32,
    pub level: i32,
    pub proficiency: i32,
}

/// Character 转 CharacterInfo
impl Character {
    pub fn to_char_info(&self, index: u32) -> CharacterInfo {
        CharacterInfo {
            index,
            name: self.name.clone(),
            class: self.class as u8,
            gender: self.gender as u8,
            level: self.level as u16,
            hp: self.hp as u32,
            mp: self.mp as u32,
            max_hp: self.hp as u32,
            max_mp: self.mp as u32,
        }
    }
}
