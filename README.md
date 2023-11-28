[![Test build](https://github.com/abdellatif-temsamani/cmake-init/actions/workflows/rust.yml/badge.svg)](https://github.com/abdellatif-temsamani/cmake-init/actions/workflows/rust.yml)

[![Test run](https://github.com/abdellatif-temsamani/cmake-init/actions/workflows/run.yml/badge.svg)](https://github.com/abdellatif-temsamani/cmake-init/actions/workflows/run.yml)

# CMAKE-INIT

<!-- toc -->

- [Installation](#installation)
  * [From source](#from-source)
  * [Requirements](#requirements)
    + [Linux](#linux)
    + [MacOS](#macos)
  * [Windows](#windows)
- [Usage](#usage)
  * [Create a project](#create-a-project)
  * [Create a project with a specific language](#create-a-project-with-a-specific-language)
  * [Create a project with a specific cmake version](#create-a-project-with-a-specific-cmake-version)
- [License](#license)
- [Contributing](#contributing)
  * [Contributors](#contributors)

<!-- tocstop -->

`cmake-init` is a cli tool to initialize a cmake project. It creates a cmake
project with a basic structure of your desired language and `CmakeLists.txt`

# Installation

## From source

## Requirements

- [git](https://git-scm.com/)
- [cmake](https://cmake.org/)
- [rust](https://www.rust-lang.org/)

### Linux

```bash
git clone https://github.com/abdellatif-temsamani/cmake-init

cd cmake-init

./scripts/linux/install.sh
```

### MacOS

```bash
git clone https://github.com/abdellatif-temsamani/cmake-init

cd cmake-init

./scripts/macos/install.sh
```

## Windows

```sh
git clone https://github.com/abdellatif-temsamani/cmake-init

cd cmake-init

.\scripts\win\install.bat
```

# Usage

Help menu

```sh
cmake-init --help

## Usage: cmake-init --name=<name>
##
## Options:
##     --name <name>   [required]   The name of the project.
##     --cmake-version <version>    The minimum version of CMake to use.
##     --lang <version>             The language chosen for the project(cpp, c).
##     --templates-dir <dir>        The directory containing the templates.
##     --git-path <path>            The path to the git binary.
##       if you're using GitHub CLI, you can set this to 'gh'
##       if you're using git, you can set this to 'git'
##
##     --help | -h                  Print this help message.
##     --version | -v               Print the version of cmake-init.
```

## Create a project

```sh
#
cmake-init --name=project_name
```

## Create a project with a specific language

```sh
cmake-init --name=project_name --lang=cpp
```

## Create a project with a specific cmake version

```sh
cmake-init --name=project_name --cmake-version=3.20
```

# License

[GPL-3.0](./LICENSE)

# Contributing

Every contribution is welcome, feel free to open an issue or a pull request.

## Contributors

- [Abdellatif Temsamani](https://github.com/abdellatif-temsamani)
