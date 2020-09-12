# Pretty Exec

[![Test](https://github.com/KSXGitHub/pretty-exec/workflows/Test/badge.svg)](https://github.com/KSXGitHub/pretty-exec/actions?query=workflow%3ATest)
[![Travis Build Status](https://img.shields.io/travis/KSXGitHub/pretty-exec/master?label=build&logo=travis)](https://travis-ci.org/KSXGitHub/pretty-exec)

Print a command and then execute it.

## Usage

### Print a command and then execute it

```sh
pretty-exec -- <program> [arguments]...
```

### Print a command only

```sh
pretty-exec --skip-exec -- <program> [arguments]...
```

### Group command output in a group in a GitHub Action logs

```sh
pretty-exec --github-actions -- <program> [arguments]...
```

### Print help message

```sh
pretty-exec --help
```

## Installation

### From [Crates.io](https://crates.io/crates/pretty-exec/)

```sh
cargo install pretty-exec
```

### From [GitHub Release](https://github.com/KSXGitHub/sane-fmt/releases)

Just go to [the release page](https://github.com/KSXGitHub/sane-fmt/releases) and download suitable binary.

### From [the Arch User Repository (AUR)](https://aur.archlinux.org)

#### [Build from source](https://aur.archlinux.org/packages/pretty-exec/)

```sh
yay -S pretty-exec
```

#### [Download prebuilt binary](https://aur.archlinux.org/packages/pretty-exec-bin/)

```sh
yay -S pretty-exec-bin
```

## License

[MIT](https://git.io/JfwzH) © [Hoàng Văn Khải](https://github.com/KSXGitHub/)
