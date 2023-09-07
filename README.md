# OneLife

## Setup

The setup is based on
[this tutorial](https://rustwasm.github.io/docs/wasm-pack/prerequisites/index.html),
but an excerpt follows below.

Install rust and then wasm-pack:

```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

Install [nvm](https://github.com/nvm-sh/nvm):

```bash
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.1/install.sh | bash
```

Follow any extra instructions from the script and restart your terminal. Then run:

```bash
nvm install v16.13.2
nvm use
```

You don't need to specify the version since it is set by the `.nvmrc` file.

Install npm and then run:

```bash
npm install
```

To start a web server that auto-reloads on changes, run:

```bash
npm start
```

## Test Release version

Run:

```bash
npm run-script build
cd dist
python3.8 -m http.server 8000
```

Note that you need quite a new server (python 3.8 for example)
or the mime type won't be correct for the wasm content.

## Release version

We release by pushing to the github pages branch with the help of the
[gh-pages](https://www.npmjs.com/package/gh-pages) package.

To publish/deploy run:

```bash
npm run deploy
```

## Test

### Easy way

Run all tests:

```bash
npm run test:node
```

That command is an alias for `WASM_BINDGEN_TEST_TIMEOUT=60 wasm-pack test --node`.

Or run a specific test file:

```bash
npm run test:node -- --test tier_0
```

### Manual Way

Install:

```bash
cargo install wasm-bindgen-cli
```

Run with:

```bash
WASM_BINDGEN_TEST_TIMEOUT=60 cargo test --target wasm32-unknown-unknown
```

## Code Standards

We use [prettier](https://prettier.io/) for formatting.
Please run the following before committing:

```bash
npx prettier --write .
```

Or a faster version:

```bash
npx prettier \
    $(git diff --name-only --diff-filter=ACM) \
    $(git diff --cached --name-only --diff-filter=ACM) \
    --write --ignore-unknown
```
