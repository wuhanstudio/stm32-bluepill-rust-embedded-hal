# Rust Embedded HAL: From Zero to Aync

> STM32F103C8T6 (Cortex-M1)  
> Flash: 64KB, SRAM: 20KB, CPU: 72MHz

<img width="750" height="500" alt="image" src="https://github.com/user-attachments/assets/90791183-cced-4191-876d-653bbe6f1b05" />

## Prerequisites

Install [Probe-rs](https://probe.rs/docs/getting-started/installation/):

```
# Linux
$ curl --proto '=https' --tlsv1.2 -LsSf https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.sh | sh
```

```
# Windows
$ Set-ExecutionPolicy RemoteSigned -scope CurrentUser
$ irm https://github.com/probe-rs/probe-rs/releases/latest/download/probe-rs-tools-installer.ps1 | iex
```

Install Rust toolchain:

```
# Linux
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

```
# Windows
$ winget install rustup

# If you don't have Visual Studio on Windows
$ rustup toolchain install stable-x86_64-pc-windows-gnu
$ rustup default stable-x86_64-pc-windows-gnu
```

Add Cortex-M thmbv7em (no hardware floating point) support

```
$ rustup target add thumbv7em-none-eabi
```

## References

- Async/await on Embedded Rust: https://ferrous-systems.com/blog/async-on-embedded/#from-blocking-to-non-blocking
- TheRustyBits: https://www.youtube.com/@therustybits
- From Zero to Async: https://github.com/therustybits/zero-to-async
