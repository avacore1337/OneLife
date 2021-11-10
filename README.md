# OneLife

## Setup

The setup is based on [this tutorial](https://rustwasm.github.io/docs/wasm-pack/prerequisites/index.html), but an excerp follows below.

Install rust and then wasm-pack:

```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

install npm and then run:

```bash
npm install
```

Then run:

```bash
npm start
```

To start a webserver that auto-reloads on changes.

## Code Standards

We use [prettier](https://prettier.io/) for formating, please run the following before commiting.

```
npx prettier --write .
```

## Notes:

- Money = money during life.
- Coins = coins you get when passing into the underworld.
- Karma = tips and boosts

# Features

- C - Core, needed for even testing begins.
- R - Release, needed before the game is released.
- E - Expansion, Thing to look at after the base game is finished
- N - Nice to have, thing/ideas that we can implement if we think they are fun or a good learning experience

## Both

## Rust/Engine

### R - Implement save game export bonuses

Players should get a bonus when they export the game. 30m time, once every 6h?

### Implement save game upgrades.

When game is expanded/changed migrations will need to happen.
Most things can probably be handled by checking version number of save and then migrating values over to new clean save but with previous values migrated:
https://stackoverflow.com/questions/47070876/how-can-i-merge-two-json-objects-with-rust

## Vue/Frontend

### R - Auto-save

add a checkbox to turn it on/off, set_autosave is index.js

### E - Import save file through text pasting

Players should easily be able to import a save either through a text file upload or by pasting in text.
