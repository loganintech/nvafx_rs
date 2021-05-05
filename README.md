# NVAFX_RS

Rust bindings to the Nvidia Maxine Audio FX Engine (Powering Nvidia Broadcast)

### Compilation

1. Download the Nvidia Maxine Audio Effects SDK: https://developer.nvidia.com/maxine-getting-started#audio
2. Move the included `nvafx` folder to the root of this project. Your directory tree should look like the diagram below

```
.
├── bindings.rs
├── build.rs
├── Cargo.lock
├── Cargo.toml
├── nvafx
│   ├── include
│   │   └── nvAudioEffects.h
│   └── lib
│       ├── libnv_audiofx.so -> libnv_audiofx.so.1
│       ├── libnv_audiofx.so.1 -> libnv_audiofx.so.1.0.0
│       └── libnv_audiofx.so.1.0.0
└── src
    ├── lib.rs
    └── main.rs

```