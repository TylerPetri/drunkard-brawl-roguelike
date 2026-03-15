# Drunkard Brawl 🍺🥴

A silly 1v1 turn-based drinking card game written in Rust with bracket-lib.
My first Rust build, dipping toes into the Rust gaming community!

You and your opponent take turns slamming real beers (Pliny the Elder, Heady Topper, Guinness, etc.).  
The goal? Make the other person black out first (reach 0 HP).  
Occasional roguelike mixer events coming soon (Red Bull, Fireball shots, pickles at 3 a.m., etc.).

Very much a work-in-progress / learning project — Skipping all tutorials tonight for obvious reasons, shoutout to grok

Bracket-lib roguelike tutorial:
- https://bfnightly.bracketproductions.com/rustbook/

## Current gameplay

- 5 beer cards in hand every round
- Press 1–5 to play a card
- AI opponent auto-plays something stupid
- HP goes down (sometimes up if you're drinking Guinness)
- First to 0 HP loses (blackout)

## Requirements

- **Rust** ≥ 1.70 (2021 edition)
- A terminal that supports colors (most do)

## Installation & Running

### 1. Install Rust (if you don't have it yet)

https://www.rust-lang.org/tools/install

```bash
# Linux / macOS
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows → use the .exe installer from the link above (recommended)
# or in PowerShell / cmd:
winget install Rustlang.Rustup