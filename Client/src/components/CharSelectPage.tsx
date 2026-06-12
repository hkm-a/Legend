import { useState } from 'react';
import {
  Box,
  Typography,
  Button,
  Card,
  CardContent,
  CardActions,
  Dialog,
  DialogTitle,
  DialogContent,
  DialogActions,
  TextField,
  Radio,
  RadioGroup,
  FormControlLabel,
  FormControl,
  FormLabel,
  Alert,
  Stack,
  Grid,
  Chip,
  IconButton,
  Tooltip,
} from '@mui/material';

import { CharacterInfo } from '../hooks/useGameState';
import { ConnectionState } from '../network/types';
import { ConnectionStatus } from './ConnectionStatus';

interface CharSelectPageProps {
  connectionState: ConnectionState;
  username: string;
  characters: CharacterInfo[];
  onNewCharacter: (name: string, charClass: number, gender: number) => void;
  onDeleteCharacter: (charIndex: number) => void;
  onStartGame: (charIndex: number) => void;
  onLogout: () => void;
  error: string | null;
  clearError: () => void;
}

const CLASS_NAMES: Record<number, string> = {
  0: '战士',
  1: '法师',
  2: '道士',
  3: '刺客',
  4: '弓箭手',
};

const CLASS_ICONS: Record<number, string> = {
  0: '⚔️',
  1: '🔮',
  2: '☯️',
  3: '🗡️',
  4: '🏹',
};

const GENDER_NAMES: Record<number, string> = {
  0: '男',
  1: '女',
};

/**
 * 角色选择页面组件
 *
 * 显示已创建的角色列表，支持创建新角色、删除角色、进入游戏
 */
export function CharSelectPage({
  connectionState,
  username,
  characters,
  onNewCharacter,
  onDeleteCharacter,
  onStartGame,
  onLogout,
  error,
  clearError,
}: CharSelectPageProps) {
  const [createDialogOpen, setCreateDialogOpen] = useState(false);
  const [deleteDialogOpen, setDeleteDialogOpen] = useState(false);
  const [deleteTarget, setDeleteTarget] = useState<number>(-1);
  const [newName, setNewName] = useState('');
  const [newClass, setNewClass] = useState(0);
  const [newGender, setNewGender] = useState(0);
  const [createError, setCreateError] = useState('');

  const isConnected = connectionState === ConnectionState.Connected;

  const handleOpenCreate = () => {
    setNewName('');
    setNewClass(0);
    setNewGender(0);
    setCreateError('');
    setCreateDialogOpen(true);
  };

  const handleCreate = () => {
    if (!newName.trim()) {
      setCreateError('请输入角色名');
      return;
    }
    if (newName.trim().length < 3 || newName.trim().length > 14) {
      setCreateError('角色名长度为 3~14 个字符');
      return;
    }
    onNewCharacter(newName.trim(), newClass, newGender);
    setCreateDialogOpen(false);
  };

  const handleOpenDelete = (index: number) => {
    setDeleteTarget(index);
    setDeleteDialogOpen(true);
  };

  const handleConfirmDelete = () => {
    if (deleteTarget >= 0) {
      onDeleteCharacter(deleteTarget);
    }
    setDeleteDialogOpen(false);
    setDeleteTarget(-1);
  };

  const handleStartGame = (index: number) => {
    onStartGame(index);
  };

  return (
    <Box
      sx={{
        width: '100%',
        height: '100%',
        display: 'flex',
        flexDirection: 'column',
        bgcolor: 'background.default',
        overflow: 'hidden',
      }}
    >
      {/* 顶部栏 */}
      <Box
        sx={{
          p: 1.5,
          borderBottom: 1,
          borderColor: 'divider',
          display: 'flex',
          alignItems: 'center',
          gap: 2,
        }}
      >
        <Typography variant="h6" fontWeight="bold" color="primary" sx={{ mr: 'auto' }}>
          CRYSTAL MIR 2
        </Typography>
        <Typography variant="body2" color="text.secondary">
          欢迎回来，{username}
        </Typography>
        <ConnectionStatus state={connectionState} />
        <Button variant="outlined" size="small" onClick={onLogout} disabled={!isConnected}>
          退出登录
        </Button>
      </Box>

      {/* 错误提示 */}
      {error && (
        <Alert severity="error" onClose={clearError} sx={{ m: 1 }}>
          {error}
        </Alert>
      )}

      {/* 角色列表 */}
      <Box sx={{ flex: 1, overflow: 'auto', p: 2 }}>
        {characters.length === 0 ? (
          <Box
            sx={{
              display: 'flex',
              flexDirection: 'column',
              alignItems: 'center',
              justifyContent: 'center',
              height: '100%',
              color: 'text.secondary',
            }}
          >
            <Typography variant="h6" gutterBottom>
              暂无角色
            </Typography>
            <Typography variant="body2" sx={{ mb: 2 }}>
              创建你的第一个角色来开始冒险吧
            </Typography>
            <Button variant="contained" onClick={handleOpenCreate}>
              + 创建角色
            </Button>
          </Box>
        ) : (
          <Grid container spacing={2}>
            {characters.map((char) => (
              <Grid item xs={12} sm={6} md={4} key={char.index}>
                <Card variant="outlined">
                  <CardContent>
                    <Stack direction="row" alignItems="center" spacing={1} sx={{ mb: 1 }}>
                      <Typography variant="h5">{CLASS_ICONS[char.class] || '❓'}</Typography>
                      <Typography variant="h6" sx={{ flex: 1 }}>
                        {char.name}
                      </Typography>
                      <Chip
                        label={`Lv.${char.level}`}
                        size="small"
                        color="primary"
                        variant="outlined"
                      />
                    </Stack>
                    <Stack direction="row" spacing={1} sx={{ mt: 1 }}>
                      <Chip
                        label={CLASS_NAMES[char.class] || '未知'}
                        size="small"
                        variant="filled"
                        color="secondary"
                      />
                      <Chip
                        label={GENDER_NAMES[char.gender] || '未知'}
                        size="small"
                        variant="outlined"
                      />
                    </Stack>
                  </CardContent>
                  <CardActions sx={{ justifyContent: 'space-between', px: 2, pb: 1.5 }}>
                    <Tooltip title="删除角色">
                      <IconButton
                        size="small"
                        color="error"
                        onClick={() => handleOpenDelete(char.index)}
                      >
                        🗑
                      </IconButton>
                    </Tooltip>
                    <Button
                      variant="contained"
                      size="small"
                      onClick={() => handleStartGame(char.index)}
                    >
                      进入游戏
                    </Button>
                  </CardActions>
                </Card>
              </Grid>
            ))}

            {/* 如果角色少于4个，显示创建按钮 */}
            {characters.length < 4 && (
              <Grid item xs={12} sm={6} md={4}>
                <Card
                  variant="outlined"
                  sx={{
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'center',
                    minHeight: 160,
                    cursor: 'pointer',
                    borderStyle: 'dashed',
                    '&:hover': { bgcolor: 'action.hover' },
                  }}
                  onClick={handleOpenCreate}
                >
                  <Stack alignItems="center" spacing={1}>
                    <Typography variant="h3" color="text.secondary">
                      +
                    </Typography>
                    <Typography variant="body2" color="text.secondary">
                      创建角色
                    </Typography>
                  </Stack>
                </Card>
              </Grid>
            )}
          </Grid>
        )}
      </Box>

      {/* 底部操作栏 */}
      {characters.length > 0 && (
        <Box
          sx={{
            p: 1.5,
            borderTop: 1,
            borderColor: 'divider',
            display: 'flex',
            justifyContent: 'center',
            gap: 2,
          }}
        >
          {characters.length < 4 && (
            <Button variant="outlined" onClick={handleOpenCreate}>
              + 创建角色
            </Button>
          )}
        </Box>
      )}

      {/* 创建角色对话框 */}
      <Dialog open={createDialogOpen} onClose={() => setCreateDialogOpen(false)} maxWidth="xs" fullWidth>
        <DialogTitle>创建角色</DialogTitle>
        <DialogContent>
          <Stack spacing={2} sx={{ mt: 1 }}>
            {createError && <Alert severity="error">{createError}</Alert>}
            <TextField
              label="角色名"
              variant="outlined"
              size="small"
              fullWidth
              value={newName}
              onChange={(e) => setNewName(e.target.value)}
              inputProps={{ maxLength: 14 }}
              autoFocus
            />
            <FormControl>
              <FormLabel>职业</FormLabel>
              <RadioGroup row value={newClass} onChange={(e) => setNewClass(Number(e.target.value))}>
                <FormControlLabel value={0} control={<Radio />} label="⚔️ 战士" />
                <FormControlLabel value={1} control={<Radio />} label="🔮 法师" />
                <FormControlLabel value={2} control={<Radio />} label="☯️ 道士" />
              </RadioGroup>
            </FormControl>
            <FormControl>
              <FormLabel>性别</FormLabel>
              <RadioGroup row value={newGender} onChange={(e) => setNewGender(Number(e.target.value))}>
                <FormControlLabel value={0} control={<Radio />} label="男" />
                <FormControlLabel value={1} control={<Radio />} label="女" />
              </RadioGroup>
            </FormControl>
          </Stack>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setCreateDialogOpen(false)}>取消</Button>
          <Button variant="contained" onClick={handleCreate}>
            创建
          </Button>
        </DialogActions>
      </Dialog>

      {/* 删除角色确认对话框 */}
      <Dialog open={deleteDialogOpen} onClose={() => setDeleteDialogOpen(false)} maxWidth="xs" fullWidth>
        <DialogTitle>确认删除</DialogTitle>
        <DialogContent>
          <Typography>确定要删除该角色吗？此操作不可撤销。</Typography>
        </DialogContent>
        <DialogActions>
          <Button onClick={() => setDeleteDialogOpen(false)}>取消</Button>
          <Button variant="contained" color="error" onClick={handleConfirmDelete}>
            确认删除
          </Button>
        </DialogActions>
      </Dialog>
    </Box>
  );
}
