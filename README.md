# devo
development environment organizer


## Problem that devo solve.
Always that you change of company role or a computer you need to setup your dev environment.

Exist good but really huge solution like Ansible,NixOS, NixPackage manager to set your dev env.
But I have need learn another language only to install things.



Using a simple toml file you can install your packages set your dotfiles and git config really
straightfoward without need learn another language or command line tool.




## Config

in `~/.config/devo/devo.toml`


you can set several different sections like 

```toml
[packages]
names=[
    "neovim",
    "fzf"
]
```

## Usage

```bash
devo
```

## Pending Features.

Use devo vault to encrypt tokens and ssh keys like gh 
