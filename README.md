# Teleport Dir

Simple Rust command to navigate into the origin dir of a symlinked file.

## Getting started 

Currently this package only has zsh shell integration.

### Installation 

**Step 1: Install package from cargo.io**

Use cargo to install the Rust crate.

```shell 
cargo install teleport-dir
```

**Step 2: Add to your ZSH config**

Add the following code to your `.zshrc` file, or another file in your function search path (`$fpath`).

```shell 
eval "$(teleport-dir init)"
```

This will add the `td` function.
You can also give the function any other name by passing the `--cmd` flag.

Below is an example where the function will be made available as `symdir`.

```shell
eval "$(teleport-dir init --symdir)"
```

## Usage 

Let's say for example you use `laravel/valet` or `brew` to manage multiple php versions, and you need to be in the directory of the currently linked php version.

You could achieve this by (e.g.) executing the command shown below.

```shell 
cd $(readlink -f /usr/local/bin/php)/../
```

This can be a little bit tedious or maybe hard to remember, so instead you can use:

```shell 
td /usr/local/bin/php 
```

The command checks if the path is symlinked resolves the origin, if this origin is a file it `cd`s to the directory of the file. If the path is already a directory it just `cd`s there.

For example when you have linked `php@8.0` with _HomeBrew_.
All of the commands below will equate to `cd /usr/local/Cellar/php@8.0/8.0.17/bin/` 

```shell 
td /usr/local/bin/php

td /usr/local/Cellar/php@8.0/8.0.17/bin/php 

td /usr/local/Cellar/php@8.0/8.0.17/bin/
```

## Credits 

This project is mainly just an excercise while I'm trying to learn Rust, and I have learned a lot by looking at other crate's I use or do similar things. 
When trying to solve the issue you can't change a directory in your main shell from a Rust command, I have taken a lot (or most) of the idea from looking at [zoxide](https://github.com/ajeetdsouza/zoxide).

## License 

This composer package is an open-sourced software licensed under the [MIT License](https://github.com/jascha030/teleport-dir/blob/master/LICENSE)

