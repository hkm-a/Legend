use std::sync::Arc;

use mir2_shared::enums::{ChatType, MirDirection};
use mir2_shared::net::error::NetError;
use mir2_shared::net::packet_id::ClientOpcode;
use mir2_shared::packets::server::{
    DamageIndicatorPacket, GameMessagePacket, GainedItemPacket, MapChangedPacket,
    MapInformationPacket, NPCGoodsItem, NPCGoodsPacket, NPCResponsePacket, ObjectAttackPacket,
    ObjectPlayerPacket, ObjectRemovePacket, ObjectStruckPacket, ObjectTurnPacket, ObjectWalkPacket,
    ServerChatPacket, UserLocationPacket,
};
use mir2_shared::packets::Packet;
use mir2_shared::types::Point;

use super::session_manager::SessionManager;
use crate::combat::CombatSystem;
use crate::game::WorldState;

/// 包路由器 — 注册表模式分发
///
/// 以 HashMap 存储 packet_id → handler 的映射，
/// 避免巨型 match 表达式，方便扩展。
pub struct PacketRouter {
    handlers: std::sync::Mutex<
        std::collections::HashMap<
            u16,
            Box<dyn Fn(u32, &[u8]) + Send + Sync>,
        >,
    >,
}

impl PacketRouter {
    pub fn new() -> Self {
        Self {
            handlers: std::sync::Mutex::new(std::collections::HashMap::new()),
        }
    }

    /// 注册 handler：packet_id → 处理函数
    pub fn register<F>(&self, packet_id: u16, handler: F)
    where
        F: Fn(u32, &[u8]) + Send + Sync + 'static,
    {
        let mut handlers = self.handlers.lock().expect("PacketRouter lock poisoned");
        handlers.insert(packet_id, Box::new(handler));
        tracing::debug!("Registered handler for packet_id={}", packet_id);
    }

    /// 派发包到已注册的 handler
    pub fn dispatch(&self, session_id: u32, packet_id: u16, payload: &[u8]) {
        let handlers = self.handlers.lock().expect("PacketRouter lock poisoned");
        match handlers.get(&packet_id) {
            Some(handler) => {
                handler(session_id, payload);
            }
            None => {
                tracing::debug!(
                    "No handler registered for packet_id={} (session={})",
                    packet_id,
                    session_id
                );
            }
        }
    }

    /// 返回已注册的 handler 数量
    pub fn handler_count(&self) -> usize {
        let handlers = self.handlers.lock().expect("PacketRouter lock poisoned");
        handlers.len()
    }
}

/// 游戏逻辑处理器
///
/// 包含所有客户端包的处理函数，处理行走、攻击、聊天、拾取等游戏逻辑。
pub struct GameLogicHandler {
    session_manager: Arc<SessionManager>,
    world_state: Option<Arc<WorldState>>,
}

impl GameLogicHandler {
    pub fn new(session_manager: Arc<SessionManager>) -> Self {
        Self {
            session_manager,
            world_state: None,
        }
    }

    /// 设置 WorldState（在 WorldState 初始化后调用）
    pub fn set_world_state(&mut self, world_state: Arc<WorldState>) {
        self.world_state = Some(world_state);
    }

    /// 在 PacketRouter 上注册所有游戏包 handler
    pub fn register_all(&self, router: &PacketRouter) {
        // KeepAlive
        router.register(ClientOpcode::KeepAlive as u16, |session_id, _payload| {
            GameLogicHandler::on_keep_alive(session_id);
        });

        // Walk
        let sm = Arc::clone(&self.session_manager);
        let ws = self.world_state.clone();
        router.register(ClientOpcode::Walk as u16, move |session_id, payload| {
            let sm = Arc::clone(&sm);
            let ws = ws.clone();
            let payload = payload.to_vec();
            tokio::spawn(async move {
                GameLogicHandler::on_walk(session_id, &payload, sm, ws).await;
            });
        });

        // Run (same as walk for now)
        let sm = Arc::clone(&self.session_manager);
        let ws = self.world_state.clone();
        router.register(ClientOpcode::Run as u16, move |session_id, payload| {
            let sm = Arc::clone(&sm);
            let ws = ws.clone();
            let payload = payload.to_vec();
            tokio::spawn(async move {
                GameLogicHandler::on_walk(session_id, &payload, sm, ws).await;
            });
        });

        // Attack
        let sm = Arc::clone(&self.session_manager);
        let ws = self.world_state.clone();
        router.register(ClientOpcode::Attack as u16, move |session_id, payload| {
            let sm = Arc::clone(&sm);
            let ws = ws.clone();
            let payload = payload.to_vec();
            tokio::spawn(async move {
                GameLogicHandler::on_attack(session_id, &payload, sm, ws).await;
            });
        });

        // Chat
        let sm = Arc::clone(&self.session_manager);
        router.register(ClientOpcode::Chat as u16, move |session_id, payload| {
            let sm = Arc::clone(&sm);
            let payload = payload.to_vec();
            tokio::spawn(async move {
                GameLogicHandler::on_chat(session_id, &payload, sm).await;
            });
        });

        // Turn
        let sm = Arc::clone(&self.session_manager);
        router.register(ClientOpcode::Turn as u16, move |session_id, payload| {
            let sm = Arc::clone(&sm);
            let payload = payload.to_vec();
            tokio::spawn(async move {
                GameLogicHandler::on_turn(session_id, &payload, sm).await;
            });
        });

        // LogOut
        let sm = Arc::clone(&self.session_manager);
        router.register(ClientOpcode::LogOut as u16, move |session_id, _payload| {
            let sm = Arc::clone(&sm);
            tokio::spawn(async move {
                GameLogicHandler::on_log_out(session_id, sm).await;
            });
        });

        // PickUp
        let sm = Arc::clone(&self.session_manager);
        let ws = self.world_state.clone();
        router.register(ClientOpcode::PickUp as u16, move |session_id, _payload| {
            let sm = Arc::clone(&sm);
            let ws = ws.clone();
            tokio::spawn(async move {
                GameLogicHandler::on_pick_up(session_id, sm, ws).await;
            });
        });

        // CallNPC
        let sm = Arc::clone(&self.session_manager);
        let ws = self.world_state.clone();
        router.register(ClientOpcode::CallNPC as u16, move |session_id, payload| {
            let sm = Arc::clone(&sm);
            let ws = ws.clone();
            let payload = payload.to_vec();
            tokio::spawn(async move {
                GameLogicHandler::on_call_npc(session_id, &payload, sm, ws).await;
            });
        });

        // BuyItem
        let sm = Arc::clone(&self.session_manager);
        let ws = self.world_state.clone();
        router.register(ClientOpcode::BuyItem as u16, move |session_id, payload| {
            let sm = Arc::clone(&sm);
            let ws = ws.clone();
            let payload = payload.to_vec();
            tokio::spawn(async move {
                GameLogicHandler::on_buy_item(session_id, &payload, sm, ws).await;
            });
        });

        // SellItem
        let sm = Arc::clone(&self.session_manager);
        let ws = self.world_state.clone();
        router.register(ClientOpcode::SellItem as u16, move |session_id, payload| {
            let sm = Arc::clone(&sm);
            let ws = ws.clone();
            let payload = payload.to_vec();
            tokio::spawn(async move {
                GameLogicHandler::on_sell_item(session_id, &payload, sm, ws).await;
            });
        });

        tracing::info!(
            "Registered {} client packet handlers",
            router.handler_count()
        );
    }

    // =============================================
    // 包处理函数
    // =============================================

    /// 处理心跳包
    fn on_keep_alive(session_id: u32) {
        tracing::debug!("Session {} KeepAlive", session_id);
    }

    /// 处理行走包
    async fn on_walk(
        session_id: u32,
        payload: &[u8],
        sm: Arc<SessionManager>,
        world_state: Option<Arc<WorldState>>,
    ) {
        if payload.is_empty() {
            return;
        }

        let dir_value = payload[0];
        let direction = direction_to_enum(dir_value);

        // 获取当前状态
        let state = match sm.get_state(session_id).await {
            Some(s) => s,
            None => return,
        };

        // 计算新位置
        let (dx, dy) = direction_offset(direction);
        let new_x = state.location.0 + dx;
        let new_y = state.location.1 + dy;

        // 阻挡检测
        let can_walk = if let Some(ref ws) = world_state {
            ws.map_manager.is_walkable(state.map_id, new_x, new_y)
        } else {
            true // 无 world_state 时默认允许
        };

        if !can_walk {
            // 发送原地位置包，纠正客户端
            let location_packet = UserLocationPacket::new(
                Point {
                    x: state.location.0,
                    y: state.location.1,
                },
                direction,
            );
            if let Ok(data) = location_packet.encode() {
                sm.send_to_session(session_id, &data).await;
            }
            return;
        }

        // 更新状态
        sm.update_state(session_id, |s| {
            s.location = (new_x, new_y);
            s.direction = direction;
        })
        .await;

        // 发送位置确认给客户端
        let location_packet = UserLocationPacket::new(
            Point {
                x: new_x,
                y: new_y,
            },
            direction,
        );
        if let Ok(data) = location_packet.encode() {
            sm.send_to_session(session_id, &data).await;
        }

        // 广播行走动画到同地图的其他玩家
        let walk_packet = ObjectWalkPacket::new(
            session_id,
            Point {
                x: new_x,
                y: new_y,
            },
            direction,
        );
        if let Ok(data) = walk_packet.encode() {
            sm.broadcast_to_map(state.map_id, Some(session_id), &data)
                .await;
        }

        // 检测地图连接点
        if let Some(ref ws) = world_state {
            if let Some(connection) = ws.map_manager.find_connection(state.map_id, new_x, new_y) {
                // 检查目标地图是否存在
                if ws.map_manager.get_map(connection.target_map_id).is_some() {
                    // 更新玩家地图和位置
                    sm.update_state(session_id, |s| {
                        s.map_id = connection.target_map_id;
                        s.location = (connection.target_x, connection.target_y);
                    })
                    .await;

                    // 广播移除旧地图的玩家
                    let remove_packet = ObjectRemovePacket::new(session_id);
                    if let Ok(data) = remove_packet.encode() {
                        sm.broadcast_to_map(state.map_id, Some(session_id), &data).await;
                    }

                    // 发送 MapChanged 包
                    let map_changed = MapChangedPacket::new(connection.target_map_id);
                    if let Ok(data) = map_changed.encode() {
                        sm.send_to_session(session_id, &data).await;
                    }

                    // 发送目标地图信息
                    if let Some(target_map) = ws.map_manager.get_map(connection.target_map_id) {
                        let map_info = MapInformationPacket::new(
                            target_map.id,
                            target_map.width,
                            target_map.height,
                            target_map.title.clone(),
                            target_map.filename.clone(),
                        );
                        if let Ok(data) = map_info.encode() {
                            sm.send_to_session(session_id, &data).await;
                        }
                    }

                    // 发送新位置
                    let new_loc = UserLocationPacket::new(
                        Point {
                            x: connection.target_x,
                            y: connection.target_y,
                        },
                        direction,
                    );
                    if let Ok(data) = new_loc.encode() {
                        sm.send_to_session(session_id, &data).await;
                    }

                    // 广播新地图上的玩家给进入者
                    send_players_on_map(&sm, session_id, connection.target_map_id).await;
                }
            }
        }
    }

    /// 处理攻击包
    async fn on_attack(
        session_id: u32,
        payload: &[u8],
        sm: Arc<SessionManager>,
        world_state: Option<Arc<WorldState>>,
    ) {
        if payload.len() < 2 {
            return;
        }

        let direction = direction_to_enum(payload[0]);
        let _spell = payload[1];

        let state = match sm.get_state(session_id).await {
            Some(s) => s,
            None => return,
        };

        // 获取 world_state
        let ws = match world_state {
            Some(ref w) => Arc::clone(w),
            None => {
                // 没有 world_state，只广播攻击动画
                let attack_packet = ObjectAttackPacket::new(session_id, direction, 0);
                if let Ok(data) = attack_packet.encode() {
                    sm.broadcast_to_map(state.map_id, Some(session_id), &data).await;
                }
                return;
            }
        };

        // 广播攻击动画
        let attack_packet = ObjectAttackPacket::new(session_id, direction, 0);
        if let Ok(data) = attack_packet.encode() {
            sm.broadcast_to_map(state.map_id, Some(session_id), &data).await;
        }

        // 查找攻击方向上的最近怪物
        let (dx, dy) = direction_offset(direction);
        let target_x = state.location.0 + dx;
        let target_y = state.location.1 + dy;

        // 获取怪物管理器
        let monster_manager = ws.monster_manager.read().await;
        let monsters_near =
            monster_manager.get_monsters_near(state.map_id, target_x, target_y, 1);

        if monsters_near.is_empty() {
            return;
        }

        // 攻击第一个怪物
        let target_id = monsters_near[0];
        let monster = match monster_manager.get_monster(target_id) {
            Some(m) => m,
            None => return,
        };

        // 准备攻击参数
        let monster_ac = monster.template.ac as u32;
        let monster_agility = monster.template.agility;
        let monster_x = monster.location.0;
        let monster_y = monster.location.1;

        // 执行伤害计算（需要 mutable access）
        drop(monster_manager);
        let mut mm = ws.monster_manager.write().await;
        let monster = match mm.get_monster_mut(target_id) {
            Some(m) => m,
            None => return,
        };

        let result = CombatSystem::player_attack_monster(
            state.level,
            5,  // 简化：玩家 DC 从装备计算
            15,
            10, // 简化：准确
            &state.last_attack_time,
            state.location.0,
            state.location.1,
            monster_ac,
            monster_agility,
            &mut monster.current_hp,
            target_id,
            monster_x,
            monster_y,
        );

        // 更新上次攻击时间
        drop(mm);
        sm.update_state(session_id, |s| {
            s.last_attack_time = std::time::Instant::now();
            s.direction = direction;
        })
        .await;

        // 广播结果
        if result.hit {
            // 受击动画
            let struck_packet = ObjectStruckPacket::new(target_id, session_id);
            if let Ok(data) = struck_packet.encode() {
                sm.broadcast_to_map(state.map_id, None, &data).await;
            }

            // 伤害指示
            let dmg_packet = DamageIndicatorPacket::new(target_id, result.damage, 0);
            if let Ok(data) = dmg_packet.encode() {
                sm.broadcast_to_map(state.map_id, None, &data).await;
            }

            // 检查怪物死亡
            if !result.target_alive {
                let died_packet = mir2_shared::packets::server::ObjectDiedPacket::new(target_id);
                if let Ok(data) = died_packet.encode() {
                    sm.broadcast_to_map(state.map_id, None, &data).await;
                }

                // 给予经验
                let mm = ws.monster_manager.read().await;
                if let Some(mon) = mm.get_monster(target_id) {
                    let exp = mon.template.exp;
                    sm.update_state(session_id, |s| {
                        s.experience = s.experience.saturating_add(exp as u64);
                    })
                    .await;

                    let exp_packet = mir2_shared::packets::server::GainExperiencePacket::new(exp);
                    if let Ok(data) = exp_packet.encode() {
                        sm.send_to_session(session_id, &data).await;
                    }
                }
            }
        } else {
            // 闪避提示
            let dmg_packet = DamageIndicatorPacket::new(target_id, 0, 1); // Miss 类型
            if let Ok(data) = dmg_packet.encode() {
                sm.broadcast_to_map(state.map_id, None, &data).await;
            }

            let msg = "You missed!".to_string();
            if let Ok(data) = encode_game_message(&msg, ChatType::Hint) {
                sm.send_to_session(session_id, &data).await;
            }
        }
    }

    /// 处理聊天包 — 广播给同地图所有玩家
    async fn on_chat(session_id: u32, payload: &[u8], sm: Arc<SessionManager>) {
        if payload.len() < 2 {
            return;
        }

        let msg_len = u16::from_le_bytes([payload[0], payload[1]]) as usize;
        if payload.len() < 2 + msg_len {
            return;
        }

        let message = String::from_utf8_lossy(&payload[2..2 + msg_len]).to_string();
        tracing::info!("Session {} chat: {}", session_id, message);

        // 获取玩家信息
        let state = match sm.get_state(session_id).await {
            Some(s) => s,
            None => return,
        };

        let player_name = if state.character_name.is_empty() {
            format!("Player {}", session_id)
        } else {
            state.character_name.clone()
        };

        let broadcast_msg = format!("[{}] {}", player_name, message);
        let chat_packet = ServerChatPacket::new(broadcast_msg, ChatType::Normal);

        // 只广播给同地图的玩家
        if let Ok(data) = chat_packet.encode() {
            sm.broadcast_to_map(state.map_id, None, &data).await;
        }
    }

    /// 处理转向包
    async fn on_turn(session_id: u32, payload: &[u8], sm: Arc<SessionManager>) {
        if payload.is_empty() {
            return;
        }

        let dir_value = payload[0];
        let direction = direction_to_enum(dir_value);

        // 更新朝向
        let map_id = match sm.get_state(session_id).await {
            Some(ref s) => {
                if s.direction != direction {
                    sm.update_state(session_id, |s| {
                        s.direction = direction;
                    })
                    .await;
                }
                s.map_id
            }
            None => return,
        };

        // 广播转向动画
        let turn_packet = ObjectTurnPacket::new(session_id, direction);
        if let Ok(data) = turn_packet.encode() {
            sm.broadcast_to_map(map_id, Some(session_id), &data).await;
        }
    }

    /// 处理登出包
    async fn on_log_out(session_id: u32, sm: Arc<SessionManager>) {
        tracing::info!("Player {} logging out", session_id);

        // 获取玩家地图信息，广播移除
        if let Some(state) = sm.get_state(session_id).await {
            let remove_packet = ObjectRemovePacket::new(session_id);
            if let Ok(data) = remove_packet.encode() {
                sm.broadcast_to_map(state.map_id, None, &data).await;
            }
        }

        sm.remove(session_id).await;
    }

    /// 处理拾取包
    async fn on_pick_up(session_id: u32, sm: Arc<SessionManager>, world_state: Option<Arc<WorldState>>) {
        let ws = match world_state {
            Some(ref w) => Arc::clone(w),
            None => return,
        };

        let state = match sm.get_state(session_id).await {
            Some(s) => s,
            None => return,
        };

        // 查找玩家附近的地面物品
        let ground_ids = {
            let item_manager = ws.item_manager.read().await;
            item_manager.get_ground_items_near(state.map_id, state.location.0, state.location.1, 1)
        };

        if ground_ids.is_empty() {
            let msg = "Nothing to pick up.".to_string();
            if let Ok(data) = encode_game_message(&msg, ChatType::Hint) {
                sm.send_to_session(session_id, &data).await;
            }
            return;
        }

        // 拾取第一个物品
        let ground_id = ground_ids[0];
        let (item_id, _count, item_name) = {
            let mut item_manager = ws.item_manager.write().await;
            if let Some(ground) = item_manager.get_ground_item(ground_id) {
                let item_id = ground.item_id;
                let count = ground.count;
                let item_name = ground.item_name.clone();
                item_manager.remove_ground_item(ground_id);
                (item_id, count, item_name)
            } else {
                return;
            }
        };

        // 发送拾取确认
        let gain_packet = GainedItemPacket::new(item_id as u32, item_name.clone());
        if let Ok(data) = gain_packet.encode() {
            sm.send_to_session(session_id, &data).await;
        }

        // 广播地面物品移除
        let remove_packet = ObjectRemovePacket::new(ground_id);
        if let Ok(data) = remove_packet.encode() {
            sm.broadcast_to_map(state.map_id, None, &data).await;
        }

        // 提示消息
        let msg = format!("You picked up {}.", item_name);
        if let Ok(data) = encode_game_message(&msg, ChatType::Hint) {
            sm.send_to_session(session_id, &data).await;
        }
    }

    // =============================================
    // NPC/商店包处理函数
    // =============================================

    /// 处理呼叫 NPC 包
    async fn on_call_npc(
        session_id: u32,
        payload: &[u8],
        sm: Arc<SessionManager>,
        world_state: Option<Arc<WorldState>>,
    ) {
        if payload.len() < 2 {
            return;
        }

        let npc_id = u16::from_le_bytes([payload[0], payload[1]]);
        let ws = match world_state {
            Some(ref w) => Arc::clone(w),
            None => return,
        };

        let state = match sm.get_state(session_id).await {
            Some(s) => s,
            None => return,
        };

        // 查找 NPC 模板
        let npc = match ws.npc_manager.get_npc(npc_id) {
            Some(n) => n.clone(),
            None => {
                let msg = format!("NPC {} not found.", npc_id);
                if let Ok(data) = encode_game_message(&msg, ChatType::Hint) {
                    sm.send_to_session(session_id, &data).await;
                }
                return;
            }
        };

        // 检查距离
        if npc.map_id != state.map_id {
            let msg = "NPC is not on this map.".to_string();
            if let Ok(data) = encode_game_message(&msg, ChatType::Hint) {
                sm.send_to_session(session_id, &data).await;
            }
            return;
        }

        let dist = (npc.x - state.location.0).abs() + (npc.y - state.location.1).abs();
        if dist > 12 {
            let msg = "You are too far away from this NPC.".to_string();
            if let Ok(data) = encode_game_message(&msg, ChatType::Hint) {
                sm.send_to_session(session_id, &data).await;
            }
            return;
        }

        // 如果是商店 NPC，返回商品列表
        if !npc.shop_type.is_empty() && !npc.selling_items.is_empty() {
            let item_manager = ws.item_manager.read().await;
            let mut goods = Vec::with_capacity(npc.selling_items.len());

            for &item_id in &npc.selling_items {
                if let Some(template) = item_manager.get_template(item_id) {
                    goods.push(NPCGoodsItem {
                        item_id: template.id,
                        price: template.price,
                        name: template.name.clone(),
                    });
                }
            }

            let packet = NPCGoodsPacket::new(npc.id, goods);
            if let Ok(data) = packet.encode() {
                sm.send_to_session(session_id, &data).await;
            }
        } else {
            // 非商店 NPC，返回 NPCResponse
            let packet = NPCResponsePacket::new(1); // 1 = 对话成功
            if let Ok(data) = packet.encode() {
                sm.send_to_session(session_id, &data).await;
            }
        }
    }

    /// 处理购买物品包
    async fn on_buy_item(
        session_id: u32,
        payload: &[u8],
        sm: Arc<SessionManager>,
        world_state: Option<Arc<WorldState>>,
    ) {
        if payload.len() < 6 {
            return;
        }

        let npc_id = u16::from_le_bytes([payload[0], payload[1]]);
        let item_id = u16::from_le_bytes([payload[2], payload[3]]);
        let _count = u16::from_le_bytes([payload[4], payload[5]]);

        let ws = match world_state {
            Some(ref w) => Arc::clone(w),
            None => return,
        };

        let state = match sm.get_state(session_id).await {
            Some(s) => s,
            None => return,
        };

        // 验证 NPC
        let npc = match ws.npc_manager.get_npc(npc_id) {
            Some(n) => n.clone(),
            None => {
                let msg = "NPC not found.".to_string();
                if let Ok(data) = encode_game_message(&msg, ChatType::Hint) {
                    sm.send_to_session(session_id, &data).await;
                }
                return;
            }
        };

        // 验证物品
        let item_template = match ws.item_manager.read().await.get_template(item_id) {
            Some(t) => t.clone(),
            None => {
                let msg = "Item not found.".to_string();
                if let Ok(data) = encode_game_message(&msg, ChatType::Hint) {
                    sm.send_to_session(session_id, &data).await;
                }
                return;
            }
        };

        // 检查金币和背包
        let current_gold = state.gold;
        let result = crate::npc::shop::ShopSystem::try_buy(
            &npc,
            &item_template,
            current_gold,
        );

        match result {
            crate::npc::shop::ShopResult::Success => {
                let price = item_template.price as u64;
                // 更新金币
                let new_gold = current_gold - price;
                sm.update_state(session_id, |s| {
                    s.gold = new_gold;
                })
                .await;

                // TODO: 添加物品到背包（需要 InventoryManager）
                let msg = format!("You bought {}.", item_template.name);
                if let Ok(data) = encode_game_message(&msg, ChatType::Hint) {
                    sm.send_to_session(session_id, &data).await;
                }
            }
            crate::npc::shop::ShopResult::InsufficientGold => {
                let msg = "You do not have enough gold.".to_string();
                if let Ok(data) = encode_game_message(&msg, ChatType::Hint) {
                    sm.send_to_session(session_id, &data).await;
                }
            }
            crate::npc::shop::ShopResult::NpcNotSellItem => {
                let msg = "This NPC does not sell that item.".to_string();
                if let Ok(data) = encode_game_message(&msg, ChatType::Hint) {
                    sm.send_to_session(session_id, &data).await;
                }
            }
            _ => {
                let msg = "Transaction failed.".to_string();
                if let Ok(data) = encode_game_message(&msg, ChatType::Hint) {
                    sm.send_to_session(session_id, &data).await;
                }
            }
        }
    }

    /// 处理出售物品包
    async fn on_sell_item(
        session_id: u32,
        payload: &[u8],
        sm: Arc<SessionManager>,
        _world_state: Option<Arc<WorldState>>,
    ) {
        if payload.len() < 7 {
            return;
        }

        let _slot = payload[0];
        let _item_uid = u32::from_le_bytes([payload[1], payload[2], payload[3], payload[4]]);
        let _count = u16::from_le_bytes([payload[5], payload[6]]);

        // TODO: 在完整背包系统就绪后实现出售逻辑
        let msg = "Sell function not yet available.".to_string();
        if let Ok(data) = encode_game_message(&msg, ChatType::Hint) {
            sm.send_to_session(session_id, &data).await;
        }
    }
}

// =============================================
// 辅助函数
// =============================================

/// 编码游戏消息包
fn encode_game_message(message: &str, chat_type: ChatType) -> Result<Vec<u8>, NetError> {
    let packet = GameMessagePacket::new(message.to_string(), chat_type, 0);
    packet.encode()
}

/// u8 → MirDirection
fn direction_to_enum(value: u8) -> MirDirection {
    match value {
        0 => MirDirection::Up,
        1 => MirDirection::UpRight,
        2 => MirDirection::Right,
        3 => MirDirection::DownRight,
        4 => MirDirection::Down,
        5 => MirDirection::DownLeft,
        6 => MirDirection::Left,
        7 => MirDirection::UpLeft,
        _ => MirDirection::Down,
    }
}

/// 获取方向偏移
fn direction_offset(dir: MirDirection) -> (i32, i32) {
    match dir {
        MirDirection::Up => (0, -1),
        MirDirection::UpRight => (1, -1),
        MirDirection::Right => (1, 0),
        MirDirection::DownRight => (1, 1),
        MirDirection::Down => (0, 1),
        MirDirection::DownLeft => (-1, 1),
        MirDirection::Left => (-1, 0),
        MirDirection::UpLeft => (-1, -1),
    }
}

/// 向新地图上的玩家广播本玩家的 ObjectPlayer 包
async fn send_players_on_map(sm: &SessionManager, session_id: u32, map_id: u16) {
    let state = match sm.get_state(session_id).await {
        Some(s) => s,
        None => return,
    };

    // 将新玩家广播给同地图其他玩家
    let player_packet = ObjectPlayerPacket::new(
        session_id,
        if state.character_name.is_empty() {
            format!("Player{}", session_id)
        } else {
            state.character_name.clone()
        },
        mir2_shared::enums::MirClass::Warrior,
        mir2_shared::enums::MirGender::Male,
        Point {
            x: state.location.0,
            y: state.location.1,
        },
        state.direction,
    );

    if let Ok(data) = player_packet.encode() {
        sm.broadcast_to_map(map_id, Some(session_id), &data).await;
    }

    // 向新玩家广播地图上已有玩家
    let map_states = sm.get_states_on_map(map_id).await;
    for (sid, other_state) in map_states {
        if sid == session_id {
            continue;
        }

        let other_packet = ObjectPlayerPacket::new(
            sid,
            if other_state.character_name.is_empty() {
                format!("Player{}", sid)
            } else {
                other_state.character_name.clone()
            },
            mir2_shared::enums::MirClass::Warrior,
            mir2_shared::enums::MirGender::Male,
            Point {
                x: other_state.location.0,
                y: other_state.location.1,
            },
            other_state.direction,
        );

        if let Ok(data) = other_packet.encode() {
            sm.send_to_session(session_id, &data).await;
        }
    }
}
