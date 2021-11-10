
# rusty-chess-engine

A chess engine that's kinda Rusty. Built with [bevy](https://github.com/bevyengine/bevy)


## Installation Insturctions

### macOS

Install zld
```bash
brew install michaeleisel/zld/zld
```

Then run `cargo run`.

### Arch

There's a few issues with `Arch`. Namely, you may need to comment out the following lines in` .cargo/config.toml`:
```toml
[target.x86_64-unknown-linux-gnu]
linker = "/usr/bin/clang"
rustflags = ["-Clink-arg=-fuse-ld=lld", "-Zshare-generics=y"]
```

Note the tracking issue for audio: [https://github.com/bevyengine/bevy/issues/2705](https://github.com/bevyengine/bevy/issues/2705).
