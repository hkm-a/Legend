using System.Collections.Concurrent;
using System.Net;
using System.Net.Sockets;
using System.Text;
using System.Text.Json;
using System.Text.Json.Serialization;

var options = ServerOptions.Parse(args);
var storePath = Path.GetFullPath(options.StorePath);
Directory.CreateDirectory(Path.GetDirectoryName(storePath) ?? AppContext.BaseDirectory);
var store = CharacterStore.Load(storePath);
var server = new GodotMirServer(options.Host, options.Port, store);
await server.RunAsync();

sealed class GodotMirServer
{
    private static readonly JsonSerializerOptions JsonOptions = new(JsonSerializerDefaults.Web)
    {
        DefaultIgnoreCondition = JsonIgnoreCondition.WhenWritingNull
    };

    private readonly TcpListener _listener;
    private readonly CharacterStore _store;
    private readonly ConcurrentDictionary<long, ClientState> _clients = new();
    private long _nextActorId = 1000;

    public GodotMirServer(string host, int port, CharacterStore store)
    {
        _listener = new TcpListener(IPAddress.Parse(host), port);
        _store = store;
    }

    public async Task RunAsync()
    {
        _listener.Start();
        Console.WriteLine($"GodotMirServer listening on {((IPEndPoint)_listener.LocalEndpoint).Address}:{((IPEndPoint)_listener.LocalEndpoint).Port}");
        while (true)
        {
            var tcp = await _listener.AcceptTcpClientAsync();
            _ = HandleClientAsync(tcp);
        }
    }

    private async Task HandleClientAsync(TcpClient tcp)
    {
        var actorId = Interlocked.Increment(ref _nextActorId);
        var state = new ClientState(actorId, tcp);
        _clients[actorId] = state;
        Console.WriteLine($"client connected actor={actorId} remote={tcp.Client.RemoteEndPoint}");
        try
        {
            using var stream = tcp.GetStream();
            using var reader = new StreamReader(stream, Encoding.UTF8, detectEncodingFromByteOrderMarks: false, leaveOpen: true);
            using var writer = new StreamWriter(stream, new UTF8Encoding(false), leaveOpen: true) { AutoFlush = true, NewLine = "\n" };
            state.Writer = writer;
            await SendAsync(writer, new { type = "hello", message = "GodotMirServer ready" });
            while (await reader.ReadLineAsync() is { } line)
            {
                if (string.IsNullOrWhiteSpace(line))
                {
                    continue;
                }
                await HandleMessageAsync(state, writer, line);
            }
        }
        catch (IOException)
        {
        }
        catch (SocketException)
        {
        }
        catch (Exception ex)
        {
            Console.WriteLine($"client actor={actorId} error: {ex.Message}");
        }
        finally
        {
            _clients.TryRemove(actorId, out _);
            if (state.IsInGame)
            {
                await BroadcastAsync(new { type = "playerLeft", actorId = state.ActorId, character = state.Character }, state.ActorId);
            }
            tcp.Dispose();
            Console.WriteLine($"client disconnected actor={actorId}");
        }
    }

    private async Task HandleMessageAsync(ClientState state, StreamWriter writer, string line)
    {
        ClientRequest? request;
        try
        {
            request = JsonSerializer.Deserialize<ClientRequest>(line, JsonOptions);
        }
        catch (JsonException ex)
        {
            await SendErrorAsync(writer, $"JSON 格式错误: {ex.Message}");
            return;
        }

        if (request is null || string.IsNullOrWhiteSpace(request.Type))
        {
            await SendErrorAsync(writer, "缺少 type");
            return;
        }

        switch (request.Type)
        {
            case "query":
                state.Account = RequireAccount(request.Account);
                await SendCharactersAsync(writer, state.Account);
                break;
            case "create":
                state.Account = RequireAccount(request.Account);
                var name = RequireName(request.Character);
                var created = _store.CreateCharacter(state.Account, name, request.Job, request.Sex, request.Hair);
                if (!created)
                {
                    await SendErrorAsync(writer, $"角色已存在: {name}");
                    break;
                }
                await SendAsync(writer, new { type = "created", character = name });
                await SendCharactersAsync(writer, state.Account);
                break;
            case "enter":
                state.Account = RequireAccount(request.Account);
                var character = RequireName(request.Character);
                var selected = _store.GetOrCreateCharacter(state.Account, character, request.Job, request.Sex, request.Hair);
                state.Character = selected.Name;
                state.X = selected.X;
                state.Y = selected.Y;
                state.Direction = selected.Direction;
                state.Hp = state.MaxHp;
                await SendCharactersAsync(writer, state.Account);
                await SendAsync(writer, new
                {
                    type = "entered",
                    actorId = state.ActorId,
                    character = selected.Name,
                    x = selected.X,
                    y = selected.Y,
                    direction = selected.Direction,
                    hp = state.Hp,
                    maxHp = state.MaxHp,
                    isDead = state.IsDead,
                    players = OnlinePlayersExcept(state.ActorId)
                });
                await BroadcastAsync(new
                {
                    type = "playerJoined",
                    actorId = state.ActorId,
                    character = selected.Name,
                    x = selected.X,
                    y = selected.Y,
                    direction = selected.Direction,
                    hp = state.Hp,
                    maxHp = state.MaxHp,
                    isDead = state.IsDead
                }, state.ActorId);
                break;
            case "move":
                if (!state.IsInGame)
                {
                    await SendErrorAsync(writer, "尚未进入游戏");
                    break;
                }
                if (state.IsDead)
                {
                    await SendErrorAsync(writer, "死亡状态不能移动");
                    break;
                }
                state.X = request.X;
                state.Y = request.Y;
                state.Direction = request.Direction;
                _store.UpdatePosition(state.Account, state.Character, state.X, state.Y, state.Direction);
                var moved = new { type = "moved", actorId = state.ActorId, character = state.Character, x = state.X, y = state.Y, direction = state.Direction };
                await SendAsync(writer, moved);
                await BroadcastAsync(moved, state.ActorId);
                break;
            case "say":
                if (!state.IsInGame)
                {
                    await SendErrorAsync(writer, "尚未进入游戏");
                    break;
                }
                var text = request.Message ?? string.Empty;
                Console.WriteLine($"[{state.Character}] {text}");
                await BroadcastAsync(new { type = "chat", actorId = state.ActorId, from = state.Character, message = text });
                break;
            case "attack":
                if (!state.IsInGame)
                {
                    await SendErrorAsync(writer, "尚未进入游戏");
                    break;
                }
                if (state.IsDead)
                {
                    await SendErrorAsync(writer, "死亡状态不能攻击");
                    break;
                }
                var target = FindTarget(state, request.TargetActorId);
                var damage = target is null || target.IsDead ? 0 : 12;
                if (target is not null && damage > 0)
                {
                    target.Hp = Math.Max(0, target.Hp - damage);
                }
                await BroadcastAsync(new
                {
                    type = "attacked",
                    actorId = state.ActorId,
                    from = state.Character,
                    targetActorId = target?.ActorId ?? 0,
                    target = target?.Character ?? string.Empty,
                    x = state.X,
                    y = state.Y,
                    direction = state.Direction,
                    damage,
                    hp = target?.Hp ?? 0,
                    maxHp = target?.MaxHp ?? 0,
                    isDead = target?.IsDead ?? false
                });
                if (target is not null && target.IsDead)
                {
                    await BroadcastAsync(new { type = "died", actorId = target.ActorId, character = target.Character, hp = target.Hp, maxHp = target.MaxHp });
                }
                break;
            case "revive":
                if (!state.IsInGame)
                {
                    await SendErrorAsync(writer, "尚未进入游戏");
                    break;
                }
                state.Hp = state.MaxHp;
                state.X = 330;
                state.Y = 330;
                state.Direction = 4;
                _store.UpdatePosition(state.Account, state.Character, state.X, state.Y, state.Direction);
                await BroadcastAsync(new { type = "revived", actorId = state.ActorId, character = state.Character, x = state.X, y = state.Y, direction = state.Direction, hp = state.Hp, maxHp = state.MaxHp });
                break;
            default:
                await SendErrorAsync(writer, $"未知消息类型: {request.Type}");
                break;
        }
    }

    private async Task SendCharactersAsync(StreamWriter writer, string account)
    {
        var characters = _store.GetCharacters(account)
            .Select(c => new { name = c.Name, job = c.Job, hair = c.Hair, level = c.Level, sex = c.Sex, selected = false })
            .ToArray();
        await SendAsync(writer, new { type = "characters", characters });
    }

    private object[] OnlinePlayersExcept(long actorId)
    {
        return _clients.Values
            .Where(client => client.IsInGame && client.ActorId != actorId)
            .Select(client => new
            {
                actorId = client.ActorId,
                character = client.Character,
                x = client.X,
                y = client.Y,
                direction = client.Direction,
                hp = client.Hp,
                maxHp = client.MaxHp,
                isDead = client.IsDead
            })
            .ToArray();
    }

    private async Task BroadcastAsync(object message, long exceptActorId = 0)
    {
        var clients = _clients.Values
            .Where(client => client.IsInGame && client.ActorId != exceptActorId && client.Writer is not null)
            .ToArray();
        foreach (var client in clients)
        {
            try
            {
                await SendAsync(client.Writer!, message);
            }
            catch (IOException)
            {
            }
            catch (ObjectDisposedException)
            {
            }
        }
    }

    private ClientState? FindTarget(ClientState attacker, long targetActorId)
    {
        if (targetActorId > 0 && _clients.TryGetValue(targetActorId, out var target) && target.IsInGame && target.ActorId != attacker.ActorId)
        {
            return target;
        }
        return _clients.Values.FirstOrDefault(client => client.IsInGame && client.ActorId != attacker.ActorId);
    }

    private static string RequireAccount(string? value)
    {
        value = value?.Trim();
        if (string.IsNullOrEmpty(value))
        {
            throw new InvalidOperationException("账号不能为空");
        }
        return value;
    }

    private static string RequireName(string? value)
    {
        value = value?.Trim();
        if (string.IsNullOrEmpty(value))
        {
            throw new InvalidOperationException("角色名不能为空");
        }
        return value;
    }

    private static Task SendErrorAsync(StreamWriter writer, string message)
    {
        return SendAsync(writer, new { type = "error", message });
    }

    private static Task SendAsync(StreamWriter writer, object message)
    {
        return writer.WriteLineAsync(JsonSerializer.Serialize(message, JsonOptions));
    }

    private sealed class ClientState(long actorId, TcpClient tcp)
    {
        public long ActorId { get; } = actorId;
        public TcpClient Tcp { get; } = tcp;
        public StreamWriter? Writer { get; set; }
        public string Account { get; set; } = string.Empty;
        public string Character { get; set; } = string.Empty;
        public int X { get; set; }
        public int Y { get; set; }
        public int Direction { get; set; }
        public int Hp { get; set; } = 100;
        public int MaxHp { get; } = 100;
        public bool IsDead => Hp <= 0;
        public bool IsInGame => !string.IsNullOrWhiteSpace(Account) && !string.IsNullOrWhiteSpace(Character);
    }
}

sealed class CharacterStore
{
    private readonly object _sync = new();
    private readonly string _path;
    private StoreData _data;

    private CharacterStore(string path, StoreData data)
    {
        _path = path;
        _data = data;
    }

    public static CharacterStore Load(string path)
    {
        if (!File.Exists(path))
        {
            return new CharacterStore(path, Seed());
        }

        try
        {
            var data = JsonSerializer.Deserialize<StoreData>(File.ReadAllText(path), new JsonSerializerOptions(JsonSerializerDefaults.Web));
            return new CharacterStore(path, data ?? Seed());
        }
        catch
        {
            return new CharacterStore(path, Seed());
        }
    }

    public IReadOnlyList<CharacterRecord> GetCharacters(string account)
    {
        lock (_sync)
        {
            return GetAccount(account).Characters.Select(c => c.Clone()).ToArray();
        }
    }

    public bool CreateCharacter(string account, string name, int job, int sex, int hair)
    {
        lock (_sync)
        {
            var accountData = GetAccount(account);
            if (accountData.Characters.Any(c => string.Equals(c.Name, name, StringComparison.OrdinalIgnoreCase)))
            {
                return false;
            }
            accountData.Characters.Add(NewCharacter(name, job, sex, hair));
            SaveLocked();
            return true;
        }
    }

    public CharacterRecord GetOrCreateCharacter(string account, string name, int job, int sex, int hair)
    {
        lock (_sync)
        {
            var accountData = GetAccount(account);
            var existing = accountData.Characters.FirstOrDefault(c => string.Equals(c.Name, name, StringComparison.OrdinalIgnoreCase));
            if (existing is not null)
            {
                return existing.Clone();
            }
            var created = NewCharacter(name, job, sex, hair);
            accountData.Characters.Add(created);
            SaveLocked();
            return created.Clone();
        }
    }

    public void UpdatePosition(string account, string character, int x, int y, int direction)
    {
        lock (_sync)
        {
            var existing = GetAccount(account).Characters.FirstOrDefault(c => string.Equals(c.Name, character, StringComparison.OrdinalIgnoreCase));
            if (existing is null)
            {
                return;
            }
            existing.X = x;
            existing.Y = y;
            existing.Direction = direction;
            SaveLocked();
        }
    }

    private AccountData GetAccount(string account)
    {
        if (!_data.Accounts.TryGetValue(account, out var accountData))
        {
            accountData = new AccountData();
            _data.Accounts[account] = accountData;
        }
        return accountData;
    }

    private void SaveLocked()
    {
        File.WriteAllText(_path, JsonSerializer.Serialize(_data, new JsonSerializerOptions(JsonSerializerDefaults.Web) { WriteIndented = true }));
    }

    private static StoreData Seed()
    {
        var data = new StoreData();
        data.Accounts["test007"] = new AccountData
        {
            Characters =
            [
                new CharacterRecord
                {
                    Name = "test007",
                    Job = 0,
                    Hair = 0,
                    Level = 1,
                    Sex = 0,
                    X = 330,
                    Y = 330,
                    Direction = 4
                }
            ]
        };
        return data;
    }

    private static CharacterRecord NewCharacter(string name, int job, int sex, int hair)
    {
        return new CharacterRecord
        {
            Name = name,
            Job = Math.Clamp(job, 0, 2),
            Hair = Math.Clamp(hair, 0, 3),
            Level = 1,
            Sex = Math.Clamp(sex, 0, 1),
            X = 330,
            Y = 330,
            Direction = 4
        };
    }
}

sealed class StoreData
{
    public Dictionary<string, AccountData> Accounts { get; set; } = new(StringComparer.OrdinalIgnoreCase);
}

sealed class AccountData
{
    public List<CharacterRecord> Characters { get; set; } = [];
}

sealed class CharacterRecord
{
    public string Name { get; set; } = string.Empty;
    public int Job { get; set; }
    public int Hair { get; set; }
    public int Level { get; set; } = 1;
    public int Sex { get; set; }
    public int X { get; set; }
    public int Y { get; set; }
    public int Direction { get; set; }

    public CharacterRecord Clone()
    {
        return new CharacterRecord
        {
            Name = Name,
            Job = Job,
            Hair = Hair,
            Level = Level,
            Sex = Sex,
            X = X,
            Y = Y,
            Direction = Direction
        };
    }
}

sealed record ClientRequest(
    string? Type,
    string? Account,
    string? Password,
    string? Character,
    int Job,
    int Sex,
    int Hair,
    int X,
    int Y,
    int Direction,
    string? Message,
    long TargetActorId);

sealed record ServerOptions(string Host, int Port, string StorePath)
{
    public static ServerOptions Parse(string[] args)
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

        return new ServerOptions(
            values.GetValueOrDefault("host") ?? "127.0.0.1",
            int.TryParse(values.GetValueOrDefault("port"), out var port) ? port : 7400,
            values.GetValueOrDefault("store") ?? Path.Combine(AppContext.BaseDirectory, "characters.json"));
    }
}
