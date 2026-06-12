import { useEffect, useRef, useCallback } from 'react';
import { Application, Container, Graphics, Text, TextStyle } from 'pixi.js';
import { GameWorldState } from '../types/game';

/** 每格像素大小 */
const TILE_SIZE = 32;

/** 地图格颜色映射 */
const TILE_COLORS: Record<number, number> = {
  0: 0x4a7c3f, // Walk — 草地
  1: 0x666666, // HighWall — 墙壁
  2: 0x8b7355, // LowWall — 道路
};

/** 安全区叠加色（半透明） */
const SAFE_ZONE_COLOR = 0x5a9e4f;

/** 玩家颜色 */
const PLAYER_COLOR = 0x4488ff;

/** 怪物颜色 */
const MONSTER_COLOR = 0xff4444;

/** 其他玩家颜色 */
const OTHER_PLAYER_COLOR = 0x44ff44;

/** 地面物品颜色 */
const ITEM_COLOR = 0xffff44;

export interface GameCanvasProps {
  state: GameWorldState;
  onTileClick: (x: number, y: number) => void;
  onMonsterClick: (objectId: number) => void;
}

/**
 * PixiJS 游戏场景渲染组件
 *
 * 使用 PixiJS Application 渲染地图、实体和特效。
 * 相机跟随玩家，将场景偏移使玩家居中。
 */
export function GameCanvas({ state, onTileClick, onMonsterClick }: GameCanvasProps) {
  const containerRef = useRef<HTMLDivElement>(null);
  const appRef = useRef<Application | null>(null);
  const sceneContainerRef = useRef<Container>(new Container());
  const tileGraphicsRef = useRef<Graphics>(new Graphics());
  const entityGraphicsRef = useRef<Graphics>(new Graphics());
  const overlayGraphicsRef = useRef<Graphics>(new Graphics());

  // 初始化 PixiJS
  useEffect(() => {
    const container = containerRef.current;
    if (!container) return;

    const app = new Application();
    appRef.current = app;

    app.init({
      resizeTo: container,
      backgroundColor: 0x1a1a2e,
      antialias: true,
      autoDensity: true,
    }).then(() => {
      container.appendChild(app.canvas as HTMLCanvasElement);

      const scene = sceneContainerRef.current;
      const tileGfx = tileGraphicsRef.current;
      const entityGfx = entityGraphicsRef.current;
      const overlayGfx = overlayGraphicsRef.current;

      scene.addChild(tileGfx);
      scene.addChild(entityGfx);
      scene.addChild(overlayGfx);

      app.stage.addChild(scene);

      // 交互事件 — 点击地图
      app.stage.eventMode = 'static';
      app.stage.on('pointerdown', (event) => {
        const pos = event.global;
        // 将屏幕坐标转换为世界坐标
        const worldX = pos.x - app.screen.width / 2 + (state.player?.location.x ?? 0) * TILE_SIZE + TILE_SIZE / 2;
        const worldY = pos.y - app.screen.height / 2 + (state.player?.location.y ?? 0) * TILE_SIZE + TILE_SIZE / 2;

        const tileX = Math.floor(worldX / TILE_SIZE);
        const tileY = Math.floor(worldY / TILE_SIZE);

        // 检查是否点击了怪物
        let clickedMonster = false;
        for (const [, monster] of state.monsters) {
          if (monster.location.x === tileX && monster.location.y === tileY) {
            onMonsterClick(monster.objectId);
            clickedMonster = true;
            break;
          }
        }

        if (!clickedMonster) {
          onTileClick(tileX, tileY);
        }
      });
    });

    return () => {
      app.destroy(true);
      appRef.current = null;
    };
  }, []); // 只初始化一次

  // 渲染地图格
  const renderTiles = useCallback(() => {
    const gfx = tileGraphicsRef.current;
    gfx.clear();

    if (state.mapWidth === 0 || state.mapHeight === 0) return;

    for (let y = 0; y < state.mapHeight; y++) {
      for (let x = 0; x < state.mapWidth; x++) {
        const idx = y * state.mapWidth + x;
        const tile = state.tiles[idx];
        if (!tile) continue;

        const color = TILE_COLORS[tile.attr] ?? 0x000000;
        const px = x * TILE_SIZE;
        const py = y * TILE_SIZE;

        gfx.setFillStyle({ color });
        gfx.rect(px, py, TILE_SIZE, TILE_SIZE);
        gfx.fill();

        // 安全区叠加
        if (tile.isSafeZone) {
          gfx.setFillStyle({ color: SAFE_ZONE_COLOR, alpha: 0.15 });
          gfx.rect(px, py, TILE_SIZE, TILE_SIZE);
          gfx.fill();
        }

        // 边框
        gfx.setStrokeStyle({ color: 0x000000, width: 0.5, alpha: 0.15 });
        gfx.rect(px, py, TILE_SIZE, TILE_SIZE);
        gfx.stroke();
      }
    }
  }, [state.mapWidth, state.mapHeight, state.tiles]);

  // 渲染实体
  const renderEntities = useCallback(() => {
    const gfx = entityGraphicsRef.current;
    gfx.clear();
    // 移除之前添加的 Text 子对象
    while (gfx.children.length > 0) {
      gfx.removeChildAt(0);
    }

    // 绘制其他玩家
    for (const [, player] of state.otherPlayers) {
      const px = player.location.x * TILE_SIZE;
      const py = player.location.y * TILE_SIZE;
      gfx.setFillStyle({ color: OTHER_PLAYER_COLOR });
      gfx.rect(px + 4, py + 4, TILE_SIZE - 8, TILE_SIZE - 8);
      gfx.fill();

      // 玩家名称
      const nameText = new Text({
        text: player.name,
        style: new TextStyle({ fontSize: 10, fill: '#ffffff' }),
      });
      nameText.x = px;
      nameText.y = py - 14;
      gfx.addChild(nameText);
    }

    // 绘制怪物
    for (const [, monster] of state.monsters) {
      if (!monster.isAlive) continue;
      const px = monster.location.x * TILE_SIZE;
      const py = monster.location.y * TILE_SIZE;

      // 怪物身体（红色菱形）
      gfx.setFillStyle({ color: MONSTER_COLOR });
      gfx.moveTo(px + TILE_SIZE / 2, py + 4);
      gfx.lineTo(px + TILE_SIZE - 4, py + TILE_SIZE / 2);
      gfx.lineTo(px + TILE_SIZE / 2, py + TILE_SIZE - 4);
      gfx.lineTo(px + 4, py + TILE_SIZE / 2);
      gfx.closePath();
      gfx.fill();

      // 怪物名称
      const nameText = new Text({
        text: monster.name,
        style: new TextStyle({ fontSize: 10, fill: '#ffaaaa' }),
      });
      nameText.x = px;
      nameText.y = py - 14;
      gfx.addChild(nameText);

      // 血条
      if (monster.maxHp > 0) {
        const hpRatio = monster.currentHp / monster.maxHp;
        gfx.setFillStyle({ color: 0x333333 });
        gfx.rect(px + 2, py - 4, TILE_SIZE - 4, 3);
        gfx.fill();
        gfx.setFillStyle({ color: hpRatio > 0.5 ? 0x44cc44 : hpRatio > 0.25 ? 0xcccc44 : 0xcc4444 });
        gfx.rect(px + 2, py - 4, (TILE_SIZE - 4) * hpRatio, 3);
        gfx.fill();
      }
    }

    // 绘制地面物品
    for (const [, item] of state.groundItems) {
      const px = item.location.x * TILE_SIZE;
      const py = item.location.y * TILE_SIZE;
      gfx.setFillStyle({ color: ITEM_COLOR });
      gfx.circle(px + TILE_SIZE / 2, py + TILE_SIZE / 2, 6);
      gfx.fill();

      // 物品名称
      const nameText = new Text({
        text: item.name,
        style: new TextStyle({ fontSize: 10, fill: '#ffff88' }),
      });
      nameText.x = px;
      nameText.y = py - 14;
      gfx.addChild(nameText);
    }

    // 绘制玩家
    if (state.player) {
      const px = state.player.location.x * TILE_SIZE;
      const py = state.player.location.y * TILE_SIZE;

      gfx.setFillStyle({ color: PLAYER_COLOR });
      gfx.rect(px + 4, py + 4, TILE_SIZE - 8, TILE_SIZE - 8);
      gfx.fill();

      // 玩家名称
      const nameText = new Text({
        text: state.player.name,
        style: new TextStyle({ fontSize: 11, fill: '#ffffff', fontWeight: 'bold' }),
      });
      nameText.x = px;
      nameText.y = py - 16;
      gfx.addChild(nameText);
    }
  }, [state.player, state.monsters, state.otherPlayers, state.groundItems]);

  // 相机跟随
  const updateCamera = useCallback(() => {
    const app = appRef.current;
    if (!app || !state.player) return;

    const targetX = -(state.player.location.x * TILE_SIZE) + app.screen.width / 2 - TILE_SIZE / 2;
    const targetY = -(state.player.location.y * TILE_SIZE) + app.screen.height / 2 - TILE_SIZE / 2;

    sceneContainerRef.current.x = targetX;
    sceneContainerRef.current.y = targetY;
  }, [state.player]);

  // 当 tiles 变化时重新渲染地图
  useEffect(() => {
    renderTiles();
  }, [renderTiles]);

  // 当实体状态变化时重新渲染
  useEffect(() => {
    renderEntities();
  }, [renderEntities]);

  // 相机跟随
  useEffect(() => {
    updateCamera();
  }, [updateCamera]);

  return (
    <div
      ref={containerRef}
      style={{
        position: 'absolute',
        top: 0,
        left: 0,
        width: '100%',
        height: '100%',
        overflow: 'hidden',
      }}
    />
  );
}
