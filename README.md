# OneLife

## Setup
Follow this setup:
https://rustwasm.github.io/docs/wasm-pack/prerequisites/index.html

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

## Notes:
* Money = money during life.
* Coins = coins you get when passing into the underworld.
* Karma = tips and boosts

Big list of features:

# Implement save game upgrades. 
When game is expanded/changed migrations will need to happen.
Most things can probably be handled by checking version number of save and then migrating values over to new clean save but with previous values migrated:
https://stackoverflow.com/questions/47070876/how-can-i-merge-two-json-objects-with-rust


