pub mod ai;
pub mod spawner;

use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Instant;

use mir2_shared::enums::MirDirection;
use mir2_shared::types::StatRange;
use serde::Deserialize;

use crate::map::{MapManager, MonsterSpawnConfig};

/// 全局怪物 ID 生成器
static NEXT_MONSTER_ID: AtomicU32 = AtomicU32::new(10000);

/// 怪物状态
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MonsterState {
    Idle,
    Patrol,
    Chase,
    Attack,
    Dead,
}

/// 怪物模板（从 JSON 加载）
#[derive(Debug, Clone, Deserialize)]
pub struct MonsterTemplate {
    pub id: u16,
    pub name: String,
    pub level: u16,
    pub max_hp: u32,
    pub max_mp: u32,
    pub dc: StatRange,
    pub ac: u32,
    pub mac: u32,
    pub accuracy: u32,
    pub agility: u32,
    pub exp: u32,
    pub attack_speed: u64, // ms between attacks
    pub move_speed: u64,   // ms between moves
    pub image: u16,
}

/// 怪物实例（运行时）
#[derive(Debug)]
pub struct Monster {
    pub object_id: u32,
    pub template_id: u16,
    pub template: Arc<MonsterTemplate>,
    pub current_hp: i32,
    pub current_mp: i32,
    pub location: (i32, i32),
    pub direction: MirDirection,
    pub spawn_point: (i32, i32),
    pub state: MonsterState,
    pub target_id: Option<u32>,
    pub last_attack_time: Instant,
    pub last_move_time: Instant,
    pub map_id: u16,
    pub is_alive: bool,
}

impl Monster {
    pub fn new(template: Arc<MonsterTemplate>, map_id: u16, x: i32, y: i32) -> Self {
        let object_id = NEXT_MONSTER_ID.fetch_add(1, Ordering::SeqCst);
        Self {
            object_id,
            template_id: template.id,
            template,
            current_hp: 0, // will be set to max_hp after creation
            current_mp: 0,
            location: (x, y),
            direction: MirDirection::Down,
            spawn_point: (x, y),
            state: MonsterState::Idle,
            target_id: None,
            last_attack_time: Instant::now(),
            last_move_time: Instant::now(),
            map_id,
            is_alive: true,
        }
    }

    /// 受伤，返回是否死亡
    pub fn take_damage(&mut self, amount: i32) -> bool {
        if !self.is_alive {
            return false;
        }
        self.current_hp = self.current_hp.saturating_sub(amount);
        if self.current_hp <= 0 {
            self.current_hp = 0;
            self.is_alive = false;
            self.state = MonsterState::Dead;
            true
        } else {
            false
        }
    }

    /// 曼哈顿距离
    pub fn distance_to(&self, x: i32, y: i32) -> i32 {
        (self.location.0 - x).abs() + (self.location.1 - y).abs()
    }
}

/// 怪物管理器
pub struct MonsterManager {
    pub monsters: HashMap<u32, Monster>,
    pub templates: HashMap<u16, MonsterTemplate>,
    pub monsters_by_map: HashMap<u16, Vec<u32>>,
    pub spawn_configs: HashMap<u16, Vec<MonsterSpawnConfig>>,
    pub map_manager: Arc<MapManager>,
    /// 每张地图的刷怪计时（上次刷新 tick）
    last_spawn_time: HashMap<u16, Instant>,
}

impl MonsterManager {
    pub fn new(map_manager: Arc<MapManager>) -> Self {
        Self {
            monsters: HashMap::new(),
            templates: HashMap::new(),
            monsters_by_map: HashMap::new(),
            spawn_configs: HashMap::new(),
            map_manager,
            last_spawn_time: HashMap::new(),
        }
    }

    /// 从 JSON 加载怪物模板
    pub async fn load_templates(path: &str) -> Result<Vec<MonsterTemplate>, anyhow::Error> {
        let content = tokio::fs::read_to_string(path).await?;
        let templates: Vec<MonsterTemplate> = serde_json::from_str(&content)?;
        Ok(templates)
    }

    /// 初始化模板（批量加载后调用）
    pub fn init_templates(&mut self, templates: Vec<MonsterTemplate>) {
        for t in templates {
            self.templates.insert(t.id, t);
        }
        tracing::info!("MonsterManager loaded {} templates", self.templates.len());
    }

    /// 分配地图的刷怪配置
    pub fn assign_map_spawns(&mut self, map_id: u16, configs: Vec<MonsterSpawnConfig>) {
        self.spawn_configs.insert(map_id, configs);
        // 初始化刷新计时
        self.last_spawn_time.insert(map_id, Instant::now());
    }

    /// 在地图上生成一只怪物
    pub fn spawn_monster(&mut self, map_id: u16, spawn: &MonsterSpawnConfig) -> Option<u32> {
        let template = self.templates.get(&spawn.monster_id)?;

        // 在配置区域内查找可行走空地
        let (x, y) = self.find_spawn_position(map_id, spawn)?;

        let mut monster = Monster::new(Arc::new(template.clone()), map_id, x, y);
        monster.current_hp = template.max_hp as i32;
        monster.current_mp = template.max_mp as i32;

        let object_id = monster.object_id;
        self.monsters.insert(object_id, monster);
        self.monsters_by_map.entry(map_id).or_default().push(object_id);

        Some(object_id)
    }

    /// 查找刷怪位置
    fn find_spawn_position(&self, map_id: u16, spawn: &MonsterSpawnConfig) -> Option<(i32, i32)> {
        let map = self.map_manager.get_map(map_id)?;
        let range = spawn.range.max(1) as i32;

        // 在配置区域随机尝试若干次
        for _ in 0..20 {
            let rx = if spawn.w > 0 {
                spawn.x + fastrand::i32(0..spawn.w)
            } else {
                spawn.x
            };
            let ry = if spawn.h > 0 {
                spawn.y + fastrand::i32(0..spawn.h)
            } else {
                spawn.y
            };

            // 检查该格是否可行走且不在其他怪物位置上
            if map.is_walkable_at(rx, ry) && !self.is_occupied(map_id, rx, ry) {
                return Some((rx, ry));
            }
        }

        // 回退：尝试在 spawn 点附近找
        for dx in -range..=range {
            for dy in -range..=range {
                let cx = spawn.x + dx;
                let cy = spawn.y + dy;
                if map.is_walkable_at(cx, cy) && !self.is_occupied(map_id, cx, cy) {
                    return Some((cx, cy));
                }
            }
        }

        None
    }

    /// 检查某格是否已被怪物占据
    fn is_occupied(&self, map_id: u16, x: i32, y: i32) -> bool {
        if let Some(ids) = self.monsters_by_map.get(&map_id) {
            for &id in ids {
                if let Some(monster) = self.monsters.get(&id) {
                    if monster.location == (x, y) && monster.is_alive {
                        return true;
                    }
                }
            }
        }
        false
    }

    /// 为地图初始生成所有配置的怪物
    pub fn spawn_all(&mut self, map_id: u16) {
        let configs = match self.spawn_configs.get(&map_id) {
            Some(c) => c.clone(),
            None => return,
        };

        for spawn in &configs {
            let _template = match self.templates.get(&spawn.monster_id) {
                Some(t) => t,
                None => continue,
            };

            let spawned_count = self
                .monsters
                .values()
                .filter(|m| m.map_id == map_id && m.template_id == spawn.monster_id && m.is_alive)
                .count() as u8;

            let to_spawn = spawn.count.saturating_sub(spawned_count);
            for _ in 0..to_spawn {
                self.spawn_monster(map_id, spawn);
            }
        }
    }

    pub fn get_monster(&self, id: u32) -> Option<&Monster> {
        self.monsters.get(&id)
    }

    pub fn get_monster_mut(&mut self, id: u32) -> Option<&mut Monster> {
        self.monsters.get_mut(&id)
    }

    pub fn get_monsters_on_map(&self, map_id: u16) -> Vec<&Monster> {
        if let Some(ids) = self.monsters_by_map.get(&map_id) {
            ids.iter()
                .filter_map(|id| self.monsters.get(id))
                .filter(|m| m.is_alive)
                .collect()
        } else {
            Vec::new()
        }
    }

    pub fn remove_monster(&mut self, id: u32) {
        if let Some(monster) = self.monsters.remove(&id) {
            let map_id = monster.map_id;
            if let Some(ids) = self.monsters_by_map.get_mut(&map_id) {
                ids.retain(|&i| i != id);
            }
        }
    }

    /// 获取指定位置附近的怪物 ID 列表
    pub fn get_monsters_near(&self, map_id: u16, x: i32, y: i32, range: i32) -> Vec<u32> {
        if let Some(ids) = self.monsters_by_map.get(&map_id) {
            ids.iter()
                .filter_map(|id| self.monsters.get(id))
                .filter(|m| m.is_alive && m.distance_to(x, y) <= range)
                .map(|m| m.object_id)
                .collect()
        } else {
            Vec::new()
        }
    }

    /// 获取某地图活着的怪物数量
    pub fn alive_count_on_map(&self, map_id: u16, template_id: u16) -> usize {
        if let Some(ids) = self.monsters_by_map.get(&map_id) {
            ids.iter()
                .filter_map(|id| self.monsters.get(id))
                .filter(|m| m.is_alive && m.template_id == template_id)
                .count()
        } else {
            0
        }
    }

    /// 全局 tick — 驱动所有怪物 AI 和刷新
    pub async fn tick(&mut self) {
        // 1. 驱怪 AI（在每个地图上处理）
        let map_ids: Vec<u16> = self.monsters_by_map.keys().copied().collect();
        for map_id in &map_ids {
            // 收集活怪 ID
            let live_ids: Vec<u32> = {
                let ids = self.monsters_by_map.get(map_id);
                match ids {
                    Some(ids) => ids
                        .iter()
                        .filter_map(|id| {
                            let m = self.monsters.get(id)?;
                            if m.is_alive { Some(*id) } else { None }
                        })
                        .collect(),
                    None => continue,
                }
            };

            // AI tick — 使用 map_manager 引用
            for &id in &live_ids {
                if let Some(monster) = self.monsters.get_mut(&id) {
                    ai::tick_monster_simple(monster);
                }
            }

            // 清理死亡怪物（延迟移除）
            // 暂不移除，由外部显式调用 remove_dead
        }

        // 2. 刷新检查
        for map_id in &map_ids {
            self.check_respawns(*map_id);
        }
    }

    /// 检查是否需要刷新怪物
    fn check_respawns(&mut self, map_id: u16) {
        let configs = match self.spawn_configs.get(&map_id) {
            Some(c) => c.clone(),
            None => return,
        };
        let now = Instant::now();

        for spawn in &configs {
            let last_time = self.last_spawn_time.entry(map_id).or_insert_with(Instant::now);
            let elapsed = now.duration_since(*last_time).as_millis() as u64;

            // 先检查是否还有空位
            let alive = self.alive_count_on_map(map_id, spawn.monster_id) as u8;
            if alive >= spawn.count {
                continue;
            }

            // 检查间隔
            if elapsed < spawn.interval {
                continue;
            }

            // 重生
            let to_spawn = spawn.count.saturating_sub(alive);
            for _ in 0..to_spawn {
                self.spawn_monster(map_id, spawn);
            }
            *self.last_spawn_time.get_mut(&map_id).unwrap() = now;
        }
    }

    /// 移除所有死亡怪物
    pub fn remove_dead_monsters(&mut self) {
        let dead_ids: Vec<u32> = self
            .monsters
            .iter()
            .filter(|(_, m)| !m.is_alive)
            .map(|(&id, _)| id)
            .collect();

        for id in dead_ids {
            self.remove_monster(id);
        }
    }
}

impl Default for MonsterManager {
    fn default() -> Self {
        Self::new(Arc::new(MapManager::new()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 验证 monsters.json 文件能被正确加载
    #[test]
    fn test_load_monsters_json_file() {
        let content = std::fs::read_to_string("data/monsters.json")
            .expect("monsters.json 文件应存在");
        let monsters: Vec<MonsterTemplate> = serde_json::from_str(&content)
            .expect("monsters.json 应能被反序列化为 Vec<MonsterTemplate>");

        // 应有 8 种怪物 (0-7)
        assert_eq!(monsters.len(), 8, "monsters.json 应有 8 种怪物");

        // 验证怪物 4 (骷髅)
        let skel = monsters.iter().find(|m| m.id == 4).expect("怪物 4 (骷髅) 应存在");
        assert_eq!(skel.name, "骷髅");
        assert_eq!(skel.level, 8);
        assert_eq!(skel.max_hp, 120);
        assert_eq!(skel.dc.min, 8);
        assert_eq!(skel.dc.max, 15);
        assert_eq!(skel.image, 4);
    }

    /// 验证 NPC JSON 文件能被正确加载
    #[test]
    fn test_load_npcs_json_file() {
        use serde::Deserialize;

        #[derive(Debug, Clone, Deserialize)]
        struct NpcTemplate {
            id: u16,
            name: String,
            map_id: u16,
            x: u16,
            y: u16,
            image: u16,
            shop_type: String,
            selling_items: Vec<u16>,
        }

        let content = std::fs::read_to_string("data/npcs.json")
            .expect("npcs.json 文件应存在");
        let npcs: Vec<NpcTemplate> = serde_json::from_str(&content)
            .expect("npcs.json 应能被反序列化");
        assert_eq!(npcs.len(), 4, "npcs.json 应有 4 个 NPC");
        assert_eq!(npcs[0].name, "药店老板");
        assert_eq!(npcs[1].name, "武器店老板");
    }

    /// 验证 skills.json 文件能被正确加载
    #[test]
    fn test_load_skills_json_file() {
        use serde::Deserialize;

        #[derive(Debug, Clone, Deserialize)]
        struct SkillTemplate {
            id: u16,
            name: String,
            spell: String,
            required_class: i32,
            required_level: u16,
            is_passive: bool,
        }

        let content = std::fs::read_to_string("data/skills.json")
            .expect("skills.json 文件应存在");
        let skills: Vec<SkillTemplate> = serde_json::from_str(&content)
            .expect("skills.json 应能被反序列化");
        assert_eq!(skills.len(), 3, "skills.json 应有 3 个技能");
        assert_eq!(skills[0].name, "基本剑术");
        assert_eq!(skills[0].required_class, 0);
        assert_eq!(skills[0].is_passive, true);
        assert_eq!(skills[1].name, "火球术");
        assert_eq!(skills[1].required_class, 1);
        assert_eq!(skills[2].name, "治愈术");
        assert_eq!(skills[2].required_class, 2);
    }

    /// 验证 drops.json 文件格式与 DropRule 兼容
    #[test]
    fn test_drops_json_format_compatible() {
        let content = std::fs::read_to_string("data/drops.json")
            .expect("drops.json 文件应存在");

        // 尝试用 Vec<DropRule> 格式解析（扁平数组，应成功）
        #[derive(Debug, Clone, Deserialize)]
        struct DropRule {
            monster_id: u16,
            item_id: u16,
            probability: f64,
            count: u32,
        }
        let result: Result<Vec<DropRule>, _> = serde_json::from_str(&content);
        let rules = result.expect("drops.json 应兼容 Vec<DropRule> 扁平数组格式");

        // 验证包含预期的掉落规则
        assert!(!rules.is_empty(), "drops.json 不应为空");
        assert_eq!(rules[0].monster_id, 4, "第一个掉落规则应为 monster 4");
        assert_eq!(rules[6].item_id, 0, "最后一个掉落规则应为 item 0");
    }

    /// 验证地图数据文件非空
    #[test]
    fn test_map_files_have_tiles() {
        use serde::Deserialize;

        #[derive(Debug, Clone, Deserialize)]
        struct MapLayer {
            data: Vec<u16>,
            width: u16,
            height: u16,
        }
        #[derive(Debug, Clone, Deserialize)]
        struct MapFile {
            width: u16,
            height: u16,
            layers: Vec<MapLayer>,
        }

        // 验证地图 2
        let content2 = std::fs::read_to_string("data/maps/map_2.json")
            .expect("map_2.json 应存在");
        let map2: MapFile = serde_json::from_str(&content2).unwrap();
        assert_eq!(map2.width, 30);
        assert_eq!(map2.height, 30);
        assert!(!map2.layers.is_empty(), "map_2 应有数据层");
        assert_eq!(map2.layers[0].data.len(), 900, "map_2 应有 30×30=900 个 tile");

        // 验证地图 3
        let content3 = std::fs::read_to_string("data/maps/map_3.json")
            .expect("map_3.json 应存在");
        let map3: MapFile = serde_json::from_str(&content3).unwrap();
        assert_eq!(map3.width, 40);
        assert_eq!(map3.height, 40);
        assert!(!map3.layers.is_empty(), "map_3 应有数据层");
        assert_eq!(map3.layers[0].data.len(), 1600, "map_3 应有 40×40=1600 个 tile");
    }

    /// 验证地图连接一致性
    #[test]
    fn test_map_connections_consistency() {
        use serde::Deserialize;

        #[derive(Debug, Clone, Deserialize)]
        struct MapProperty {
            name: String,
            value: serde_json::Value,
        }
        #[derive(Debug, Clone, Deserialize)]
        struct MapFile {
            properties: Vec<MapProperty>,
        }
        #[derive(Debug, Clone, Deserialize)]
        struct Connection {
            sx: u16,
            sy: u16,
            tm: u16,
            tx: u16,
            ty: u16,
        }

        fn load_connections(path: &str) -> Vec<Connection> {
            let content = std::fs::read_to_string(path).unwrap();
            let map: MapFile = serde_json::from_str(&content).unwrap();
            let conn_str = map.properties.iter()
                .find(|p| p.name == "connections")
                .map(|p| p.value.as_str().unwrap().to_string())
                .unwrap();
            serde_json::from_str(&conn_str).unwrap()
        }

        let conn2 = load_connections("data/maps/map_2.json");
        let conn3 = load_connections("data/maps/map_3.json");

        // Map 2 → Map 3: sx=25,sy=25 → tm=3,tx=10,ty=38
        let m2_to_m3 = conn2.iter().find(|c| c.tm == 3).unwrap();
        assert_eq!(m2_to_m3.sx, 25, "map_2→map_3 入口 sx 应为 25");
        assert_eq!(m2_to_m3.sy, 25, "map_2→map_3 入口 sy 应为 25");
        assert_eq!(m2_to_m3.tx, 10, "map_2→map_3 目标 tx 应为 10");
        assert_eq!(m2_to_m3.ty, 38, "map_2→map_3 目标 ty 应为 38");

        // Map 3 → Map 2: sx=10,sy=38 → tm=2,tx=25,ty=25 (回环一致)
        let m3_to_m2 = conn3.iter().find(|c| c.tm == 2).unwrap();
        assert_eq!(m3_to_m2.sx, 10, "map_3→map_2 入口 sx 应为 10");
        assert_eq!(m3_to_m2.sy, 38, "map_3→map_2 入口 sy 应为 38");
        assert_eq!(m3_to_m2.tx, 25, "map_3→map_2 目标 tx 应为 25");
        assert_eq!(m3_to_m2.ty, 25, "map_3→map_2 目标 ty 应为 25");

        // Map 2 → Map 1: sx=2,sy=2 → tm=1,tx=48,ty=25
        let m2_to_m1 = conn2.iter().find(|c| c.tm == 1).unwrap();
        assert_eq!(m2_to_m1.sx, 2, "map_2→map_1 入口 sx 应为 2");
        assert_eq!(m2_to_m1.sy, 2, "map_2→map_1 入口 sy 应为 2");
        assert_eq!(m2_to_m1.tx, 48, "map_2→map_1 目标 tx 应为 48");
        assert_eq!(m2_to_m1.ty, 25, "map_2→map_1 目标 ty 应为 25");

        // 验证 map_1 有返回连接至 map_2
        let conn1 = load_connections("data/maps/map_1.json");
        let m1_to_m2 = conn1.iter().find(|c| c.tm == 2);
        assert!(
            m1_to_m2.is_some(),
            "map_1 应有返回连接至 map_2 — 目前缺失"
        );
    }
}
