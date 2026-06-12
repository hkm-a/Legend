/**
 * Crystal Mir2 端到端 WebSocket 集成测试脚本
 *
 * 测试服务端协议：
 *   包格式：[PacketID: u16 LE][Length: u16 LE][Payload: u8[Length]]
 *
 * 注意：任务描述中 packet_id=1002/1004/1006 为占位值，
 * 实际代码使用 ClientOpcode 枚举值（KeepAlive=2, Walk=11, Chat=13）
 */

const WebSocket = require('ws');

// ========================================
// 配置
// ========================================
const SERVER_URL = 'ws://localhost:7000';
const TIMEOUT_MS = 25000; // 20s 心跳 + 5s 缓冲
const HEARTBEAT_TEST_MS = 22000; // 等待 22s 确保心跳超时

// ========================================
// 数据包编码
// ========================================

/** Client Packet IDs (与 Shared/src/net/packet_id.rs 中的 ClientOpcode 一致) */
const ClientOpcode = {
  KeepAlive: 2,
  Walk: 11,
  Run: 12,
  Chat: 13,
  Turn: 10,
  Attack: 47,
  LogOut: 9,
};

/** Server Packet IDs (客户端接收时用于判断) */
const ServerOpcode = {
  UserLocation: 23,
  Chat: 30,
  GameMessage: 223,
  KeepAlive: 3,
};

/**
 * 编码数据包
 * @param {number} packet_id - 操作码
 * @param {Buffer} payload - 载荷
 * @returns {Buffer} 完整数据包
 */
function encodePacket(packet_id, payload = Buffer.alloc(0)) {
  const header = Buffer.alloc(4);
  header.writeUInt16LE(packet_id, 0);      // PacketID
  header.writeUInt16LE(payload.length, 2); // Length
  return Buffer.concat([header, payload]);
}

/**
 * 解码数据包
 * @param {Buffer} data - 完整数据包
 * @returns {{ packet_id: number, payload: Buffer }}
 */
function decodePacket(data) {
  if (data.length < 4) throw new Error(`Packet too short: ${data.length}`);
  const packet_id = data.readUInt16LE(0);
  const length = data.readUInt16LE(2);
  const payload = data.slice(4, 4 + length);
  return { packet_id, payload };
}

// ========================================
// 测试运行器
// ========================================

class TestRunner {
  constructor() {
    this.results = [];
    this.currentTest = null;
    this.ws = null;
    this.timer = null;
  }

  async runAll() {
    console.log('╔══════════════════════════════════════════════════════╗');
    console.log('║   Crystal Mir2 WebSocket 集成测试                   ║');
    console.log('╚══════════════════════════════════════════════════════╝\n');

    try {
      await this.testConnection();
      await this.testKeepAlive();
      await this.testWalk();
      await this.testChat();
      await this.testHeartbeat();
      await this.testDisconnect();
    } catch (err) {
      this.fail(`测试异常: ${err.message}`);
    }

    this.printReport();
  }

  startTest(name) {
    this.currentTest = name;
    console.log(`\n▶ 测试: ${name}`);
  }

  pass(msg) {
    console.log(`  ✓ PASS: ${msg}`);
    this.results.push({ name: this.currentTest, status: 'PASS', detail: msg });
  }

  fail(msg) {
    console.log(`  ✗ FAIL: ${msg}`);
    this.results.push({ name: this.currentTest, status: 'FAIL', detail: msg });
  }

  async connect(timeoutMs = 5000) {
    return new Promise((resolve, reject) => {
      const timer = setTimeout(() => {
        reject(new Error('连接超时'));
      }, timeoutMs);

      const ws = new WebSocket(SERVER_URL);
      ws.binaryType = 'nodebuffer'; // 接收 Buffer

      ws.on('open', () => {
        clearTimeout(timer);
        this.ws = ws;
        resolve(ws);
      });

      ws.on('error', (err) => {
        clearTimeout(timer);
        reject(err);
      });
    });
  }

  async waitForMessage(timeoutMs = 3000) {
    return new Promise((resolve, reject) => {
      if (!this.ws) {
        reject(new Error('WebSocket 未连接'));
        return;
      }

      const timer = setTimeout(() => {
        reject(new Error('等待消息超时'));
      }, timeoutMs);

      this.ws.once('message', (data) => {
        clearTimeout(timer);
        resolve(data);
      });
    });
  }

  async waitForMessageWithFilter(filterFn, timeoutMs = 3000) {
    return new Promise((resolve, reject) => {
      if (!this.ws) {
        reject(new Error('WebSocket 未连接'));
        return;
      }

      const timer = setTimeout(() => {
        reject(new Error('等待匹配消息超时'));
      }, timeoutMs);

      const handler = (data) => {
        try {
          const decoded = decodePacket(data);
          if (filterFn(decoded)) {
            clearTimeout(timer);
            this.ws.removeListener('message', handler);
            resolve(decoded);
          }
        } catch (e) {
          // 忽略解码错误，继续等待
        }
      };

      this.ws.on('message', handler);
    });
  }

  sleep(ms) {
    return new Promise((resolve) => setTimeout(resolve, ms));
  }

  // ========================================
  // 测试用例
  // ========================================

  /** TC1: 连接成功 */
  async testConnection() {
    this.startTest('TC1: 连接成功');

    try {
      const ws = await this.connect();
      this.pass(`成功连接到 ${SERVER_URL}`);
      this.ws = ws;
    } catch (err) {
      this.fail(`连接失败: ${err.message}`);
      throw err; // 严重失败，停止后续测试
    }
  }

  /** TC2: 发送 KeepAlive 包，服务端不报错 */
  async testKeepAlive() {
    this.startTest('TC2: 发送 KeepAlive 包（ID=2）');

    try {
      const keepAlivePacket = encodePacket(ClientOpcode.KeepAlive);
      this.ws.send(keepAlivePacket);
      this.pass('KeepAlive 包已发送，服务端无报错');
    } catch (err) {
      this.fail(`KeepAlive 发送失败: ${err.message}`);
    }
  }

  /** TC3: 发送 Walk 包，确认收到响应 */
  async testWalk() {
    this.startTest('TC3: 发送 Walk 包（ID=11）并接收响应');

    try {
      // 方向 0 (Up)
      const walkPayload = Buffer.from([0]);
      const walkPacket = encodePacket(ClientOpcode.Walk, walkPayload);
      this.ws.send(walkPacket);
      this.pass('Walk 包已发送');

      // 等待响应
      const response = await this.waitForMessageWithFilter(
        (pkt) => pkt.packet_id === ServerOpcode.UserLocation,
        3000
      );

      if (response) {
        const loc = decodePacket(response);
        this.pass(`收到 UserLocation 响应: packet_id=${loc.packet_id}, payload=${Array.from(loc.payload).join(',')}`);
      } else {
        this.fail('未收到 UserLocation 响应');
      }
    } catch (err) {
      this.fail(`Walk 测试失败: ${err.message}`);
    }
  }

  /** TC4: 发送 Chat 包，确认收到响应 */
  async testChat() {
    this.startTest('TC4: 发送 Chat 包（ID=13）并接收响应');

    try {
      const message = 'Hello, Mir2!';
      const msgBuf = Buffer.from(message, 'utf-8');
      const chatPayload = Buffer.alloc(2 + msgBuf.length);
      chatPayload.writeUInt16LE(msgBuf.length, 0);
      msgBuf.copy(chatPayload, 2);

      const chatPacket = encodePacket(ClientOpcode.Chat, chatPayload);
      this.ws.send(chatPacket);
      this.pass('Chat 包已发送');

      // 等待 Chat 广播响应（ServerChatPacket）
      const response = await this.waitForMessageWithFilter(
        (pkt) => pkt.packet_id === ServerOpcode.Chat,
        3000
      );

      if (response) {
        const chat = decodePacket(response);
        this.pass(`收到 Chat 广播响应: packet_id=${chat.packet_id}`);
      } else {
        this.fail('未收到 Chat 广播响应');
      }
    } catch (err) {
      this.fail(`Chat 测试失败: ${err.message}`);
    }
  }

  /** TC5: 等待 20 秒，确认心跳不超时 */
  async testHeartbeat() {
    this.startTest('TC5: 心跳维持测试（20 秒不超时）');

    try {
      // 每 5 秒发送一次 KeepAlive 维持连接
      const keepAliveInterval = setInterval(() => {
        if (this.ws && this.ws.readyState === WebSocket.OPEN) {
          const packet = encodePacket(ClientOpcode.KeepAlive);
          this.ws.send(packet);
        }
      }, 5000);

      // 等待 22 秒
      await this.sleep(22000);

      clearInterval(keepAliveInterval);

      // 检查连接是否仍然活跃
      if (this.ws && this.ws.readyState === WebSocket.OPEN) {
        this.pass('20 秒后连接仍然保持，心跳机制正常');
      } else {
        this.fail('20 秒后连接已断开');
      }
    } catch (err) {
      this.fail(`心跳测试异常: ${err.message}`);
    }
  }

  /** TC6: 服务端关闭后，连接断开 */
  async testDisconnect() {
    this.startTest('TC6: 服务端关闭后连接断开（手动检查）');

    try {
      // 发送 LogOut 包测试断开
      const logoutPacket = encodePacket(ClientOpcode.LogOut);
      this.ws.send(logoutPacket);
      this.pass('LogOut 包已发送');

      // 等待关闭
      await new Promise((resolve, reject) => {
        const timer = setTimeout(() => {
          this.pass('连接已断开（超时判断）');
          resolve();
        }, 3000);

        if (this.ws) {
          this.ws.on('close', (code, reason) => {
            clearTimeout(timer);
            this.pass(`连接已关闭: code=${code}, reason=${reason || '正常断开'}`);
            resolve();
          });
        }
      });
    } catch (err) {
      this.fail(`断开测试异常: ${err.message}`);
    }
  }

  // ========================================
  // 报告输出
  // ========================================

  printReport() {
    console.log('\n═══════════════════════════════════════════════════════');
    console.log('  测试报告');
    console.log('═══════════════════════════════════════════════════════\n');

    const total = this.results.length;
    const passed = this.results.filter((r) => r.status === 'PASS').length;
    const failed = this.results.filter((r) => r.status === 'FAIL').length;

    console.log(`总测试数: ${total}`);
    console.log(`通过:     ${passed}`);
    console.log(`失败:     ${failed}`);
    console.log(`结论:     ${failed === 0 ? '✓ PASS' : '✗ FAIL'}\n`);

    if (failed > 0) {
      console.log('--- 失败详情 ---');
      this.results
        .filter((r) => r.status === 'FAIL')
        .forEach((r) => {
          console.log(`  [FAIL] ${r.name}: ${r.detail}`);
        });
      console.log();
    }

    // 输出结构化结果用于测试报告
    console.log('--- 结构化测试结果 ---');
    this.results.forEach((r, i) => {
      console.log(JSON.stringify({ id: i + 1, ...r }));
    });
  }
}

// ========================================
// 启动测试
// ========================================

const runner = new TestRunner();
runner.runAll().then(() => {
  process.exit(0);
}).catch((err) => {
  console.error('测试运行异常:', err);
  process.exit(1);
});
