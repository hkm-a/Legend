import { useEffect, useCallback } from 'react';
import { Box, Dialog, DialogTitle, DialogContent, Typography, Button } from '@mui/material';
import { useConnection } from '../hooks/useConnection';
import { useGameWorld } from '../hooks/useGameWorld';
import { GameCanvas } from './GameCanvas';
import { GameHUD } from './GameHUD';
import { ChatBox } from './ChatBox';
import { ActionBar } from './ActionBar';
import { Minimap } from './Minimap';

/**
 * 游戏主界面布局容器
 *
 * 组合所有子组件：
 * - 底层：GameCanvas (PixiJS 全屏场景)
 * - 覆盖层：GameHUD, Minimap, ChatBox, ActionBar, 各面板
 */
export function GamePage() {
  const connection = useConnection();
  const {
    state,
    walk,
    attack,
    pickUp,
    chat,
    toggleInventory,
    toggleCharacter,
    toggleSkill,
    toggleSettings,
    closeAllPanels,
    respawn,
    respawnTown,
  } = useGameWorld(connection);

  // ---- 键盘快捷键 ----
  const handleKeyDown = useCallback(
    (e: KeyboardEvent) => {
      // 方向键 — 行走
      switch (e.key) {
        case 'ArrowUp':
          e.preventDefault();
          walk(0); // Up
          return;
        case 'ArrowDown':
          e.preventDefault();
          walk(4); // Down
          return;
        case 'ArrowLeft':
          e.preventDefault();
          walk(6); // Left
          return;
        case 'ArrowRight':
          e.preventDefault();
          walk(2); // Right
          return;
      }

      // 字母快捷键
      if (!e.ctrlKey && !e.metaKey) {
        switch (e.key.toLowerCase()) {
          case 'b':
            toggleInventory();
            break;
          case 'c':
            toggleCharacter();
            break;
          case 'v':
            toggleSkill();
            break;
          case 'escape':
            if (
              state.isInventoryOpen ||
              state.isCharacterOpen ||
              state.isSkillOpen ||
              state.isSettingsOpen
            ) {
              closeAllPanels();
            } else {
              toggleSettings();
            }
            break;
        }
      }
    },
    [walk, toggleInventory, toggleCharacter, toggleSkill, toggleSettings, closeAllPanels, state.isInventoryOpen, state.isCharacterOpen, state.isSkillOpen, state.isSettingsOpen],
  );

  useEffect(() => {
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [handleKeyDown]);

  // ---- 交互回调 ----
  const handleTileClick = useCallback(
    (x: number, y: number) => {
      // 点击地图格 — 行走
      if (!state.player) return;

      const dx = x - state.player.location.x;
      const dy = y - state.player.location.y;

      // 确定行走方向
      let direction = 4; // 默认向下
      if (dx === 0 && dy < 0) direction = 0; // Up
      else if (dx > 0 && dy < 0) direction = 1; // UpRight
      else if (dx > 0 && dy === 0) direction = 2; // Right
      else if (dx > 0 && dy > 0) direction = 3; // DownRight
      else if (dx === 0 && dy > 0) direction = 4; // Down
      else if (dx < 0 && dy > 0) direction = 5; // DownLeft
      else if (dx < 0 && dy === 0) direction = 6; // Left
      else if (dx < 0 && dy < 0) direction = 7; // UpLeft

      walk(direction);
    },
    [state.player, walk],
  );

  const handleMonsterClick = useCallback(
    (_objectId: number) => {
      // 点击怪物 — 攻击
      if (!state.player) return;
      attack(state.player.direction, 0);
    },
    [state.player, attack],
  );

  const handlePickUp = useCallback(() => {
    // 拾取脚下物品
    if (!state.player) return;
    for (const [, item] of state.groundItems) {
      if (
        item.location.x === state.player.location.x &&
        item.location.y === state.player.location.y
      ) {
        pickUp(item.objectId);
        break;
      }
    }
  }, [state.player, state.groundItems, pickUp]);

  // 空格键拾取
  useEffect(() => {
    const onKeyUp = (e: KeyboardEvent) => {
      if (e.key === ' ' && !e.repeat) {
        e.preventDefault();
        handlePickUp();
      }
    };
    window.addEventListener('keyup', onKeyUp);
    return () => window.removeEventListener('keyup', onKeyUp);
  }, [handlePickUp]);

  return (
    <Box
      sx={{
        width: '100%',
        height: '100%',
        position: 'relative',
        bgcolor: '#000000',
        overflow: 'hidden',
      }}
    >
      {/* 底层：PixiJS 场景 */}
      <GameCanvas
        state={state}
        onTileClick={handleTileClick}
        onMonsterClick={handleMonsterClick}
      />

      {/* 覆盖层：顶部状态条 */}
      <GameHUD
        playerName={state.player?.name ?? ''}
        level={state.level}
        currentHp={state.currentHp}
        maxHp={state.maxHp}
        currentMp={state.currentMp}
        maxMp={state.maxMp}
        experience={state.experience}
        maxExperience={state.maxExperience}
      />

      {/* 覆盖层：小地图 */}
      <Minimap state={state} />

      {/* 覆盖层：聊天框 */}
      <ChatBox messages={state.chatMessages} onSendMessage={chat} />

      {/* 覆盖层：功能按钮 */}
      <ActionBar
        onInventory={toggleInventory}
        onCharacter={toggleCharacter}
        onSkill={toggleSkill}
        onSettings={toggleSettings}
      />

      {/* 背包面板 */}
      <Dialog
        open={state.isInventoryOpen}
        onClose={closeAllPanels}
        maxWidth="sm"
        fullWidth
        PaperProps={{
          sx: {
            bgcolor: 'rgba(20, 20, 30, 0.95)',
            color: '#ffffff',
            border: '1px solid rgba(255,255,255,0.1)',
          },
        }}
      >
        <DialogTitle>背包</DialogTitle>
        <DialogContent>
          {state.inventory.length === 0 ? (
            <Typography variant="body2" color="text.secondary">
              背包为空
            </Typography>
          ) : (
            <Box sx={{ display: 'grid', gridTemplateColumns: 'repeat(4, 1fr)', gap: 1 }}>
              {state.inventory.map((item) => (
                <Box
                  key={item.uid}
                  sx={{
                    p: 1,
                    border: '1px solid rgba(255,255,255,0.15)',
                    borderRadius: 1,
                    textAlign: 'center',
                    cursor: 'pointer',
                    '&:hover': { bgcolor: 'rgba(255,255,255,0.1)' },
                  }}
                >
                  <Typography variant="caption" display="block">
                    {item.name}
                  </Typography>
                  <Typography variant="caption" color="text.secondary">
                    x{item.count}
                  </Typography>
                </Box>
              ))}
            </Box>
          )}
        </DialogContent>
      </Dialog>

      {/* 角色面板 */}
      <Dialog
        open={state.isCharacterOpen}
        onClose={closeAllPanels}
        maxWidth="xs"
        fullWidth
        PaperProps={{
          sx: {
            bgcolor: 'rgba(20, 20, 30, 0.95)',
            color: '#ffffff',
            border: '1px solid rgba(255,255,255,0.1)',
          },
        }}
      >
        <DialogTitle>角色信息</DialogTitle>
        <DialogContent>
          <Typography variant="body2">名称: {state.player?.name ?? '-'}</Typography>
          <Typography variant="body2">等级: {state.level}</Typography>
          <Typography variant="body2">HP: {state.currentHp}/{state.maxHp}</Typography>
          <Typography variant="body2">MP: {state.currentMp}/{state.maxMp}</Typography>
          <Typography variant="body2">经验: {state.experience}/{state.maxExperience}</Typography>
          <Typography variant="body2">金币: {state.gold}</Typography>
        </DialogContent>
      </Dialog>

      {/* 技能面板（占位） */}
      <Dialog
        open={state.isSkillOpen}
        onClose={closeAllPanels}
        maxWidth="xs"
        fullWidth
        PaperProps={{
          sx: {
            bgcolor: 'rgba(20, 20, 30, 0.95)',
            color: '#ffffff',
            border: '1px solid rgba(255,255,255,0.1)',
          },
        }}
      >
        <DialogTitle>技能</DialogTitle>
        <DialogContent>
          <Typography variant="body2" color="text.secondary">
            技能系统开发中...
          </Typography>
        </DialogContent>
      </Dialog>

      {/* 设置面板（占位） */}
      <Dialog
        open={state.isSettingsOpen}
        onClose={closeAllPanels}
        maxWidth="xs"
        fullWidth
        PaperProps={{
          sx: {
            bgcolor: 'rgba(20, 20, 30, 0.95)',
            color: '#ffffff',
            border: '1px solid rgba(255,255,255,0.1)',
          },
        }}
      >
        <DialogTitle>设置</DialogTitle>
        <DialogContent>
          <Typography variant="body2" color="text.secondary">
            设置面板开发中...
          </Typography>
        </DialogContent>
      </Dialog>

      {/* 死亡遮罩 */}
      {state.isDead && (
        <Box sx={{ position: 'fixed', inset: 0, bgcolor: 'rgba(0,0,0,0.7)', display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center', zIndex: 9999 }}>
          <Typography variant="h2" color="error">你已死亡</Typography>
          <Button variant="contained" sx={{mt:2}} onClick={respawn}>原地复活</Button>
          <Button variant="outlined" sx={{mt:1}} onClick={respawnTown}>回城复活</Button>
        </Box>
      )}
    </Box>
  );
}
