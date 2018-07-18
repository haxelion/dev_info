# dev_info

[![Crates.io](https://img.shields.io/crates/v/dev_info.svg)](https://crates.io/crates/dev_info)
[![Build Status](https://travis-ci.org/haxelion/dev_info.svg?branch=master)](https://travis-ci.org/haxelion/dev_info)

``dev_info`` is a simple command line utility that can display informations related to the current 
git directory. Contrary to other alternatives, it is quite fast because it relies on ``libgit2`` 
to directly obtain repository information without spawning other processes.

## Installation

First run:

```sh
cargo install dev_info
```

Then add it to your ``PROMPT`` or ``PS1`` (this depends on your shell). For ``zsh`` I have the 
following:

```sh
setopt PROMPT_SUBST
PROMPT='[%{%F{red}%}%n%{%f%}@%{%F{red}%}%m%{%f%}]%{%F{blue}%}%~%{%f%} $(dev_info -b -c 7 -s) %# '
```

For ``bash`` you can look there: https://wiki.archlinux.org/index.php/Bash/Prompt_customization#Embedding_commands

## Options

Currently the available options are limited:

```
Usage: dev_info [options]

Options:
    -h, --help          print this help menu
    -C, --color SCHEME  render using a color scheme
    -b, --branch        print the branch name
    -c, --commit LENGTH print the commit id, truncated to LENGTH
    -s, --state         print the repository state
```

## Known problems

Right now the colorized output creates cursor position reset bugs, I have to figure out how to 
escape the ANSI sequences.
