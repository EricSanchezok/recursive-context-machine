# MCP Server 配置

MCP (Model Context Protocol) 让 agent 能通过标准化的对外接口调用外部工具——搜索、查数据库、操作文件系统等。RICA 通过 `--mcp-server` 参数配置，支持 stdio 子进程和 Streamable HTTP 两种传输方式。

## 快速开始

```bash
# Stdio 子进程（MCPWorld 上最常见的类型）
accelerate run "抓取这个网页内容" \
  --mcp-server "fetch=uvx mcp-server-fetch"

# HTTP 远程服务器（如 AnySearch 搜索）
accelerate run "搜索量子计算最新进展" \
  --mcp-server "search=https://api.anysearch.com/mcp|Authorization:Bearer <api-key>"
```

---

## 从标准配置到 RICA CLI

大多数 MCP 服务器在文档中展示的都是 Claude Desktop 的 JSON 配置格式。映射到 RICA 很简单：`command` + `args` 用空格拼起来就行。

### Basic — uvx

```json
{
  "mcpServers": {
    "fetch": {
      "command": "uvx",
      "args": ["mcp-server-fetch"]
    }
  }
}
```

```bash
--mcp-server "fetch=uvx mcp-server-fetch"
```

### npx

```json
{
  "mcpServers": {
    "github": {
      "command": "npx",
      "args": ["-y", "@modelcontextprotocol/server-github"]
    }
  }
}
```

```bash
--mcp-server "github=npx -y @modelcontextprotocol/server-github"
```

### Python

```json
{
  "mcpServers": {
    "fetch": {
      "command": "python",
      "args": ["-m", "mcp_server_fetch"]
    }
  }
}
```

```bash
--mcp-server "fetch=python -m mcp_server_fetch"
```

### Docker

```json
{
  "mcpServers": {
    "fetch": {
      "command": "docker",
      "args": ["run", "-i", "--rm", "mcp/fetch"]
    }
  }
}
```

```bash
--mcp-server "fetch=docker run -i --rm mcp/fetch"
```

### HTTP 远程（带自定义头部）

```json
{
  "mcpServers": {
    "anysearch": {
      "type": "streamable-http",
      "url": "https://api.anysearch.com/mcp",
      "headers": {
        "Authorization": "Bearer <api-key>"
      }
    }
  }
}
```

```bash
--mcp-server "anysearch=https://api.anysearch.com/mcp|Authorization:Bearer <api-key>"
```

---

## 语法

```
--mcp-server "label=command arg1 arg2"
--mcp-server "label=https://url|HeaderName:Value"
```

| 部分 | 说明 |
|------|------|
| `label=` | 服务标识名，用于日志和调试 |
| `command` | 子进程可执行文件（`uvx`、`npx`、`python`、`docker` 等） |
| `arg1 arg2` | 传给子进程的参数，**空格分隔** |
| `https://url` | HTTP MCP 端点（以 `http://` 或 `https://` 开头） |
| `HeaderName:Value` | HTTP 请求头，多个用 `|` 分隔 |

> **为什么用 `|` 而不是 `,`？** URL 和 HTTP header value 中可能含有逗号和冒号，用管道符可以避免歧义。

## 传输方式

`--mcp-server` 的值以 `http://` 或 `https://` 开头 → **HTTP**，否则 → **Stdio**。

| 传输方式 | 自动判断条件 | 适用场景 |
|----------|-------------|---------|
| **Stdio** | 值以 `http://` 或 `https://` 开头 | 本地 MCP 服务器（uvx / npx / python / docker） |
| **HTTP** | 值以 `http://` 或 `https://` 开头 | 远程 API 服务（AnySearch 等） |

---

## 示例

### Stdio 子进程

```bash
# uvx
--mcp-server "fetch=uvx mcp-server-fetch"

# npx
--mcp-server "github=npx -y @modelcontextprotocol/server-github"

# python
--mcp-server "fetch=python -m mcp_server_fetch --ignore-robots-txt"

# docker
--mcp-server "fetch=docker run -i --rm mcp/fetch"
```

### 多个服务器混用

```bash
accelerate run "对比 GPU 价格并搜索相关讨论" \
  --mcp-server "fetch=uvx mcp-server-fetch" \
  --mcp-server "github=npx -y @modelcontextprotocol/server-github"
```

### HTTP 远程（带认证）

```bash
accelerate run "搜索量子计算" \
  --mcp-server "search=https://api.anysearch.com/mcp|Authorization:Bearer as_sk_xxxxxxxxxxxx"
```

### HTTP 多头部

```bash
accelerate run "带自定义追踪ID的查询" \
  --mcp-server "api=https://api.example.com/mcp|Authorization:Bearer <key>|X-Trace-Id:abc-123"
```

---

## 工作原理

启动时，RICA 依次：

1. **连接** — 根据格式创建 HTTP 客户端或启动子进程
2. **握手** — 发送 `initialize` 请求，协商协议版本
3. **发现工具** — 调用 `tools/list` 获取服务端提供的所有工具列表
4. **注册** — 将每个工具包装为 `machine::Tool`，注入 agent 的可用工具池
5. **执行** — agent 在对话中调用工具时，通过原有连接转发 `tools/call` 请求并返回结果

启动日志中会显示发现的工具：

```
INFO mcp: "search" server="search" tool_count=4 "MCP server ready"
DEBUG mcp: discovered tool  server="search" tool="search"
DEBUG mcp: discovered tool  server="search" tool="batch_search"
```

---

## 注意事项

- Stdio 子进程使用 `kill_on_drop(true)` 管理生命周期，进程随 agent 退出而终止
- Authorization header 等敏感信息会随每个请求发送，确保使用 HTTPS
- Header value 中如果包含 `|` 字符，当前不支持转义——可用环境变量注入
- 多个同 label 的服务器不会报错，但工具名会合并，后注册的工具覆盖同名的已有工具
