using OpenMir2;
using OpenMir2.Packets.ClientPackets;
using System.Net.Sockets;
using System.Text;

Encoding.RegisterProvider(CodePagesEncodingProvider.Instance);

var options = ClientOptions.Parse(args);
if (string.IsNullOrWhiteSpace(options.Account) || string.IsNullOrWhiteSpace(options.Password))
{
    PrintUsage();
    return 2;
}

try
{
    var client = new MirClient(options);
    await client.RunAsync();
    return 0;
}
catch (SocketException ex)
{
    Console.WriteLine($"连接失败: {ex.Message}");
    return 1;
}
catch (IOException ex)
{
    Console.WriteLine($"网络读写失败: {ex.Message}");
    return 1;
}
catch (InvalidDataException ex)
{
    Console.WriteLine($"协议解析失败: {ex.Message}");
    return 1;
}

static void PrintUsage()
{
    Console.WriteLine("用法:");
    Console.WriteLine("  dotnet run --project MinimalMirClient -- --account test001 --password 123456 [--action query]");
    Console.WriteLine("动作:");
    Console.WriteLine("  login     登录并选择服务器，输出 SelGate/session");
    Console.WriteLine("  query     查询角色列表，默认动作");
    Console.WriteLine("  create    创建角色，需要 --character 名字，可配 --job 0|1|2 --sex 0|1 --hair 0..3");
    Console.WriteLine("  delete    删除角色，需要 --character 名字");
    Console.WriteLine("  select    选择角色并获取 RunGate，需要 --character 名字");
    Console.WriteLine("  play      选择角色并连接 RunGate 发送登录串，需要 --character 名字");
    Console.WriteLine("  say       进游戏后发送聊天，需要 --character 名字 --message 内容");
    Console.WriteLine("  turn      进游戏后转向，需要 --character 名字 --dir 0..7");
    Console.WriteLine("  walk/run  进游戏后移动，需要 --character 名字 --x 坐标 --y 坐标 --dir 0..7");
    Console.WriteLine("  register  注册账号后继续登录");
    Console.WriteLine("常用参数:");
    Console.WriteLine("  --host 127.0.0.1 --port 7000 --server ServerName --sel-host 127.0.0.1 --sel-port 7100 --run-host 127.0.0.1 --run-port 7200");
}

sealed class MirClient
{
    private readonly ClientOptions _options;

    public MirClient(ClientOptions options)
    {
        _options = options;
    }

    public async Task RunAsync()
    {
        switch (_options.Action)
        {
            case ClientAction.Register:
                await RegisterAsync();
                break;
            case ClientAction.Login:
                using (await LoginAndSelectServerAsync())
                {
                }
                break;
            case ClientAction.Query:
                await QueryCharactersAsync();
                break;
            case ClientAction.Create:
                await CreateCharacterAsync();
                break;
            case ClientAction.Delete:
                await DeleteCharacterAsync();
                break;
            case ClientAction.Select:
                await SelectCharacterAsync(connectRunGate: false);
                break;
            case ClientAction.Play:
            case ClientAction.Say:
            case ClientAction.Turn:
            case ClientAction.Walk:
            case ClientAction.Run:
                await SelectCharacterAsync(connectRunGate: true);
                break;
        }
    }

    private async Task RegisterAsync()
    {
        using var login = await MirGateConnection.ConnectAsync(_options.Host, _options.Port);
        Console.WriteLine($"已连接 LoginGate {_options.Host}:{_options.Port}");
        await Task.Delay(1100);
        await login.SendCommandAsync(Messages.CM_ADDNEWUSER, body: BuildRegisterBody());
        var result = await login.ReadMessageAsync();
        PrintMessage("账号注册", result);
        if (result.Command.Ident != Messages.SM_NEWID_SUCCESS && result.Command.Ident != Messages.SM_NEWID_FAIL)
        {
            throw new InvalidDataException($"账号注册返回异常: {result.Command.Ident}");
        }
        if (result.Command.Ident == Messages.SM_NEWID_FAIL && result.Command.Recog != 0)
        {
            throw new InvalidDataException($"账号注册失败，错误码: {result.Command.Recog}");
        }
    }

    private async Task<LoginContext> LoginAndSelectServerAsync()
    {
        var login = await MirGateConnection.ConnectAsync(_options.Host, _options.Port);
        Console.WriteLine($"已连接 LoginGate {_options.Host}:{_options.Port}");
        await Task.Delay(1100);

        await login.SendCommandAsync(Messages.CM_PROTOCOL, _options.ProtocolVersion);
        var protocol = await login.ReadMessageAsync();
        PrintMessage("协议检查", protocol);
        if (protocol.Command.Ident != Messages.SM_CERTIFICATION_SUCCESS)
        {
            throw new InvalidDataException($"协议版本不通过，Ident={protocol.Command.Ident}");
        }

        await login.SendCommandAsync(Messages.CM_IDPASSWORD, body: EDCode.EncodeString($"{_options.Account}/{_options.Password}"));
        var loginResult = await login.ReadMessageAsync();
        PrintMessage("账号登录", loginResult);
        if (loginResult.Command.Ident == Messages.SM_PASSWD_FAIL && loginResult.Command.Recog == -3)
        {
            await Task.Delay(5500);
            await login.SendCommandAsync(Messages.CM_IDPASSWORD, body: EDCode.EncodeString($"{_options.Account}/{_options.Password}"));
            loginResult = await login.ReadMessageAsync();
            PrintMessage("账号登录重试", loginResult);
        }
        if (loginResult.Command.Ident != Messages.SM_PASSOK_SELECTSERVER)
        {
            throw new InvalidDataException($"账号登录失败，Ident={loginResult.Command.Ident}, Code={loginResult.Command.Recog}");
        }

        var serverList = DecodeBody(loginResult.Body);
        Console.WriteLine($"服务器列表: {serverList}");
        var serverName = _options.ServerName ?? FirstServerName(serverList);
        if (string.IsNullOrWhiteSpace(serverName))
        {
            throw new InvalidDataException("没有可选服务器。");
        }

        await login.SendCommandAsync(Messages.CM_SELECTSERVER, body: EDCode.EncodeString(serverName));
        var selectServer = await login.ReadMessageAsync();
        PrintMessage("选择服务器", selectServer);
        if (selectServer.Command.Ident != Messages.SM_SELECTSERVER_OK)
        {
            throw new InvalidDataException($"选服失败，Ident={selectServer.Command.Ident}, Code={selectServer.Command.Recog}");
        }

        var selectBody = DecodeBody(selectServer.Body);
        var selectInfo = selectBody.Split('/', StringSplitOptions.RemoveEmptyEntries);
        if (selectInfo.Length < 3 || !int.TryParse(selectInfo[1], out var selPort) || !int.TryParse(selectInfo[2], out var sessionId))
        {
            throw new InvalidDataException($"选服返回格式异常: {selectBody}");
        }

        var selHost = _options.SelHost ?? selectInfo[0];
        selPort = _options.SelPort ?? selPort;
        sessionId = selectServer.Command.Recog > 0 ? selectServer.Command.Recog : sessionId;
        Console.WriteLine($"SelGate: {selHost}:{selPort}, SessionID={sessionId}");
        return new LoginContext(serverName, selHost, selPort, sessionId, login);
    }

    private async Task<CharacterList> QueryCharactersAsync()
    {
        using var context = await LoginAndSelectServerAsync();
        using var sel = await ConnectSelGateAsync(context);
        await sel.SendCommandAsync(Messages.CM_QUERYCHR, body: EDCode.EncodeString($"{_options.Account}/{context.SessionId}"));
        var result = await sel.ReadMessageAsync();
        PrintMessage("查询角色", result);
        if (result.Command.Ident != Messages.SM_QUERYCHR)
        {
            throw new InvalidDataException($"查询角色失败，Ident={result.Command.Ident}, Code={result.Command.Recog}");
        }

        var list = CharacterList.Parse(DecodeBody(result.Body));
        if (list.Characters.Count == 0)
        {
            Console.WriteLine("角色列表为空。");
        }
        else
        {
            foreach (var character in list.Characters)
            {
                Console.WriteLine($"角色: {character.Name}, Job={character.Job}, Hair={character.Hair}, Level={character.Level}, Sex={character.Sex}, Selected={character.Selected}");
            }
        }
        return list;
    }

    private async Task CreateCharacterAsync()
    {
        if (string.IsNullOrWhiteSpace(_options.CharacterName))
        {
            throw new InvalidDataException("create 动作需要 --character 名字。");
        }

        using var context = await LoginAndSelectServerAsync();
        using var sel = await ConnectSelGateAsync(context);
        await sel.SendCommandAsync(Messages.CM_QUERYCHR, body: EDCode.EncodeString($"{_options.Account}/{context.SessionId}"));
        var query = await sel.ReadMessageAsync();
        PrintMessage("查询角色", query);
        await Task.Delay(1100);
        var body = $"{_options.Account}/{_options.CharacterName}/{_options.Hair}/{_options.Job}/{_options.Sex}";
        await sel.SendCommandAsync(Messages.CM_NEWCHR, body: EDCode.EncodeString(body));
        var result = await sel.ReadMessageAsync();
        PrintMessage("创建角色", result);
        if (result.Command.Ident != Messages.SM_NEWCHR_SUCCESS)
        {
            throw new InvalidDataException($"创建角色失败，Ident={result.Command.Ident}, Code={result.Command.Recog}");
        }
        Console.WriteLine($"角色创建成功: {_options.CharacterName}");
    }

    private async Task DeleteCharacterAsync()
    {
        if (string.IsNullOrWhiteSpace(_options.CharacterName))
        {
            throw new InvalidDataException("delete 动作需要 --character 名字。");
        }

        using var context = await LoginAndSelectServerAsync();
        using var sel = await ConnectSelGateAsync(context);
        await sel.SendCommandAsync(Messages.CM_QUERYCHR, body: EDCode.EncodeString($"{_options.Account}/{context.SessionId}"));
        var query = await sel.ReadMessageAsync();
        PrintMessage("查询角色", query);
        await Task.Delay(1100);

        await sel.SendCommandAsync(Messages.CM_DELCHR, body: EDCode.EncodeString(_options.CharacterName));
        var result = await sel.ReadMessageAsync();
        PrintMessage("删除角色", result);
        if (result.Command.Ident != Messages.SM_DELCHR_SUCCESS)
        {
            throw new InvalidDataException($"删除角色失败，Ident={result.Command.Ident}, Code={result.Command.Recog}");
        }
        Console.WriteLine($"角色删除成功: {_options.CharacterName}");
    }

    private async Task SelectCharacterAsync(bool connectRunGate)
    {
        if (string.IsNullOrWhiteSpace(_options.CharacterName))
        {
            throw new InvalidDataException("select/play/say/turn/walk/run 动作需要 --character 名字。");
        }

        using var context = await LoginAndSelectServerAsync();
        using var sel = await ConnectSelGateAsync(context);
        await sel.SendCommandAsync(Messages.CM_QUERYCHR, body: EDCode.EncodeString($"{_options.Account}/{context.SessionId}"));
        var query = await sel.ReadMessageAsync();
        PrintMessage("查询角色", query);
        var characters = CharacterList.Parse(DecodeBody(query.Body));
        if (!characters.Characters.Any(c => string.Equals(c.Name, _options.CharacterName, StringComparison.OrdinalIgnoreCase)))
        {
            throw new InvalidDataException($"角色不存在: {_options.CharacterName}");
        }

        await sel.SendCommandAsync(Messages.CM_SELCHR, body: EDCode.EncodeString($"{_options.Account}/{_options.CharacterName}"));
        var start = await sel.ReadMessageAsync();
        PrintMessage("选择角色", start);
        if (start.Command.Ident != Messages.SM_STARTPLAY)
        {
            throw new InvalidDataException($"选择角色失败，Ident={start.Command.Ident}, Code={start.Command.Recog}");
        }

        var route = DecodeBody(start.Body).Split('/', StringSplitOptions.RemoveEmptyEntries);
        if (route.Length < 2 || !int.TryParse(route[1], out var runPort))
        {
            throw new InvalidDataException($"RunGate 返回格式异常: {DecodeBody(start.Body)}");
        }

        var runHost = _options.RunHost ?? route[0];
        runPort = _options.RunPort ?? runPort;
        Console.WriteLine($"RunGate: {runHost}:{runPort}");
        if (!connectRunGate)
        {
            return;
        }

        using var run = await MirGateConnection.ConnectAsync(runHost, runPort, acknowledgeProbe: true);
        Console.WriteLine($"已连接 RunGate {runHost}:{runPort}");
        await Task.Delay(1100);
        var loginText = $"**{_options.Account}/{_options.CharacterName}/{context.SessionId}/{Grobal2.ClientVersionNumber}/2022080300";
        await run.SendRawBodyAsync(EDCode.EncodeString(loginText));
        await ProcessGameMessagesAsync(run);
        await SendGameActionAsync(run);
    }

    private async Task ProcessGameMessagesAsync(MirGateConnection run)
    {
        using var timeout = new CancellationTokenSource(TimeSpan.FromSeconds(_options.ReadSeconds));
        var logon = false;
        while (!timeout.IsCancellationRequested)
        {
            MirMessage message;
            try
            {
                message = await run.ReadMessageAsync(timeout.Token);
            }
            catch (OperationCanceledException)
            {
                break;
            }

            PrintMessage("游戏消息", message);
            if (message.Body.Length > 0)
            {
                Console.WriteLine($"游戏消息 Body: {SafeDecodeBody(message.Body)}");
            }
            if (message.Command.Ident == Messages.SM_SENDNOTICE)
            {
                await run.SendCommandAsync(Messages.CM_LOGINNOTICEOK, Environment.TickCount);
                Console.WriteLine("已确认登录公告。");
            }
            else if (message.Command.Ident == Messages.SM_LOGON)
            {
                logon = true;
                Console.WriteLine($"成功进入游戏: ActorId={message.Command.Recog}, X={message.Command.Param}, Y={message.Command.Tag}, DirLight={message.Command.Series}");
                break;
            }
        }

        if (!logon)
        {
            Console.WriteLine("在读取窗口内未收到 SM_LOGON。可以用 --read-seconds 增加等待时间。");
        }
    }

    private async Task SendGameActionAsync(MirGateConnection run)
    {
        switch (_options.Action)
        {
            case ClientAction.Say:
                if (string.IsNullOrWhiteSpace(_options.Message))
                {
                    throw new InvalidDataException("say 动作需要 --message 内容。");
                }
                await run.SendCommandAsync(Messages.CM_SAY, body: EDCode.EncodeString(_options.Message));
                Console.WriteLine($"已发送聊天: {_options.Message}");
                break;
            case ClientAction.Turn:
                await run.SendCommandAsync(Messages.CM_TURN, MakeCoordRecog(_options.X, _options.Y), tag: _options.Direction);
                Console.WriteLine($"已发送转向: X={_options.X}, Y={_options.Y}, Dir={_options.Direction}");
                break;
            case ClientAction.Walk:
                await run.SendCommandAsync(Messages.CM_WALK, MakeCoordRecog(_options.X, _options.Y), tag: _options.Direction);
                Console.WriteLine($"已发送走路: X={_options.X}, Y={_options.Y}, Dir={_options.Direction}");
                break;
            case ClientAction.Run:
                await run.SendCommandAsync(Messages.CM_RUN, MakeCoordRecog(_options.X, _options.Y), tag: _options.Direction);
                Console.WriteLine($"已发送跑步: X={_options.X}, Y={_options.Y}, Dir={_options.Direction}");
                break;
        }
    }

    private static int MakeCoordRecog(ushort x, ushort y)
    {
        return HUtil32.MakeLong(x, y);
    }

    private static async Task<MirGateConnection> ConnectSelGateAsync(LoginContext context)
    {
        var sel = await MirGateConnection.ConnectAsync(context.SelHost, context.SelPort);
        Console.WriteLine($"已连接 SelGate {context.SelHost}:{context.SelPort}");
        await Task.Delay(1100);
        return sel;
    }

    private string BuildRegisterBody()
    {
        var ue = new UserEntry
        {
            Account = _options.Account ?? string.Empty,
            Password = _options.Password ?? string.Empty,
            UserName = _options.Account ?? string.Empty,
            SSNo = "650101-1455111",
            Phone = string.Empty,
            Quiz = _options.Account ?? string.Empty,
            Answer = _options.Account ?? string.Empty,
            EMail = string.Empty
        };
        var ua = new UserEntryAdd
        {
            Quiz2 = _options.Account ?? string.Empty,
            Answer2 = _options.Account ?? string.Empty,
            BirthDay = "1978/01/01",
            MobilePhone = string.Empty,
            Memo = string.Empty,
            Memo2 = string.Empty
        };
        return EDCode.EncodeBuffer(ue) + EDCode.EncodeBuffer(ua);
    }

    private static void PrintMessage(string title, MirMessage message)
    {
        Console.WriteLine($"{title}: Ident={message.Command.Ident}, Recog={message.Command.Recog}, Param={message.Command.Param}, Tag={message.Command.Tag}, Series={message.Command.Series}, BodyLength={message.Body.Length}");
    }

    private static string DecodeBody(string body)
    {
        return string.IsNullOrEmpty(body) ? string.Empty : EDCode.DeCodeString(body);
    }

    private static string SafeDecodeBody(string body)
    {
        try
        {
            return DecodeBody(body);
        }
        catch
        {
            return body;
        }
    }

    private static string? FirstServerName(string serverList)
    {
        return serverList.Split('/', StringSplitOptions.RemoveEmptyEntries).FirstOrDefault();
    }
}

sealed class MirGateConnection : IDisposable
{
    private readonly TcpClient _client;
    private readonly NetworkStream _stream;
    private readonly List<byte> _buffer = new();
    private readonly Encoding _encoding = Encoding.GetEncoding("gb2312");
    private readonly bool _acknowledgeProbe;
    private byte _sendNumber = 1;

    private MirGateConnection(TcpClient client, bool acknowledgeProbe)
    {
        _client = client;
        _stream = client.GetStream();
        _acknowledgeProbe = acknowledgeProbe;
    }

    public static async Task<MirGateConnection> ConnectAsync(string host, int port, bool acknowledgeProbe = false)
    {
        var client = new TcpClient();
        await client.ConnectAsync(host, port);
        return new MirGateConnection(client, acknowledgeProbe);
    }

    public Task SendCommandAsync(int ident, int recog = 0, ushort param = 0, ushort tag = 0, ushort series = 0, string body = "")
    {
        var command = new CommandMessage
        {
            Recog = recog,
            Ident = (ushort)ident,
            Param = param,
            Tag = tag,
            Series = series
        };
        return SendRawBodyAsync(EDCode.EncodeMessage(command) + body);
    }

    public async Task SendRawBodyAsync(string body)
    {
        var payload = $"#{_sendNumber}{body}!";
        _sendNumber++;
        if (_sendNumber >= 10)
        {
            _sendNumber = 1;
        }
        var bytes = _encoding.GetBytes(payload);
        await _stream.WriteAsync(bytes);
        await _stream.FlushAsync();
    }

    public async Task<MirMessage> ReadMessageAsync(CancellationToken cancellationToken = default)
    {
        while (true)
        {
            var existing = TryTakeFrame();
            if (existing is not null)
            {
                return DecodeFrame(existing);
            }

            var bytes = new byte[4096];
            var count = await _stream.ReadAsync(bytes, cancellationToken);
            if (count == 0)
            {
                throw new IOException("远端已关闭连接。");
            }
            await AcknowledgeRunGateProbeAsync(bytes.AsMemory(0, count), cancellationToken);
            _buffer.AddRange(bytes.AsSpan(0, count).ToArray());
        }
    }

    private async Task AcknowledgeRunGateProbeAsync(ReadOnlyMemory<byte> bytes, CancellationToken cancellationToken)
    {
        if (_acknowledgeProbe && bytes.Span.Contains((byte)'*'))
        {
            await _stream.WriteAsync(new byte[] { (byte)'*' }, cancellationToken);
            await _stream.FlushAsync(cancellationToken);
        }
    }

    private string? TryTakeFrame()
    {
        var start = _buffer.IndexOf((byte)'#');
        if (start < 0)
        {
            _buffer.Clear();
            return null;
        }
        if (start > 0)
        {
            _buffer.RemoveRange(0, start);
        }

        var end = _buffer.IndexOf((byte)'!');
        if (end < 0)
        {
            return null;
        }

        var frameBytes = _buffer.GetRange(1, end - 1).ToArray();
        var removeCount = end + 1;
        while (_buffer.Count > removeCount && _buffer[removeCount] == (byte)'$')
        {
            removeCount++;
        }
        _buffer.RemoveRange(0, removeCount);
        return _encoding.GetString(frameBytes);
    }

    private static MirMessage DecodeFrame(string frame)
    {
        if (frame.StartsWith('+'))
        {
            return new MirMessage(default, frame);
        }
        if (frame.Length < Messages.DefBlockSize)
        {
            throw new InvalidDataException($"回包长度不足: {frame.Length}");
        }

        var command = EDCode.DecodePacket(frame[..Messages.DefBlockSize]);
        var body = frame[Messages.DefBlockSize..];
        return new MirMessage(command, body);
    }

    public void Dispose()
    {
        _stream.Dispose();
        _client.Dispose();
    }
}

sealed record LoginContext(string ServerName, string SelHost, int SelPort, int SessionId, MirGateConnection LoginConnection) : IDisposable
{
    public void Dispose()
    {
        LoginConnection.Dispose();
    }
}
readonly record struct MirMessage(CommandMessage Command, string Body);
readonly record struct CharacterInfo(string Name, byte Job, byte Hair, int Level, byte Sex, bool Selected);

sealed class CharacterList
{
    public List<CharacterInfo> Characters { get; } = new();

    public static CharacterList Parse(string body)
    {
        var result = new CharacterList();
        if (string.IsNullOrWhiteSpace(body))
        {
            return result;
        }

        var parts = body.Split('/', StringSplitOptions.RemoveEmptyEntries);
        for (var i = 0; i + 4 < parts.Length; i += 5)
        {
            var name = parts[i];
            var selected = name.StartsWith('*');
            if (selected)
            {
                name = name[1..];
            }
            if (byte.TryParse(parts[i + 1], out var job) &&
                byte.TryParse(parts[i + 2], out var hair) &&
                int.TryParse(parts[i + 3], out var level) &&
                byte.TryParse(parts[i + 4], out var sex))
            {
                result.Characters.Add(new CharacterInfo(name, job, hair, level, sex, selected));
            }
        }
        return result;
    }
}

enum ClientAction
{
    Login,
    Query,
    Create,
    Delete,
    Select,
    Play,
    Say,
    Turn,
    Walk,
    Run,
    Register
}

sealed record ClientOptions(
    string Host,
    int Port,
    string? Account,
    string? Password,
    string? ServerName,
    string? SelHost,
    int? SelPort,
    string? RunHost,
    int? RunPort,
    string? CharacterName,
    string? Message,
    ushort X,
    ushort Y,
    ushort Direction,
    int ReadSeconds,
    byte Job,
    byte Sex,
    byte Hair,
    int ProtocolVersion,
    ClientAction Action)
{
    public static ClientOptions Parse(string[] args)
    {
        var values = new Dictionary<string, string>(StringComparer.OrdinalIgnoreCase);
        for (var i = 0; i < args.Length; i++)
        {
            if (!args[i].StartsWith("--", StringComparison.Ordinal))
            {
                continue;
            }
            var key = args[i][2..];
            if (i + 1 < args.Length && !args[i + 1].StartsWith("--", StringComparison.Ordinal))
            {
                values[key] = args[++i];
            }
        }

        return new ClientOptions(
            values.GetValueOrDefault("host") ?? "127.0.0.1",
            int.TryParse(values.GetValueOrDefault("port"), out var port) ? port : 7000,
            values.GetValueOrDefault("account"),
            values.GetValueOrDefault("password"),
            values.GetValueOrDefault("server"),
            values.GetValueOrDefault("sel-host"),
            int.TryParse(values.GetValueOrDefault("sel-port"), out var selPort) ? selPort : null,
            values.GetValueOrDefault("run-host"),
            int.TryParse(values.GetValueOrDefault("run-port"), out var runPort) ? runPort : null,
            values.GetValueOrDefault("character"),
            values.GetValueOrDefault("message"),
            ushort.TryParse(values.GetValueOrDefault("x"), out var x) ? x : (ushort)0,
            ushort.TryParse(values.GetValueOrDefault("y"), out var y) ? y : (ushort)0,
            ushort.TryParse(values.GetValueOrDefault("dir"), out var dir) ? dir : (ushort)0,
            int.TryParse(values.GetValueOrDefault("read-seconds"), out var readSeconds) ? readSeconds : 10,
            byte.TryParse(values.GetValueOrDefault("job"), out var job) ? job : (byte)0,
            byte.TryParse(values.GetValueOrDefault("sex"), out var sex) ? sex : (byte)0,
            byte.TryParse(values.GetValueOrDefault("hair"), out var hair) ? hair : (byte)0,
            int.TryParse(values.GetValueOrDefault("version"), out var version) ? version : 20011006,
            ParseAction(values.GetValueOrDefault("action")));
    }

    private static ClientAction ParseAction(string? value)
    {
        return value?.ToLowerInvariant() switch
        {
            "login" => ClientAction.Login,
            "create" => ClientAction.Create,
            "delete" => ClientAction.Delete,
            "select" => ClientAction.Select,
            "play" => ClientAction.Play,
            "say" => ClientAction.Say,
            "turn" => ClientAction.Turn,
            "walk" => ClientAction.Walk,
            "run" => ClientAction.Run,
            "register" => ClientAction.Register,
            _ => ClientAction.Query
        };
    }
}
