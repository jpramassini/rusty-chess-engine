
# rusty-chess-engine

A chess engine that's kinda Rusty. Built with [bevy](https://github.com/bevyengine/bevy)


## Installation Insturctions

### macOS

Install zld
```bash
brew install michaeleisel/zld/zld
```

Then run `cargo run`.

### Linux

There's a few issues with `Linux`. Namely, you may need to comment out the following lines in` .cargo/config.toml`:
```toml
[target.x86_64-unknown-linux-gnu]
linker = "/usr/bin/clang"
rustflags = ["-Clink-arg=-fuse-ld=lld", "-Zshare-generics=y"]
```

libudev-dev is also required, and is not always installed by defualt. 

Note the tracking issue for audio: [https://github.com/bevyengine/bevy/issues/2705](https://github.com/bevyengine/bevy/issues/2705).
