import { useState, useRef, useEffect } from 'react';
import { Box, TextField, IconButton, Typography } from '@mui/material';
import SendIcon from '@mui/icons-material/Send';
import { ChatMessage } from '../types/game';

export interface ChatBoxProps {
  messages: ChatMessage[];
  onSendMessage: (text: string) => void;
}

/**
 * 聊天框组件
 *
 * 固定在左下角，显示消息列表和输入框。
 */
export function ChatBox({ messages, onSendMessage }: ChatBoxProps) {
  const [inputText, setInputText] = useState('');
  const messagesEndRef = useRef<HTMLDivElement>(null);

  // 自动滚动到底部
  useEffect(() => {
    messagesEndRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  const handleSend = () => {
    const text = inputText.trim();
    if (!text) return;
    onSendMessage(text);
    setInputText('');
  };

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      handleSend();
    }
  };

  const getMessageColor = (type: ChatMessage['type']): string => {
    switch (type) {
      case 'system':
        return '#ffcc00';
      case 'whisper':
        return '#ff88ff';
      case 'chat':
      default:
        return '#ffffff';
    }
  };

  return (
    <Box
      sx={{
        position: 'absolute',
        bottom: 60,
        left: 8,
        width: 320,
        maxHeight: 240,
        zIndex: 10,
        display: 'flex',
        flexDirection: 'column',
        bgcolor: 'rgba(0,0,0,0.6)',
        borderRadius: 1,
        overflow: 'hidden',
      }}
    >
      {/* 消息列表 */}
      <Box
        sx={{
          flex: 1,
          overflowY: 'auto',
          px: 1,
          py: 0.5,
          maxHeight: 160,
          '&::-webkit-scrollbar': { width: 4 },
          '&::-webkit-scrollbar-thumb': { bgcolor: 'rgba(255,255,255,0.2)', borderRadius: 2 },
        }}
      >
        {messages.length === 0 && (
          <Typography variant="caption" sx={{ color: 'rgba(255,255,255,0.4)' }}>
            聊天消息将显示在此处
          </Typography>
        )}
        {messages.map((msg) => (
          <Typography
            key={msg.id}
            variant="caption"
            sx={{
              display: 'block',
              color: getMessageColor(msg.type),
              wordBreak: 'break-word',
              lineHeight: 1.4,
              mb: 0.25,
            }}
          >
            {msg.sender && (
              <Typography
                component="span"
                variant="caption"
                sx={{ color: '#88bbff', fontWeight: 'bold', mr: 0.5 }}
              >
                {msg.sender}
              </Typography>
            )}
            {msg.text}
          </Typography>
        ))}
        <div ref={messagesEndRef} />
      </Box>

      {/* 输入框 */}
      <Box sx={{ display: 'flex', alignItems: 'center', gap: 0.5, px: 0.5, pb: 0.5 }}>
        <TextField
          size="small"
          fullWidth
          placeholder="输入聊天信息..."
          value={inputText}
          onChange={(e) => setInputText(e.target.value)}
          onKeyDown={handleKeyDown}
          variant="standard"
          sx={{
            '& .MuiInputBase-input': {
              color: '#ffffff',
              fontSize: 12,
              py: 0.5,
            },
            '& .MuiInputBase-root:before': { borderBottom: 'none' },
            '& .MuiInputBase-root:hover:before': { borderBottom: 'none' },
            '& .MuiInputBase-root:after': { borderBottom: 'none' },
          }}
        />
        <IconButton
          size="small"
          onClick={handleSend}
          sx={{ color: '#88bbff', '&:hover': { color: '#aaddff' } }}
        >
          <SendIcon fontSize="small" />
        </IconButton>
      </Box>
    </Box>
  );
}
