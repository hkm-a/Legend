# Game Concept: 新玛法：觉醒客户端

*Created: 2026-06-02*
*Status: Draft*

> **Creative Director Review (CD-PILLARS)**: CONCERNS accepted 2026-06-02 — pillars revised for clearer 30-second feedback, long-term growth, and classic-vs-modern boundaries.
> **Art Director Review (AD-CONCEPT-VISUAL)**: CONCEPTS selected 2026-06-02 — Visual Identity Anchor set to 赤金玛法 with 爆光战利品 and 灵纹成长 accents.
> **Technical Director Review (TD-FEASIBILITY)**: CONCERNS accepted 2026-06-02 — project is viable if OpenMir2 mapping and Godot resource/rendering spikes happen early.
> **Producer Review (PR-SCOPE)**: OPTIMISTIC accepted 2026-06-02 — Phase 1 narrowed to an offline 30-second loot-loop technical slice.

---

## Elevator Pitch

> 《新玛法：觉醒客户端》是一款用 Godot 制作的现代化传奇 PC 客户端，玩家在玛法大陆中稳定刷怪、爆装备、拾取穿戴并持续变强。它保留经典传奇的职业、刷怪、爆装和地图推进骨架，同时用现代 UI、清晰掉落反馈和长期养成目标让新老玩家都更容易进入刷装循环。

---

## Core Identity

| Aspect | Detail |
| ---- | ---- |
| **Genre** | 2D / 2.5D 传奇类 ARPG 客户端，刷怪爆装养成 |
| **Platform** | PC 原生客户端 |
| **Target Audience** | 喜欢传奇刷怪爆装、长期养成和角色变强反馈的中核玩家 |
| **Player Count** | Phase 1 为单人离线或半离线；Full Vision 可扩展到弱联网 / 多人客户端 |
| **Session Length** | 30–120 分钟，支持短刷图和长时间养成 session |
| **Monetization** | 暂不确定；概念阶段不设计商业化 |
| **Estimated Scope** | Large (12–24 months, solo learning developer) |
| **Comparable Titles** | 经典传奇 / OpenMir2、原神、崩坏：星穹铁道、阴阳师 |

---

## Core Fantasy

玩家进入熟悉但更现代的玛法大陆，通过稳定刷怪、看到装备爆出、拾取并穿戴装备，让角色一步步从普通冒险者成长为能挑战更高地图和 Boss 的强者。

核心幻想不是高操作动作战斗，而是“我每次上线都能刷到东西、看懂价值、换上装备、明确变强”。玩家要感到自己正在亲手养成一个传奇角色，爆装是最强瞬间，长期成长是持续回来的理由。

---

## Unique Hook

它像经典传奇，**and also** 有现代养成游戏式的清晰成长反馈、装备价值判断、掉落高光、任务/目标引导和更顺滑的客户端体验。

差异点在于：它不是把传奇改成纯二游抽卡，也不是只做复古复刻；它以 OpenMir2 / 经典传奇行为为底层参考，把玩家最在意的刷怪爆装、装备判断和角色变强做得更清楚、更爽、更容易持续玩。

---

## Player Experience Analysis (MDA Framework)

### Target Aesthetics (What the player FEELS)

| Aesthetic | Priority | How We Deliver It |
| ---- | ---- | ---- |
| **Submission** (relaxation, comfort zone) | 1 | 低压力刷怪、稳定掉落、清晰目标、流畅移动和战斗节奏 |
| **Sensation** (sensory pleasure) | 2 | 掉落光效、拾取音效、装备品质色、战力变化反馈 |
| **Fantasy** (make-believe, role-playing) | 3 | 玛法大陆、职业身份、装备成长、Boss 和地图推进 |
| **Expression** (self-expression, creativity) | 4 | 装备选择、属性成长、后续职业/外观/Build 扩展 |
| **Challenge** (obstacle course, mastery) | 5 | 地图难度、精英怪、Boss、战力门槛；不以高操作为核心 |
| **Discovery** (exploration, secrets) | 6 | 新地图、新怪物、掉落表、装备来源和成长路线 |
| **Fellowship** (social connection) | 7 | 后续通过组队、行会、交易扩展；Phase 1 不做核心依赖 |
| **Narrative** (drama, story arc) | 8 | 轻量世界背景和目标引导；不做复杂剧情演出 |

### Key Dynamics (Emergent player behaviors)

- 玩家会寻找当前战力最适合的刷怪区域。
- 玩家会因为“还差一点升级 / 还差一个装备部位 / 背包还没满”继续再刷一轮。
- 玩家会快速判断掉落价值，把无用装备卖掉或丢弃，把更强装备穿上。
- 玩家会为了进入更高地图、挑战精英怪或 Boss 而持续提升等级和装备。
- 后续多人阶段中，玩家会围绕掉落、Boss、交易和行会形成社交目标。

### Core Mechanics (Systems we build)

1. 点击移动与传奇式地图坐标 / 阻挡 / 排序。
2. 基础刷怪战斗：锁定、攻击、怪物死亡。
3. 掉落与拾取：装备生成、品质显示、拾取反馈。
4. 背包与装备：极简物品栏、装备槽、装备对比、穿戴。
5. 成长反馈：等级、属性、战力、地图资格或阶段目标推进。

---

## Player Motivation Profile

### Primary Psychological Needs Served

| Need | How This Game Satisfies It | Strength |
| ---- | ---- | ---- |
| **Autonomy** (freedom, meaningful choice) | 玩家选择刷哪个地图、追哪个装备部位、优先升级还是挑战精英怪 / Boss。 | Supporting |
| **Competence** (mastery, skill growth) | 玩家通过更快清怪、更少死亡、能进更高地图、能穿更好装备感到自己和角色都在变强。 | Core |
| **Relatedness** (connection, belonging) | Phase 1 弱化；后续通过行会、交易、组队 Boss、排行榜建立连接。 | Minimal → Supporting |

### Player Type Appeal (Bartle Taxonomy)

- [x] **Achievers** (goal completion, collection, progression) — 核心玩家；通过等级、装备、战力、地图资格和 Boss 目标获得成就感。
- [x] **Explorers** (discovery, understanding systems, finding secrets) — 次要玩家；通过地图推进、掉落来源、装备表和成长路线获得乐趣。
- [x] **Socializers** (relationships, cooperation, community) — 后续扩展；行会、交易、组队 Boss 可服务这类玩家。
- [ ] **Killers/Competitors** (domination, PvP, leaderboards) — 暂不作为 Phase 1 目标；PvP / 攻沙属于远期高风险内容。

### Flow State Design

- **Onboarding curve**：前 10 分钟只教移动、攻击、拾取、穿戴和“装备让你变强”；不引入复杂系统。
- **Difficulty scaling**：从普通怪到精英怪，再到更高地图和 Boss；难度主要由战力、装备、地图门槛推动。
- **Feedback clarity**：掉落品质、装备对比、战力 +X、属性变化、地图资格都必须清晰可见。
- **Recovery from failure**：Phase 1 尽量低惩罚；死亡或失败应快速回到刷怪点，不破坏流畅刷装循环。

---

## Core Loop

### Moment-to-Moment (30 seconds)

玩家在地图中点击移动到怪物附近，使用基础攻击稳定击杀怪物。怪物死亡后掉落装备或金币 / 材料，掉落物有清晰的视觉和音效反馈；玩家拾取后进入背包，快速对比当前装备，穿戴后看到属性或战力提升。

这个循环的满足感来自：击杀节奏稳定、掉落出现有期待、拾取后能立刻判断价值、穿上装备能马上确认变强。

### Short-Term (5-15 minutes)

玩家选择一个适合当前战力的刷怪区域，完成一轮清怪、拾取、整理背包、换装或卖出无用装备，然后决定是否继续刷一轮。

“一再来一轮”的心理来自：
- 还差一点升级；
- 还差一个装备部位；
- 当前地图可能出稀有装备；
- 背包还没满；
- 精英怪或 Boss 刷新点快到了。

### Session-Level (30-120 minutes)

一次完整 session 可以是：登录角色，查看当前目标，去目标地图刷怪，爆装、升级、换装，回城处理背包和强化 / 卖装，然后尝试更高难度地图、精英怪或 Boss。

自然停止点包括：升了一级、换了一件核心装备、打完一个精英怪 / Boss、完成一个区域目标、背包满回城整理、角色战力达到下一阶段。

### Long-Term Progression

玩家长期成长来自：
- 等级提升；
- 装备品质和部位提升；
- 技能等级；
- 地图解锁；
- Boss 挑战资格；
- 外观、称号和角色身份展示；
- 后续行会、交易、组队和排行榜目标。

长期目标是从新手角色成长为能单刷高阶地图、参与世界 Boss 或行会玩法的强角色。

### Retention Hooks

- **Curiosity**：新地图、新怪物、新掉落表、未解锁 Boss。
- **Investment**：已经养成的角色、装备、材料、地图资格和阶段目标进度。
- **Social**：后续通过行会、交易、组队 Boss 和排行榜建立。
- **Mastery**：更高效刷怪、更合理装备选择、更高地图推进、更快 Boss 击杀。

---

## Game Pillars

### Pillar 1: 稳刷不断流

玩家每 30 秒内都应获得一次可感知的循环反馈：击杀、掉落、拾取、装备判断、进度推进或明确下一目标；等待、迷路、整理背包都不能长期打断刷怪流。

*Design test*: 如果在“复杂操作”和“持续顺滑刷怪”之间选择，本柱要求选择持续顺滑刷怪。

### Pillar 2: 爆装有戏

掉落必须看得见、听得见、想捡，并能快速判断价值；宁可略夸张，也不能平淡到被玩家忽略。

*Design test*: 如果一个掉落反馈更写实但玩家容易忽略，另一个略夸张但能立刻制造“想捡”的冲动，本柱选择后者；但不得让反馈掩盖物品价值判断。

### Pillar 3: 传奇骨架，现代皮肤

职业、刷怪、爆装、地图推进与成长逻辑遵循经典传奇底层节奏；UI、操作、反馈、引导和展示用现代标准重做。

*Design test*: 如果在“完全重做成新 ARPG”和“保留传奇规则但优化体验”之间选择，本柱要求保留传奇规则并优化体验。

### Pillar 4: 每次登录都带走成长

每次游玩结束，玩家都应带走一个明确长期进展：等级、装备、材料、技能、地图资格、Boss 门票或阶段目标推进。

*Design test*: 如果一个系统很酷但不能支持角色长期成长，本柱要求先做能带来成长闭环的系统。

### Anti-Pillars (What This Game Is NOT)

- **NOT 大世界开放探索优先**：不先做大世界开放探索，因为它会稀释稳刷爆装的核心循环。
- **NOT 复杂剧情演出优先**：不先做复杂剧情演出，因为当前目标是现代化传奇客户端，不是剧情 RPG。
- **NOT 全量 MMO 社交生态优先**：不先做完整行会、交易、组队、攻沙，因为 MVP 必须先验证刷怪爆装和客户端行为。
- **NOT 纯二游抽卡**：可以学习养成反馈，但不能让抽卡替代传奇的打怪爆装核心。
- **NOT 高操作动作战斗优先**：不先做复杂连招、闪避、格挡，因为它们会打断 Relaxation & Flow，把游戏推向现代 ARPG 而不是传奇刷装客户端。

---

## Visual Identity Anchor

### Selected Direction: 赤金玛法

**One-line visual rule**：所有重要成长反馈都必须像“火光照亮铁器”一样，热、亮、直接、可读。

### Supporting Visual Principles

1. **掉落优先级高于环境装饰**
   - 原则：任何战斗画面中，装备掉落、拾取提示、价值判断必须比地面细节更显眼。
   - Design test：截图缩小到 25% 后，玩家是否仍能在 1 秒内看出哪里有掉落、哪个更值钱？

2. **成长反馈必须有“热量”**
   - 原则：升级、装备提升、强化完成都使用暖色、放射光、短促冲击动效，形成稳定奖励节奏。
   - Design test：关闭文字提示后，仅看光效和颜色，玩家能否判断“这是变强了”而不是普通 UI 变化？

3. **现代 UI 不能破坏传奇骨架**
   - 原则：交互可以现代，但视觉材质必须保留金属、皮革、石板、符文、火光等玛法质感。
   - Design test：把 HUD 截图单独拿出来看，是否仍像“传奇客户端”，而不是通用手游 RPG 模板？

### Color Philosophy

- 基础世界色：暗褐、铁灰、煤黑、旧石、兽皮棕。
- 成长反馈色：赤金、熔橙、热铁红。
- 稀有掉落色：蓝、紫、金、赤金递进，金色必须克制，只给真正高价值瞬间。
- UI 背景：低饱和深色，让装备光效、品质边框、掉落名称成为视觉焦点。
- 掉落系统吸收「爆光战利品」方向：高价值掉落可以使用光柱、地面高亮环、品质边框和更强音效。
- 长期成长少量吸收「灵纹成长」方向：装备、技能、称号和角色周围可用赤金灵纹表达阶段成长，但不能把游戏推向纯抽卡养成。

---

## Inspiration and References

| Reference | What We Take From It | What We Do Differently | Why It Matters |
| ---- | ---- | ---- | ---- |
| 经典传奇 / OpenMir2 | 职业、刷怪、爆装、地图推进、装备成长、玛法质感 | 使用 Godot 现代化 UI、反馈和客户端体验；实现前先读 OpenMir2 原源码 | 保住“传奇骨架”，避免跑偏成新 ARPG |
| 原神 | 角色成长、阶段目标、长期养成反馈 | 不做开放世界探索优先，不做抽卡替代爆装 | 学习清晰成长目标和长期投入感 |
| 崩坏：星穹铁道 | 清晰角色养成、界面信息层级、目标引导 | 不做回合制战斗，不让菜单养成压过刷怪爆装 | 学习现代 UI 和成长表达 |
| 阴阳师 | 长期养成、装备/御魂式追求、角色提升成就感 | 不做纯抽卡收集；装备来源以打怪爆装为核心 | 学习“刷取—判断—提升”的长期循环 |

**Non-game inspirations**：铁匠铺、火光、旧石城、矿洞、金属装备落地、暗色地图中的赤金奖励光。

---

## Target Player Profile

| Attribute | Detail |
| ---- | ---- |
| **Age range** | 18–40 岁，包含怀旧传奇玩家和现代养成玩家 |
| **Gaming experience** | Mid-core；能接受刷怪养成和长期目标，但不要求高操作动作技巧 |
| **Time availability** | 平日 30–60 分钟刷一轮，周末可 1–2 小时长刷 |
| **Platform preference** | PC 客户端 |
| **Current games they play** | 经典传奇 / 私服、原神、崩坏：星穹铁道、阴阳师、刷装 ARPG |
| **What they're looking for** | 爆装备的爽感、角色成长成就感、清晰目标、低压力刷图循环 |
| **What would turn them away** | 掉落不明显、成长不清楚、操作太复杂、强制社交、纯抽卡、范围太散导致核心刷装不爽 |

---

## Technical Considerations

| Consideration | Assessment |
| ---- | ---- |
| **Recommended Engine** | Godot；适合 2D / 2.5D PC 客户端、UI、场景组织和独立开发学习，但 Godot 4.6 API 需查版本文档 |
| **Key Technical Challenges** | OpenMir2 行为/协议对齐、传奇资源与地图转换、坐标映射、Y-sort / 遮挡、掉落和背包 UI、后续网络同步 |
| **Art Style** | 2D / 2.5D，赤金玛法：厚重复古世界 + 现代清晰 UI 和掉落反馈 |
| **Art Pipeline Complexity** | Medium → High；若使用原传奇资源/格式，需要专门转换和合法性确认 |
| **Audio Needs** | Moderate；击杀、掉落、拾取、穿戴、升级必须有清晰奖励音效 |
| **Networking** | Phase 1 None / optional spike；后续 Client-Server。PC 原生客户端避免 Web TCP 限制 |
| **Content Volume** | Phase 1 一个地图、一种普通怪、可选精英怪、少量装备；Full Vision 多地图、多怪物、Boss、NPC、装备表 |
| **Procedural Systems** | Phase 1 无；掉落表可数据驱动但不做程序生成地图 |

---

## Risks and Open Questions

### Design Risks

- 核心刷怪爆装如果 30 秒内不够爽，后续内容无法补救。
- 现代化反馈如果过度，可能丢掉传奇味。
- 如果过早加入剧情、开放世界或高操作战斗，会削弱 Relaxation & Flow。

### Technical Risks

- OpenMir2 行为/协议细节复杂，必须以原源码为权威参考，不能以 MinimalMirClient 做设计依据。
- 传奇地图、资源、动画、坐标和遮挡在 Godot 中的适配可能比战斗代码更难。
- Godot 4.6 超出部分模型知识范围，API 建议必须对照 `docs/engine-reference/godot/VERSION.md` 和官方文档。
- 如果联网过早进入主线，会拖慢 Phase 1 核心循环验证。

### Market Risks

- 传奇类体验有强情怀门槛，新玩家可能不理解慢节奏刷装乐趣。
- 如果只做复古复刻，现代养成玩家可能觉得反馈不足。
- 如果太像二游或手游模板，传奇玩家可能觉得失去原味。

### Scope Risks

- 完整 MMO、行会、交易、PvP、攻沙、高并发服务端对 solo 初学者不现实。
- 背包 + 装备对比 + 穿戴 + 属性变化提示看似小，但 UI 和数据结构容易膨胀。
- OpenMir2 完整对齐不能放进第一阶段主线，只能先做核心行为映射。

### Open Questions

- OpenMir2 源码中移动、攻击、掉落、拾取、背包、装备的最小行为链路分别在哪里？通过源码行为对齐 spike 回答。
- Godot 能否稳定承载传奇地图、坐标、Y-sort 和遮挡？通过地图 / 坐标 / 渲染排序 spike 回答。
- 掉落反馈在临时素材下是否仍然让人想捡？通过离线 30 秒刷怪爆装切片回答。
- 后续是否需要连接 OpenMir2 风格服务端？Phase 1 不依赖，只保留 PC Socket / 最小协议连通 spike。

---

## MVP Definition

**Core hypothesis**：玩家在一个极小离线 Godot PC 切片中，只要能 30 秒内完成“移动 → 打怪 → 掉落 → 拾取 → 判断价值 → 穿戴 → 看到变强”，就会感到传奇刷装循环是爽的，值得继续扩展。

**Required for MVP**:
1. 一个可移动角色。
2. 一个小型刷怪地图。
3. 一种普通怪；精英怪可选。
4. 点击移动和基础攻击。
5. 怪物死亡后掉落装备。
6. 掉落物有清晰高光、品质和拾取反馈。
7. 极简背包。
8. 极简装备槽。
9. 装备对比只比较战力 / 主属性。
10. 穿戴后显示属性或战力变化。

**Explicitly NOT in MVP**:
- 登录 / 选角。
- Socket / 完整联网。
- 多地图。
- NPC / 商店 / 任务。
- Boss 门票。
- 组队 / 行会 / 交易 / PvP / 攻沙。
- 多职业。
- 复杂技能树。
- 完整 OpenMir2 数据兼容和完整协议复刻。
- 套装、强化、宝石、洗练、耐久、绑定、职业限制等复杂装备系统。

### Scope Tiers (if budget/time shrinks)

| Tier | Content | Features | Timeline |
| ---- | ---- | ---- | ---- |
| **Tier 0: 技术验证** | 一个测试地图、一个假怪物、一个掉落图标 | 点击移动、简单排序/阻挡、击杀后掉落、拾取后显示“战力 +X” | 2–4 weeks |
| **Tier 1: First Playable MVP** | 一个角色、一个地图、一种普通怪、可选精英怪、少量装备 | 基础攻击、掉落、拾取、极简背包、极简装备槽、战力对比、掉落高光 | 1–3 months |
| **Tier 2: Vertical Slice Plus** | 第二张地图、简单 Boss 或精英刷新点、2–3 类装备品质 | 地图推进、基础商店/回收、目标引导、更好的掉落展示 | 3–6 months |
| **Tier 3: Playable Alpha** | 多地图、多怪物、Boss 掉落表、NPC 商店、简单任务 | 装备词条、材料成长、基础存档、可重复刷装循环 | 6–12 months |
| **Tier 4: Fuller Vision** | 更多地图、多职业、更多技能、半联网或局域网联机 | 登录/选角、基础组队/行会/交易、更深入 OpenMir2 行为对齐；不承诺完整 MMO、攻沙、高并发 | 12–24 months |

---

## Technical Spikes Before Production

1. **OpenMir2 源码行为对齐 spike**：定位移动、攻击、怪物死亡、掉落、拾取、背包、装备的数据结构和规则来源，产出“Godot 客户端系统 ↔ OpenMir2 源码模块 ↔ MVP 行为规则”映射。
2. **Godot PC Socket / 最小协议连通 spike**：证明 Godot PC 客户端能建立 TCP 连接、发送/接收最小协议消息并记录日志；MinimalMirClient 只用于验证，不作为权威参考。
3. **地图 / 坐标 / 渲染排序 spike**：一小块地图、玩家、怪物、掉落物、点击移动、Y-sort / 遮挡和基础阻挡格跑通。
4. **掉落反馈与装备变强 spike**：怪物死亡生成装备掉落，拾取进入背包，装备对比，穿戴后属性或战力变化提示。

---

## Next Steps

**Path A — Design-First** (recommended if the concept is well-defined):
1. Run `/setup-engine` to configure the engine and populate version-aware reference docs.
2. Run `/art-bible` to create the visual identity specification — do this BEFORE writing GDDs. **The art bible is required before the Technical Setup gate.** It gates asset production and shapes technical architecture decisions (rendering, VFX, UI systems).
3. Use `/design-review design/gdd/game-concept.md` to validate concept completeness before going downstream.
4. Discuss vision with the `creative-director` agent for pillar refinement.
5. Decompose the concept into individual systems with `/map-systems` — maps dependencies, assigns priorities, and creates the systems index.
6. Author per-system GDDs with `/design-system` — guided, section-by-section GDD writing for each system identified in step 5.
7. Plan the technical architecture with `/create-architecture` — produces the master architecture blueprint and Required ADR list.
8. Record key architectural decisions with `/architecture-decision (×N)` — write one ADR per decision in the Required ADR list from `/create-architecture`.
9. Run `/architecture-review` — bootstraps the TR registry and Requirements Traceability Matrix from your GDDs and ADRs (required before the Pre-Production gate).
10. Validate readiness to advance with `/gate-check` — phase gate before committing to production.

**Path B — Prototype-First** (use if the core mechanic is unproven or the concept needs validation):
1. Run `/setup-engine` to configure the engine.
2. Run `/prototype 30秒刷怪爆装循环` — validate the core idea is fun before writing any GDDs (1–3 days throwaway code).
3. If prototype PROCEEDS: run `/art-bible`, then continue with Path A steps 5–10 above, using prototype learnings to inform your GDDs.
4. If prototype PIVOTS: return to `/brainstorm` with the learnings and reshape the concept.
5. After full design and architecture, build the `/vertical-slice` to validate production readiness before committing to sprints.
