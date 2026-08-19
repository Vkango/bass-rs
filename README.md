# bass-rs

`bass-rs` 是 [BASS](https://www.un4seen.com) 音频库的 Rust 封装 crate. 还在更新中.

## 加载 BASS

可以指定完整路径:

```rust
use bass_rs::BassEngine;

let engine = BassEngine::load(r"E:\libs\bass24\x64\bass.dll")?;
```

也可以通过目录自动枚举:

```rust
use bass_rs::BassEngine;

let engine = BassEngine::load_from_directory(r"E:\libs\bass24\x64")?;
```

单独指定 BASS_FX 路径:

```rust
use bass_rs::{BassEngine, BassEngineOptions};

let engine = BassEngine::load_with_options(
    r"E:\libs\bass24\x64\bass.dll",
    BassEngineOptions {
        fx_path: Some(r"E:\libs\bass_fx24\x64\bass_fx.dll".into()),
        require_fx: true,
    },
)?;
```

BASS 二进制库需要手动下载, 且需要与应用程序目标架构一致.


## 主要 API

- `BassEngine`: 加载 DLL、枚举输出设备、初始化 WASAPI 或 DirectSound.
- `Channel`: 加载本地文件或 URL, 播放、暂停、停止、定位和读取音频信息.
- `Plugin`: 加载插件并读取插件版本和格式信息.
- `Effect`: 使用 DX8 效果和 BASS_FX 效果.
- `TempoChannel`、`ReverseChannel`: 速度、音高、频率和倒放处理.
- `RemoteProgress`: 获取 URL 流缓冲进度、下载字节数和下载速度.