use std::{
    env,
    path::PathBuf,
    thread,
    time::{Duration, Instant},
};

use bass_rs::{
    BassEngine, BassEngineOptions, BassError, EffectKind, InitOptions, OutputBackend,
    SourceOptions, UrlOptions, raw,
};

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

fn run() -> bass_rs::Result<()> {
    let mut args = env::args().skip(1);
    let Some(command) = args.next() else {
        print_help();
        return Ok(());
    };
    let args: Vec<String> = args.collect();
    match command.as_str() {
        "devices" => {
            let engine = load_engine(&args)?;
            println!(
                "BASS 0x{:08x}; BASS_FX {}",
                engine.bass_version(),
                engine
                    .fx_version()
                    .map(|v| format!("0x{v:08x}"))
                    .unwrap_or_else(|| "not loaded".into())
            );
            for device in engine.devices()? {
                println!(
                    "{}: {} [{}] flags=0x{:08x}",
                    device.index, device.name, device.driver, device.flags
                );
            }
        }
        "plugins" => plugins(args)?,
        "inspect" => inspect(args)?,
        "play" => play(args)?,
        "effects" => effects(args)?,
        "midi" => midi(args)?,
        "help" | "--help" | "-h" => print_help(),
        other => {
            return Err(BassError::InvalidInput {
                kind: "command",
                message: format!("unknown command {other}"),
            });
        }
    }
    Ok(())
}

fn load_engine(args: &[String]) -> bass_rs::Result<BassEngine> {
    let fx_path = option_value(args, "--bass-fx").map(PathBuf::from);
    let options = BassEngineOptions {
        fx_path,
        require_fx: false,
    };
    if let Some(bass) = option_value(args, "--bass") {
        BassEngine::load_with_options(bass, options)
    } else if let Some(directory) = option_value(args, "--dll-dir") {
        BassEngine::load_from_directory_with_options(directory, options)
    } else {
        Err(BassError::InvalidInput {
            kind: "argument",
            message: "BASS path is required; use --bass PATH or --dll-dir PATH".into(),
        })
    }
}

fn init(engine: &BassEngine, backend: OutputBackend) -> bass_rs::Result<()> {
    engine.initialize(InitOptions {
        backend,
        ..InitOptions::default()
    })
}

fn plugins(paths: Vec<String>) -> bass_rs::Result<()> {
    let engine = load_engine(&paths)?;
    let plugin_paths = positional_paths(&paths);
    if plugin_paths.is_empty() {
        println!("pass plugin DLL paths after the required BASS option");
        return Ok(());
    }
    let plugins = engine.load_plugins(plugin_paths)?;
    for plugin in plugins {
        println!("plugin: version=0x{:08x}", plugin.info().version);
        for format in &plugin.info().formats {
            println!(
                "  0x{:08x}: {} ({})",
                format.channel_type, format.name, format.extensions
            );
        }
    }
    Ok(())
}

fn inspect(args: Vec<String>) -> bass_rs::Result<()> {
    let path = required_path(&args, "inspect requires a file path")?;
    let engine = load_engine(&args)?;
    init(&engine, OutputBackend::Wasapi)?;
    let channel = engine.load_file(&path, SourceOptions::default())?;
    let info = channel.info()?;
    println!(
        "file={path}\nkind={:?}\nfrequency={}\nchannels={}\ntype=0x{:08x}\nlength={:?}",
        channel.kind(),
        info.frequency,
        info.channels,
        info.channel_type,
        channel.length()?
    );
    for tag in [
        bass_rs::TagKind::Id3v2,
        bass_rs::TagKind::Ogg,
        bass_rs::TagKind::Mp4,
    ] {
        let values = channel.tags(tag);
        if !values.is_empty() {
            println!("{tag:?}: {values:?}");
        }
    }
    Ok(())
}

fn play(args: Vec<String>) -> bass_rs::Result<()> {
    let source = required_path(&args, "play requires a file path or URL")?;
    let backend = if args.iter().any(|arg| arg == "--backend=dsound") {
        OutputBackend::DirectSound
    } else {
        OutputBackend::Wasapi
    };
    let watch = args.iter().any(|arg| arg == "--watch-buffer");
    let duration = option_value(&args, "--duration")
        .map(|value| value.parse::<u64>())
        .transpose()
        .map_err(|_| BassError::InvalidInput {
            kind: "duration",
            message: "expected integer seconds".into(),
        })?;
    let engine = load_engine(&args)?;
    init(&engine, backend)?;
    let is_url = source.starts_with("http://") || source.starts_with("https://");
    let channel = if is_url {
        engine.load_url(&source, UrlOptions::default())?
    } else {
        engine.load_file(&source, SourceOptions::default())?
    };
    channel.set_volume(1.0)?;
    channel.set_pan(0.0)?;
    channel.play(false)?;
    let started = Instant::now();
    loop {
        if watch
            && is_url
            && let Ok(progress) = channel.remote_progress()
        {
            println!(
                "state={:?} buffering={:?}% downloaded={:?} speed={:?} B/s",
                progress.state,
                progress.buffering_percent,
                progress.downloaded_bytes,
                progress.bytes_per_second
            );
        }
        if let Some(limit) = duration
            && started.elapsed() >= Duration::from_secs(limit)
        {
            break;
        }
        if !is_url && matches!(channel.active_state(), bass_rs::ActiveState::Stopped) {
            break;
        }
        thread::sleep(Duration::from_millis(100));
    }
    channel.stop().ok();
    Ok(())
}

fn effects(args: Vec<String>) -> bass_rs::Result<()> {
    let path = required_path(&args, "effects requires a file path")?;
    let engine = load_engine(&args)?;
    init(&engine, OutputBackend::Wasapi)?;
    let channel = engine.load_file(&path, SourceOptions::default())?;
    let eq = channel.add_effect(EffectKind::Dx8(raw::BASS_FX_DX8_PARAMEQ), 0)?;
    let parameters = raw::BASS_DX8_PARAMEQ {
        fCenter: 1000.0,
        fBandwidth: 1.0,
        fGain: 3.0,
    };
    eq.set_parameters(&parameters)?;
    println!(
        "DX8 ParamEQ: {:?}",
        eq.get_parameters::<raw::BASS_DX8_PARAMEQ>()?
    );
    if engine.has_fx() {
        let freeverb =
            channel.add_effect(EffectKind::BassFx(bass_rs::BassFxEffect::Freeverb), 0)?;
        let parameters = raw::BASS_BFX_FREEVERB {
            fDryMix: 0.0,
            fWetMix: 1.0,
            fRoomSize: 0.5,
            fDamp: 0.5,
            fWidth: 1.0,
            lMode: 0,
            lChannel: raw::BASS_BFX_CHANALL,
        };
        freeverb.set_parameters(&parameters)?;
        println!(
            "BASS_FX Freeverb: {:?}",
            freeverb.get_parameters::<raw::BASS_BFX_FREEVERB>()?
        );
    } else {
        println!("BASS_FX is not loaded; skipped Freeverb/Tempo/Reverse tests");
    }
    Ok(())
}

fn midi(args: Vec<String>) -> bass_rs::Result<()> {
    let path = required_path(&args, "midi requires a file path")?;
    let _engine = load_engine(&args)?;
    let polyphony = option_value(&args, "--max-polyphony")
        .unwrap_or_else(|| "0".into())
        .parse()
        .map_err(|_| BassError::InvalidInput {
            kind: "max-polyphony",
            message: "expected an integer".into(),
        })?;
    let addon = if let Some(dll) = option_value(&args, "--bassmidi") {
        bass_rs::midi::MidiAddon::load(dll)?
    } else if let Some(directory) = option_value(&args, "--dll-dir") {
        bass_rs::midi::MidiAddon::load_from_directory(directory)?
    } else {
        return Err(BassError::InvalidInput {
            kind: "argument",
            message: "BASSMIDI path is required; use --bassmidi PATH or --dll-dir PATH".into(),
        });
    };
    addon.set_max_polyphony(bass_rs::midi::MidiOptions {
        max_polyphony: Some(polyphony),
    })?;
    println!("MIDI source: {path}");
    Ok(())
}

fn required_path(args: &[String], message: &'static str) -> bass_rs::Result<String> {
    positional_paths(args)
        .first()
        .cloned()
        .ok_or(BassError::InvalidInput {
            kind: "argument",
            message: message.into(),
        })
        .cloned()
}

fn option_value(args: &[String], name: &str) -> Option<String> {
    args.iter().enumerate().find_map(|(index, arg)| {
        if let Some(value) = arg.strip_prefix(&format!("{name}=")) {
            Some(value.to_owned())
        } else if arg == name {
            args.get(index + 1).cloned()
        } else {
            None
        }
    })
}

fn positional_paths(args: &[String]) -> Vec<&String> {
    let value_options = [
        "--bass",
        "--bass-fx",
        "--bassmidi",
        "--dll-dir",
        "--duration",
        "--max-polyphony",
    ];
    let mut result = Vec::new();
    let mut skip_next = false;
    for arg in args {
        if skip_next {
            skip_next = false;
            continue;
        }
        if value_options.contains(&arg.as_str()) {
            skip_next = true;
            continue;
        }
        if arg.starts_with("--") {
            continue;
        }
        result.push(arg);
    }
    result
}

fn print_help() {
    println!(
        "bass-cli commands (use --bass PATH or --dll-dir PATH):\n  devices [--bass-fx PATH]\n  plugins <plugin.dll>...\n  inspect <file>\n  play <file|url> [--bass-fx PATH] [--backend=dsound] [--duration=N] [--watch-buffer]\n  effects <file> [--bass-fx PATH]\n  midi <file> [--bassmidi PATH] --max-polyphony=N\n\n--dll-dir scans the standard BASS/BASS_FX/BASSMIDI names in one directory."
    );
}
