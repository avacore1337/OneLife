# OneLife

## Setup

The setup is based on [this tutorial](https://rustwasm.github.io/docs/wasm-pack/prerequisites/index.html), but an excerp follows below.

Install rust and then wasm-pack:

```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

install [nvm](https://github.com/nvm-sh/nvm):

```bash
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.1/install.sh | bash
```

follow any extra instructions from the script and restart your terminal. Then run:

```bash
nvm install 16
nvm use 16
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

## Test Release version

run

```bash
npm run-script build
cd dist
python3.8 -m http.server 8000
```

Note that you need quite a new server (python 3.8 for example) or the mime type won't be correct for the wasm content

## Release version

We release by pushing to the github pages branch with the help of a npm package:
https://www.npmjs.com/package/gh-pages

To publish/deploy run:

```
npm run deploy
```

## Test

### Easy way

Run all tests

```
WASM_BINDGEN_TEST_TIMEOUT=60 wasm-pack test --node
```

Run a specific test file:

```bash
WASM_BINDGEN_TEST_TIMEOUT=60 wasm-pack test --node --test tier_0
```

### Manual Way

Install:

```
cargo install wasm-bindgen-cli
```

Run with:

```
WASM_BINDGEN_TEST_TIMEOUT=60 cargo test --target wasm32-unknown-unknown
```

## Code Standards

We use [prettier](https://prettier.io/) for formating, please run the following before commiting.

```
npx prettier --write .
```

Or a faster version

```
npx prettier $(git diff --name-only --diff-filter=ACM) $(git diff --cached --name-only --diff-filter=ACM) --write --ignore-unknown
```
