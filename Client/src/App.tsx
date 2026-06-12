import { ThemeProvider, createTheme, CssBaseline, Box, CircularProgress, Typography } from '@mui/material';
import { useEffect } from 'react';
import { useConnection } from './hooks/useConnection';
import { useGameState } from './hooks/useGameState';
import { LoginPage } from './components/LoginPage';
import { CharSelectPage } from './components/CharSelectPage';
import { GamePage } from './components/GamePage';

const theme = createTheme({
  palette: {
    mode: 'dark',
  },
});

/**
 * 加载页面（游戏加载中的过渡画面）
 */
function LoadingPage() {
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
        gap: 2,
      }}
    >
      <CircularProgress size={48} />
      <Typography variant="h6" color="text.secondary">
        正在进入游戏...
      </Typography>
    </Box>
  );
}

/**
 * 断开连接页面
 */
function DisconnectedPage(_props: { onRetry: () => void }) {
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
        gap: 2,
      }}
    >
      <Typography variant="h4" fontWeight="bold" color="primary">
        CRYSTAL MIR 2
      </Typography>
      <Typography variant="body1" color="text.secondary">
        未连接到服务器
      </Typography>
    </Box>
  );
}

function App() {
  const { state: connectionState, connect } = useConnection();
  const {
    state: gameState,
    requestLogin,
    requestRegister,
    requestChangePassword,
    requestNewCharacter,
    requestDeleteCharacter,
    requestStartGame,
    clearError,
    resetToLogin,
  } = useGameState();

  // 页面加载后自动连接
  useEffect(() => {
    connect('ws://localhost:7000');
  }, []);

  const handleLogout = () => {
    resetToLogin();
  };

  // 根据阶段渲染不同页面
  const renderStage = () => {
    switch (gameState.stage) {
      case 'none':
        return <DisconnectedPage onRetry={() => connect('ws://localhost:7000')} />;

      case 'login':
        return (
          <LoginPage
            connectionState={connectionState}
            onLogin={requestLogin}
            onRegister={requestRegister}
            onChangePassword={requestChangePassword}
            error={gameState.error}
            clearError={clearError}
          />
        );

      case 'select_char':
        return (
          <CharSelectPage
            connectionState={connectionState}
            username={gameState.username}
            characters={gameState.characters}
            onNewCharacter={requestNewCharacter}
            onDeleteCharacter={requestDeleteCharacter}
            onStartGame={requestStartGame}
            onLogout={handleLogout}
            error={gameState.error}
            clearError={clearError}
          />
        );

      case 'loading':
        return <LoadingPage />;

      case 'game':
        return <GamePage />;

      default:
        return <DisconnectedPage onRetry={() => connect('ws://localhost:7000')} />;
    }
  };

  return (
    <ThemeProvider theme={theme}>
      <CssBaseline />
      <Box
        sx={{
          width: '100vw',
          height: '100vh',
          display: 'flex',
          flexDirection: 'column',
          overflow: 'hidden',
        }}
      >
        {renderStage()}
      </Box>
    </ThemeProvider>
  );
}

export default App;
