import { useState } from 'react';
import {
  Box,
  Typography,
  TextField,
  Button,
  Alert,
  CircularProgress,
  Link,
  Card,
  CardContent,
  Divider,
  Stack,
} from '@mui/material';

import { ConnectionState } from '../network/types';
import { ConnectionStatus } from './ConnectionStatus';

interface LoginPageProps {
  connectionState: ConnectionState;
  onLogin: (username: string, password: string) => void;
  onRegister: (username: string, password: string) => void;
  onChangePassword: (oldPassword: string, newPassword: string) => void;
  error: string | null;
  clearError: () => void;
}

/**
 * 登录页面组件
 *
 * 包含：游戏标题、连接状态、登录表单、注册表单、修改密码表单
 */
export function LoginPage({
  connectionState,
  onLogin,
  onRegister,
  onChangePassword,
  error,
  clearError,
}: LoginPageProps) {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [showRegister, setShowRegister] = useState(false);
  const [showChangePwd, setShowChangePwd] = useState(false);

  // 注册表单
  const [regUsername, setRegUsername] = useState('');
  const [regPassword, setRegPassword] = useState('');
  const [regConfirm, setRegConfirm] = useState('');
  const [regError, setRegError] = useState('');

  // 改密表单
  const [oldPwd, setOldPwd] = useState('');
  const [newPwd, setNewPwd] = useState('');
  const [confirmPwd, setConfirmPwd] = useState('');
  const [changePwdError, setChangePwdError] = useState('');

  const isConnected = connectionState === ConnectionState.Connected;
  const isLoading = connectionState === ConnectionState.Connecting || connectionState === ConnectionState.Reconnecting;

  const handleLogin = () => {
    if (!username.trim() || !password.trim()) {
      return;
    }
    clearError();
    onLogin(username.trim(), password);
  };

  const handleRegister = () => {
    setRegError('');
    if (!regUsername.trim() || !regPassword.trim() || !regConfirm.trim()) {
      setRegError('请填写所有字段');
      return;
    }
    if (regPassword !== regConfirm) {
      setRegError('两次密码不一致');
      return;
    }
    if (regPassword.length < 3) {
      setRegError('密码至少3个字符');
      return;
    }
    onRegister(regUsername.trim(), regPassword);
    setShowRegister(false);
    setRegUsername('');
    setRegPassword('');
    setRegConfirm('');
  };

  const handleChangePassword = () => {
    setChangePwdError('');
    if (!oldPwd || !newPwd || !confirmPwd) {
      setChangePwdError('请填写所有字段');
      return;
    }
    if (newPwd !== confirmPwd) {
      setChangePwdError('两次新密码不一致');
      return;
    }
    onChangePassword(oldPwd, newPwd);
    setShowChangePwd(false);
    setOldPwd('');
    setNewPwd('');
    setConfirmPwd('');
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      handleLogin();
    }
  };

  return (
    <Box
      sx={{
        width: '100%',
        height: '100%',
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        justifyContent: 'center',
        bgcolor: 'background.default',
        p: 2,
      }}
    >
      <Card sx={{ maxWidth: 420, width: '100%' }}>
        <CardContent sx={{ p: 3 }}>
          {/* 标题和连接状态 */}
          <Stack direction="row" alignItems="center" justifyContent="space-between" sx={{ mb: 2 }}>
            <Typography variant="h4" fontWeight="bold" color="primary">
              CRYSTAL MIR 2
            </Typography>
            <ConnectionStatus state={connectionState} />
          </Stack>

          {isLoading && (
            <Box sx={{ display: 'flex', justifyContent: 'center', my: 2 }}>
              <CircularProgress size={24} />
            </Box>
          )}

          {/* 错误提示 */}
          {error && (
            <Alert severity="error" onClose={clearError} sx={{ mb: 2 }}>
              {error}
            </Alert>
          )}

          {/* 登录表单 */}
          <Stack spacing={2}>
            <TextField
              label="账号"
              variant="outlined"
              size="small"
              fullWidth
              value={username}
              onChange={(e) => setUsername(e.target.value)}
              onKeyDown={handleKeyDown}
              disabled={!isConnected}
              autoFocus
            />
            <TextField
              label="密码"
              type="password"
              variant="outlined"
              size="small"
              fullWidth
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              onKeyDown={handleKeyDown}
              disabled={!isConnected}
            />
            <Button
              variant="contained"
              size="large"
              fullWidth
              onClick={handleLogin}
              disabled={!isConnected || !username.trim() || !password.trim()}
            >
              登录
            </Button>
          </Stack>

          <Divider sx={{ my: 2 }} />

          {/* 操作链接 */}
          <Stack direction="row" spacing={2} justifyContent="center">
            <Link
              component="button"
              variant="body2"
              onClick={() => {
                setShowRegister(!showRegister);
                setShowChangePwd(false);
              }}
              underline="hover"
            >
              {showRegister ? '收起注册' : '注册新账号'}
            </Link>
            <Link
              component="button"
              variant="body2"
              onClick={() => {
                setShowChangePwd(!showChangePwd);
                setShowRegister(false);
              }}
              underline="hover"
            >
              {showChangePwd ? '收起改密' : '修改密码'}
            </Link>
          </Stack>

          {/* 注册表单 */}
          {showRegister && (
            <Box sx={{ mt: 2 }}>
              <Divider sx={{ mb: 2 }} />
              {regError && (
                <Alert severity="error" sx={{ mb: 1 }}>
                  {regError}
                </Alert>
              )}
              <Stack spacing={1.5}>
                <TextField
                  label="新账号"
                  variant="outlined"
                  size="small"
                  fullWidth
                  value={regUsername}
                  onChange={(e) => setRegUsername(e.target.value)}
                  disabled={!isConnected}
                />
                <TextField
                  label="密码"
                  type="password"
                  variant="outlined"
                  size="small"
                  fullWidth
                  value={regPassword}
                  onChange={(e) => setRegPassword(e.target.value)}
                  disabled={!isConnected}
                />
                <TextField
                  label="确认密码"
                  type="password"
                  variant="outlined"
                  size="small"
                  fullWidth
                  value={regConfirm}
                  onChange={(e) => setRegConfirm(e.target.value)}
                  disabled={!isConnected}
                />
                <Button
                  variant="outlined"
                  size="small"
                  fullWidth
                  onClick={handleRegister}
                  disabled={!isConnected}
                >
                  注册
                </Button>
              </Stack>
            </Box>
          )}

          {/* 修改密码表单 */}
          {showChangePwd && (
            <Box sx={{ mt: 2 }}>
              <Divider sx={{ mb: 2 }} />
              {changePwdError && (
                <Alert severity="error" sx={{ mb: 1 }}>
                  {changePwdError}
                </Alert>
              )}
              <Stack spacing={1.5}>
                <TextField
                  label="旧密码"
                  type="password"
                  variant="outlined"
                  size="small"
                  fullWidth
                  value={oldPwd}
                  onChange={(e) => setOldPwd(e.target.value)}
                  disabled={!isConnected}
                />
                <TextField
                  label="新密码"
                  type="password"
                  variant="outlined"
                  size="small"
                  fullWidth
                  value={newPwd}
                  onChange={(e) => setNewPwd(e.target.value)}
                  disabled={!isConnected}
                />
                <TextField
                  label="确认新密码"
                  type="password"
                  variant="outlined"
                  size="small"
                  fullWidth
                  value={confirmPwd}
                  onChange={(e) => setConfirmPwd(e.target.value)}
                  disabled={!isConnected}
                />
                <Button
                  variant="outlined"
                  size="small"
                  fullWidth
                  onClick={handleChangePassword}
                  disabled={!isConnected}
                >
                  修改密码
                </Button>
              </Stack>
            </Box>
          )}
        </CardContent>
      </Card>
    </Box>
  );
}
