import { useRef, useEffect } from 'react';
import { Box } from '@mui/material';
import { GameWorldState } from '../types/game';

const MINIMAP_SIZE = 150;
const TILE_DOT_SIZE = 2;

export interface MinimapProps {
  state: GameWorldState;
}

/**
 * 小地图组件
 *
 * 固定在右上角，使用 Canvas 2D 渲染缩略图。
 * 绿色点标记玩家位置，红点标记怪物。
 */
export function Minimap({ state }: MinimapProps) {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    ctx.clearRect(0, 0, MINIMAP_SIZE, MINIMAP_SIZE);

    // 背景
    ctx.fillStyle = 'rgba(0, 0, 0, 0.6)';
    ctx.fillRect(0, 0, MINIMAP_SIZE, MINIMAP_SIZE);

    if (state.mapWidth === 0 || state.mapHeight === 0) {
      ctx.fillStyle = '#666666';
      ctx.font = '12px sans-serif';
      ctx.textAlign = 'center';
      ctx.fillText('无地图', MINIMAP_SIZE / 2, MINIMAP_SIZE / 2);
      return;
    }

    // 计算缩放比例
    const scaleX = MINIMAP_SIZE / state.mapWidth;
    const scaleY = MINIMAP_SIZE / state.mapHeight;

    // 绘制地图格（采样绘制）
    const step = Math.max(1, Math.floor(Math.min(state.mapWidth, state.mapHeight) / 50));
    for (let y = 0; y < state.mapHeight; y += step) {
      for (let x = 0; x < state.mapWidth; x += step) {
        const idx = y * state.mapWidth + x;
        const tile = state.tiles[idx];
        if (!tile) continue;

        // 根据格子属性设置颜色
        switch (tile.attr) {
          case 0: // Walk
            ctx.fillStyle = tile.isSafeZone ? '#5a8e4f' : '#3a6c2f';
            break;
          case 1: // HighWall
            ctx.fillStyle = '#555555';
            break;
          case 2: // LowWall
            ctx.fillStyle = '#7a6a4a';
            break;
          default:
            ctx.fillStyle = '#000000';
        }

        const px = x * scaleX;
        const py = y * scaleY;
        const w = Math.max(1, step * scaleX);
        const h = Math.max(1, step * scaleY);
        ctx.fillRect(px, py, w, h);
      }
    }

    // 绘制怪物（红点）
    ctx.fillStyle = '#ff4444';
    for (const [, monster] of state.monsters) {
      if (!monster.isAlive) continue;
      const px = monster.location.x * scaleX;
      const py = monster.location.y * scaleY;
      ctx.beginPath();
      ctx.arc(px, py, TILE_DOT_SIZE, 0, Math.PI * 2);
      ctx.fill();
    }

    // 绘制其他玩家（绿点）
    ctx.fillStyle = '#44ff44';
    for (const [, player] of state.otherPlayers) {
      const px = player.location.x * scaleX;
      const py = player.location.y * scaleY;
      ctx.beginPath();
      ctx.arc(px, py, TILE_DOT_SIZE, 0, Math.PI * 2);
      ctx.fill();
    }

    // 绘制玩家位置（亮绿色大点）
    if (state.player) {
      const px = state.player.location.x * scaleX;
      const py = state.player.location.y * scaleY;

      // 外圈光圈
      ctx.strokeStyle = '#00ff00';
      ctx.lineWidth = 2;
      ctx.beginPath();
      ctx.arc(px, py, 5, 0, Math.PI * 2);
      ctx.stroke();

      // 中心点
      ctx.fillStyle = '#00ff00';
      ctx.beginPath();
      ctx.arc(px, py, 3, 0, Math.PI * 2);
      ctx.fill();
    }
  }, [state]);

  return (
    <Box
      sx={{
        position: 'absolute',
        top: 100,
        right: 8,
        zIndex: 10,
        border: '1px solid rgba(255,255,255,0.2)',
        borderRadius: 1,
        overflow: 'hidden',
      }}
    >
      <canvas
        ref={canvasRef}
        width={MINIMAP_SIZE}
        height={MINIMAP_SIZE}
        style={{ display: 'block' }}
      />
    </Box>
  );
}
