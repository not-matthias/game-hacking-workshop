---
theme: uncover
marp: true
---

# Game Hacking with Rust

---

## Target Audience

- You should know how to program
- Systems Programming basics

---

## You need to download:

- [AssaultCube](https://assault.cubers.net/download.html)
- [Rust](https://rustup.rs/)
- [VSCode](https://code.visualstudio.com/download) with Rust Analyzer extension

<!-- 
- [IDA Free](https://hex-rays.com/ida-free/)
- [Cheat Engine](https://www.cheatengine.org/) (Windows)
- [Scanmem](https://github.com/scanmem/scanmem) (Linux)
 -->

---

## Source Code

Available here: 
https://github.com/not-matthias/game-hacking-workshop
or
https://shorturl.at/jmqrV

---

## About Rust

- Types: `i16` vs `u16`
- Functions: `fn foo(bar: u32) -> i16 {}`
- Variables: `let mut temp = 42;`
- Run with: `cargo r` or `cargo run`

---

## Important Concepts

---

### What is a pointer?

- Points to a memory location
- 64 Bit Process => 64 Bit pointers
- 32 Bit Process => 32 Bit pointers

---

### External vs Internal Cheats

TODO

---

## Useful tools

---

### Cheat Engine

![bg right:73% 95%](https://wiki.cheatengine.org/images/8/8e/Tutorials.CETutorialx64.step02.04.png)

---

### Reclass

![bg left:70% 90%](https://camo.githubusercontent.com/712de35cebd00cea16055c09ef4262a2476d7f2e84f9425a7fcf31de6a9dc910/68747470733a2f2f61626c6f61642e64652f696d672f6d61696e346873626a2e6a7067)

---

### IDA

![bg right:80% 95%](ida.jpg)

---

### x64Dbg

![bg left:72%  95%](https://camo.githubusercontent.com/399b3391c873c9c1484f4487de23e20435d4ef2e251104660c6f7e1fc83e8ee6/68747470733a2f2f692e696d6775722e636f6d2f563266354150392e706e67)

---

## Let's get started

<!-- 

- Show how you can find the offset in IDA
- Show how you can find it with findmem
- Write to health (static)
- Read player, write to health offset

-->

---

## What we learned



---

## And this is just the start...

---

### More ideas

- Read enemy positions -> Radar or ESP
- Aimbot
- Movement Speed multiplier
- No Recoil / Spread

---

## Anticheats

- EasyAntiCheat
- Battleye
- Vangard
- ...

---

## Binary Analysis

- Deobfuscation
- Devirtualization
- ...

---

## Recommended Resources

- [unknowncheats.me](https://www.unknowncheats.me/)
- [github.com/hax-rs](https://github.com/hax-rs)
- [/r/ReverseEngineering](https://www.reddit.com/r/ReverseEngineering/)
- Discord Servers
  - [Rust RE](https://discord.gg/m2EnYsQddj)
  - [hax-rs](https://discord.gg/TSbkZbnnjJ)
  - [Reverse Engineering](https://discord.gg/rtfm)
