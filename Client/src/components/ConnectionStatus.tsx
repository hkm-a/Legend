import React from 'react';
import { Chip } from '@mui/material';
import { ConnectionState } from '../network/types';

interface ConnectionStatusProps {
  state: ConnectionState;
}

const stateConfig: Record<ConnectionState, { label: string; color: 'error' | 'warning' | 'success' | 'info' }> = {
  [ConnectionState.Disconnected]: { label: '未连接', color: 'error' },
  [ConnectionState.Connecting]: { label: '连接中...', color: 'warning' },
  [ConnectionState.Connected]: { label: '已连接', color: 'success' },
  [ConnectionState.Reconnecting]: { label: '重连中...', color: 'info' },
};

/**
 * 连接状态指示器
 *
 * 使用 MUI Chip 组件展示 WebSocket 连接状态：
 * - Disconnected → 红色 "未连接"
 * - Connecting → 黄色 "连接中..."
 * - Connected → 绿色 "已连接"
 * - Reconnecting → 蓝色 "重连中..."
 */
export const ConnectionStatus: React.FC<ConnectionStatusProps> = ({ state }) => {
  const config = stateConfig[state];

  return (
    <Chip
      label={config.label}
      color={config.color}
      size="small"
      variant="outlined"
      sx={{ fontWeight: 500 }}
    />
  );
};
