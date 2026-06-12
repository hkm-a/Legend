import { Box, Typography, LinearProgress } from '@mui/material';

export interface GameHUDProps {
  playerName: string;
  level: number;
  currentHp: number;
  maxHp: number;
  currentMp: number;
  maxMp: number;
  experience: number;
  maxExperience: number;
}

/**
 * 顶部状态条组件
 *
 * 显示角色名、等级、HP/MP/经验条。
 */
export function GameHUD({
  playerName,
  level,
  currentHp,
  maxHp,
  currentMp,
  maxMp,
  experience,
  maxExperience,
}: GameHUDProps) {
  const hpPercent = maxHp > 0 ? (currentHp / maxHp) * 100 : 0;
  const mpPercent = maxMp > 0 ? (currentMp / maxMp) * 100 : 0;
  const expPercent = maxExperience > 0 ? (experience / maxExperience) * 100 : 0;

  return (
    <Box
      sx={{
        position: 'absolute',
        top: 0,
        left: 0,
        right: 0,
        zIndex: 10,
        background: 'linear-gradient(180deg, rgba(0,0,0,0.7) 0%, rgba(0,0,0,0.3) 100%)',
        px: 2,
        py: 1,
        display: 'flex',
        flexDirection: 'column',
        gap: 0.5,
        pointerEvents: 'none',
      }}
    >
      {/* 第一行：角色信息 */}
      <Box sx={{ display: 'flex', alignItems: 'center', gap: 2 }}>
        <Typography variant="subtitle2" sx={{ color: '#ffffff', fontWeight: 'bold', minWidth: 80 }}>
          {playerName || '未知'}
        </Typography>
        <Typography variant="caption" sx={{ color: '#ffcc00', minWidth: 40 }}>
          Lv.{level}
        </Typography>
      </Box>

      {/* HP 条 */}
      <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
        <Typography variant="caption" sx={{ color: '#ff4444', width: 20, fontWeight: 'bold' }}>
          HP
        </Typography>
        <LinearProgress
          variant="determinate"
          value={hpPercent}
          sx={{
            flex: 1,
            height: 8,
            borderRadius: 4,
            bgcolor: 'rgba(255,255,255,0.15)',
            '& .MuiLinearProgress-bar': {
              background: 'linear-gradient(90deg, #cc3333, #ff4444)',
              borderRadius: 4,
            },
          }}
        />
        <Typography variant="caption" sx={{ color: '#ffffff', minWidth: 60, textAlign: 'right' }}>
          {currentHp}/{maxHp}
        </Typography>
      </Box>

      {/* MP 条 */}
      <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
        <Typography variant="caption" sx={{ color: '#4488ff', width: 20, fontWeight: 'bold' }}>
          MP
        </Typography>
        <LinearProgress
          variant="determinate"
          value={mpPercent}
          sx={{
            flex: 1,
            height: 8,
            borderRadius: 4,
            bgcolor: 'rgba(255,255,255,0.15)',
            '& .MuiLinearProgress-bar': {
              background: 'linear-gradient(90deg, #3366cc, #4488ff)',
              borderRadius: 4,
            },
          }}
        />
        <Typography variant="caption" sx={{ color: '#ffffff', minWidth: 60, textAlign: 'right' }}>
          {currentMp}/{maxMp}
        </Typography>
      </Box>

      {/* EXP 条 */}
      <Box sx={{ display: 'flex', alignItems: 'center', gap: 1 }}>
        <Typography variant="caption" sx={{ color: '#44cc44', width: 20, fontWeight: 'bold' }}>
          EXP
        </Typography>
        <LinearProgress
          variant="determinate"
          value={expPercent}
          sx={{
            flex: 1,
            height: 4,
            borderRadius: 2,
            bgcolor: 'rgba(255,255,255,0.1)',
            '& .MuiLinearProgress-bar': {
              background: 'linear-gradient(90deg, #33aa33, #44cc44)',
              borderRadius: 2,
            },
          }}
        />
        <Typography variant="caption" sx={{ color: '#ffffff', minWidth: 80, textAlign: 'right' }}>
          {experience}/{maxExperience}
        </Typography>
      </Box>
    </Box>
  );
}
