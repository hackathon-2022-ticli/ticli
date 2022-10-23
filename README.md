<h1 align="center">TiCLI</h1>
<p align="center">
    <em>A modern cli for tikv.</em>
</p>

<p align="center">
    <a href="https://github.com/hackathon-2022-ticli/ticli/actions?query=workflow%3ACICD">
        <img src="https://github.com/hackathon-2022-ticli/ticli/workflows/CICD/badge.svg" alt="CICD"/>
    </a>
    <img alt="GitHub" src="https://img.shields.io/github/license/hackathon-2022-ticli/ticli">
    <a href="https://aur.archlinux.org/packages/ticli-git">
    <img alt="AUR version" src="https://img.shields.io/aur/version/ticli-git">
    </a>
    <a href="https://github.com/hackathon-2022-ticli/ticli/releases">
        <img src="https://img.shields.io/badge/platform-%20Linux%20|%20OSX-orange.svg" alt="Platform"/>
    </a>
</p>

### Features

- Support `Raw` and `Transaction` TiKV API.
- Support `REPL` (i.e. with auto-completion) and `Command` mode.
- Support load kv records from csv file.
- Auto detects a non-interactive terminal with raw output (i.e. when you pipe into another process or into a file).
- Auto-completion in multiple shell (i.e. `bash`, `zsh`, `fish`, `elvish` `and` `powershell`)
- Support different output table styles, inluding markdown table.
- Correctly align CJK and emoji characters.

### Usage

```zsh
$ ticli
count     cnt   -- Count keys between the range
decr            -- Decrease the specified kye by one
delete    del   -- Delete the specified key
exists          -- Returns if key exists
exit      quit  -- Exit the program
flushall        -- Remove all keys from tikv
get             -- Get the value of key
getb            -- Get the value of key in binary format
help            -- Print this message or the help of the given subcommand(s)
incr            -- Increase the specified kye by one
loadcsv         -- Load kv records from csv file
noop            -- No Operation
ping            -- Return pong when connection is alive
scan            -- Scan keys between the range
set             -- Set key to hold the string value
setb            -- Set key to hold the binary data from the file
source          -- Execute commands from file
strlen          -- Get the length of the bytes stored at key
style           -- Specify the output table style
```

Run `ticli --help` to view detailed usage.

### Installation

#### On Arch Linux

`ticli` is available in the Arch User Repository. To install it from [AUR](https://aur.archlinux.org/packages/ticli-git):

```
yay -S ticli-git
```

Or

```
paru -S ticli-git
```

#### On macOS

You can install `ticli` with Homebrew:

```
brew tap hackathon-2022-ticli/homebrew-ticli
brew install ticli
```

#### From binaries

Pre-built versions of `ticli` for various architectures are available at [Github release page](https://github.com/hackathon-2022-ticli/ticli/releases).

#### From source

If you have latest Rust toolchains installed you can use `cargo` to install it from source:

```
cargo install --git https://github.com/hackathon-2022-ticli/ticli
```

### Credits

* [tcil](https://github.com/c4pt0r/tcli): A tikv cli written in go.

### License

`ticli` is distributed under the terms of both the MIT License and the Apache License 2.0.

See the [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) files for license details.
