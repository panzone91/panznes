# panznes

panznes is a simple emulator for the Nintendo Entertainment System written in Rust. Currently some of the earlier NES games are running on the emulator, with some minor glitches.

## What it's working

- Simple ROM support
- CPU
- (Basic) PPU

## What it's missing

- MMC support
- Audio
- Several other hardware features like different controllers

## Usage

You can download the current build from [here](https://github.com/afiuorio/panznes/releases/tag/latest). Then, for running the emulator:

```bash
./panznes ROM_FILE
```

## Build
panznes uses cargo for building. After having installed the Rust toolchain and a C compiler:

```bash
cargo build --release
```