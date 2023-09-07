[<img alt="github" src="https://img.shields.io/badge/github-kakoc/birdy?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/kakoc/birdy)
[<img alt="crates.io" src="https://img.shields.io/crates/v/birdy.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/birdy)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-66c2a5?style=for-the-badge&labelColor=555555&logoColor=white&logo=docs.rs" height="20">](https://docs.rs/birdy/latest/birdy)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/kakoc/birdy/rust.yml?branch=main&style=for-the-badge" height="20">](https://github.com/kakoc/birdy/actions/workflows/rust.yml)

Screenshot tool for Linux. Supports both X11 and Wayland.  
Since I'm not using neither MacOS or Windows at all there are no any guarantees related to these platforms.  
If you want to add a support for MacOS or Windows - PRs are welcome.  

![image info](./assets/demo/demo.png)

## Installation

```bash
cargo install birdy
```

## Usage

```
birdy --help

Usage: 
  Currently it can be run only through "birdy" executable(from terminal, app launcher(e.g. rofi), bound to a hotkey):

  # bash
  birdy

  # e.g. sway
  bindsym $mod+Shift+p exec birdy


Hotkeys:
  Enter - take a screenshot of selected area, save to a clipboard and exit
  f - take a screenshot where selected area is focused, save to a clipboard and exit

  l - draw a line. after that hotkey you can press left button and start drawing a line
  r - draw a rectangular border. after that hotkey you can press left button and start drawing a rectangular border

  Esc - exit
```
