import { Box, IconButton, Tooltip } from '@mui/material';
import InventoryIcon from '@mui/icons-material/Inventory';
import PersonIcon from '@mui/icons-material/Person';
import BoltIcon from '@mui/icons-material/Bolt';
import SettingsIcon from '@mui/icons-material/Settings';

export interface ActionBarProps {
  onInventory: () => void;
  onCharacter: () => void;
  onSkill: () => void;
  onSettings: () => void;
}

/**
 * 功能按钮栏组件
 *
 * 固定在右下角，包含背包、角色、技能、设置四个按钮。
 */
export function ActionBar({ onInventory, onCharacter, onSkill, onSettings }: ActionBarProps) {
  return (
    <Box
      sx={{
        position: 'absolute',
        bottom: 12,
        right: 12,
        zIndex: 10,
        display: 'flex',
        gap: 1,
        bgcolor: 'rgba(0,0,0,0.5)',
        borderRadius: 2,
        p: 0.5,
      }}
    >
      <Tooltip title="背包 (B)" arrow placement="top">
        <IconButton
          onClick={onInventory}
          sx={{
            color: '#ffffff',
            bgcolor: 'rgba(255,255,255,0.1)',
            '&:hover': { bgcolor: 'rgba(255,255,255,0.2)' },
            width: 40,
            height: 40,
          }}
        >
          <InventoryIcon />
        </IconButton>
      </Tooltip>

      <Tooltip title="角色 (C)" arrow placement="top">
        <IconButton
          onClick={onCharacter}
          sx={{
            color: '#ffffff',
            bgcolor: 'rgba(255,255,255,0.1)',
            '&:hover': { bgcolor: 'rgba(255,255,255,0.2)' },
            width: 40,
            height: 40,
          }}
        >
          <PersonIcon />
        </IconButton>
      </Tooltip>

      <Tooltip title="技能 (V)" arrow placement="top">
        <IconButton
          onClick={onSkill}
          sx={{
            color: '#ffffff',
            bgcolor: 'rgba(255,255,255,0.1)',
            '&:hover': { bgcolor: 'rgba(255,255,255,0.2)' },
            width: 40,
            height: 40,
          }}
        >
          <BoltIcon />
        </IconButton>
      </Tooltip>

      <Tooltip title="设置 (Esc)" arrow placement="top">
        <IconButton
          onClick={onSettings}
          sx={{
            color: '#ffffff',
            bgcolor: 'rgba(255,255,255,0.1)',
            '&:hover': { bgcolor: 'rgba(255,255,255,0.2)' },
            width: 40,
            height: 40,
          }}
        >
          <SettingsIcon />
        </IconButton>
      </Tooltip>
    </Box>
  );
}
