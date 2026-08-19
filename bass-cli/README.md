# bass-cli

用来测试 crate.

## 库

```powershell
cargo run -p bass-cli -- devices --dll-dir E:\libs\bass24\x64
```

目录中需要包含 `bass.dll`, `bass_fx.dll` 等.

## 命令

```text
bass-cli devices [--dll-dir PATH]
bass-cli plugins PLUGIN.dll... --dll-dir PATH
bass-cli inspect FILE --dll-dir PATH
bass-cli play FILE_OR_URL --dll-dir PATH
bass-cli effects FILE --dll-dir PATH
bass-cli midi FILE --dll-dir PATH --max-polyphony N
```

所有命令都需要 BASS 路径参数：`--dll-dir PATH` 或 `--bass PATH`. 

### 枚举设备

```powershell
cargo run -p bass-cli -- devices --dll-dir E:\libs\bass24\x64
```

### 检查音频文件

```powershell
cargo run -p bass-cli -- inspect "E:\Music\歌曲.mp3" `
  --dll-dir E:\libs\bass24\x64
```

### 播放文件

```powershell
cargo run -p bass-cli -- play "E:\Music\歌曲.mp3" `
  --dll-dir E:\libs\bass24\x64 `
  --duration 10
```

使用 DirectSound：

```powershell
cargo run -p bass-cli -- play "E:\Music\歌曲.mp3" `
  --dll-dir E:\libs\bass24\x64 `
  --backend=dsound
```

### 测试 URL 流

```powershell
cargo run -p bass-cli -- play https://example.com/audio.mp3 `
  --dll-dir E:\libs\bass24\x64 `
  --watch-buffer --duration 30
```

### 测试效果

```powershell
cargo run -p bass-cli -- effects "E:\Music\歌曲.mp3" `
  --dll-dir E:\libs\bass24\x64
```

### 加载插件

```powershell
cargo run -p bass-cli -- plugins E:\libs\bass24\x64\bassflac.dll `
  --dll-dir E:\libs\bass24\x64
```