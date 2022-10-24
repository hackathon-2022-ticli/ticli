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

- Support both `Raw` and `Transaction` KV API.
- Support both `REPL` and `Command` mode.
- Support for parsing and executing command scripts.
- Correctly handling CJK and emoji characters.
- Pretty and configurable output styles.
- Auto completion and syntax highlight.
- Auto switch the output style according to whether stdout is a tty.

### Installation

#### On macOS

You can install `ticli` with Homebrew:

```
brew tap hackathon-2022-ticli/homebrew-ticli
brew install ticli
```

#### On Arch Linux

`ticli` is available in the Arch User Repository. To install it from [AUR](https://aur.archlinux.org/packages/ticli):

```
paru -S ticli # or yay -S ticli
```

#### From binaries

Pre-built versions of `ticli` for various architectures are available at [Github release page](https://github.com/hackathon-2022-ticli/ticli/releases).

#### From source

If you have latest Rust toolchains installed you can use `cargo` to install it from source:

```
cargo install --git https://github.com/hackathon-2022-ticli/ticli
```

### Supported commands

| command   |                        Description                        |
|:---------:|-----------------------------------------------------------|
| GET       | Get the value of key                                      |
| GETB      | Get the value of key in binary format                     |
| SET       | Set key to hold the string value                          |
| SETB      | Set key to hold the binary data from the file             |
| INCR      | Increments the number stored at key by one                |
| INCRBY    | Increments the number stored at key by increment          |
| DECR      | Decrements the number stored at key by one                |
| DECRBY    | Decrements the number stored at key by decrement          |
| DELETE    | Delete the specified key                                  |
| STRLEN    | Get the length of the bytes stored at key                 |
| EXISTS    | Returns if key exists                                     |
| SCAN      | Scan keys between the range                               |
| COUNT     | Count keys between the range                              |
| SOURCE    | Execute commands from file                                |
| LOADCSV   | Load kv records from csv file                             |
| FLUSHALL  | Remove all keys from tikv                                 |
| PING      | Return pong when connection is alive                      |
| STYLE     | Specify the output table style                            |
| QUIT      | Exit the program                                          |
| HELP      | Print this message or the help of the given subcommand(s) |

Run `ticli --help` to view detailed usage.

### Credits

* [tcil](https://github.com/c4pt0r/tcli): A tikv cli written in go.

### License

`ticli` is distributed under the terms of both the MIT License and the Apache License 2.0.

See the [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT) files for license details.
