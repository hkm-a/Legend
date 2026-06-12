/// 游戏世界状态 — 全局游戏循环驱动
use std::sync::Arc;

use tokio::sync::RwLock;
use tokio::time::interval;

use crate::item::ItemManager;
use crate::map::MapManager;
use crate::monster::MonsterManager;
use crate::network::session_manager::SessionManager;
use crate::npc::NpcManager;

use mir2_shared::packets::server::HealthChangedPacket;
use mir2_shared::packets::Packet;

pub struct WorldState {
    pub map_manager: Arc<MapManager>,
    pub monster_manager: Arc<RwLock<MonsterManager>>,
    pub item_manager: Arc<RwLock<ItemManager>>,
    pub npc_manager: Arc<NpcManager>,
    pub session_manager: Arc<SessionManager>,
    pub tick_rate_ms: u64,
}

impl WorldState {
    pub fn new(
        map_manager: MapManager,
        monster_manager: MonsterManager,
        item_manager: ItemManager,
        npc_manager: NpcManager,
        session_manager: Arc<SessionManager>,
        tick_rate_ms: u64,
    ) -> Self {
        Self {
            map_manager: Arc::new(map_manager),
            monster_manager: Arc::new(RwLock::new(monster_manager)),
            item_manager: Arc::new(RwLock::new(item_manager)),
            npc_manager: Arc::new(npc_manager),
            session_manager,
            tick_rate_ms,
        }
    }

    /// 启动全局 tick 循环
    pub async fn start_tick_loop(&self) {
        let monster_manager = self.monster_manager.clone();
        let item_manager = self.item_manager.clone();
        let map_manager = self.map_manager.clone();

        // 预热：所有地图初始刷怪
        {
            let mut mm = monster_manager.write().await;
            for map_id in 0..=65535u16 {
                if map_manager.get_map(map_id).is_some() {
                    let configs = {
                        let map = map_manager.get_map(map_id).unwrap();
                        map.spawn_configs.monsters.clone()
                    };
                    if !configs.is_empty() {
                        mm.assign_map_spawns(map_id, configs);
                        mm.spawn_all(map_id);
                        tracing::info!(
                            "Warmup: spawned monsters on map {}",
                            map_id
                        );
                    }
                }
            }
        }

        let mut ticker = interval(std::time::Duration::from_millis(
            self.tick_rate_ms,
        ));

        let mut tick_count: u64 = 0;

        loop {
            ticker.tick().await;
            tick_count += 1;

            // 怪物 AI tick
            if let Ok(mut mm) = monster_manager.try_write() {
                mm.tick().await;
            }

            // 清理过期地面物品
            if let Ok(mut im) = item_manager.try_write() {
                im.clean_expired_ground_items();
            }

            // 安全区回血（每 10 tick 检查一次）
            if tick_count % 10 == 0 {
                let sm = self.session_manager.clone();
                let mm = self.map_manager.clone();
                tokio::spawn(async move {
                    let states = sm.get_all_states().await;
                    for (sid, state) in states {
                        if let Some(loc) = Some(state.location) {
                            if mm.is_safe_zone(state.map_id as u16, loc.0, loc.1) {
                                let hp_gain = (state.max_hp as f64 * 0.02).max(1.0) as u32;
                                let mp_gain = (state.max_mp as f64 * 0.02).max(1.0) as u32;

                                let new_hp = (state.current_hp + hp_gain).min(state.max_hp);
                                let new_mp = (state.current_mp + mp_gain).min(state.max_mp);

                                if new_hp != state.current_hp || new_mp != state.current_mp {
                                    sm.update_state(sid, |s| {
                                        s.current_hp = new_hp;
                                        s.current_mp = new_mp;
                                    })
                                    .await;

                                    let hp_packet = HealthChangedPacket::new(new_hp, state.max_hp);
                                    if let Ok(data) = hp_packet.encode() {
                                        sm.send_to_session(sid, &data).await;
                                    }
                                }
                            }
                        }
                    }
                });
            }
        }
    }
}
