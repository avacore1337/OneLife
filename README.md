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

Optionally, you may install [just](https://just.systems/man/en/), which is
essentially a modernized makefile. It is not required, but it is convenient.
This repo's `justfile` has some useful commands. that are also referenced
in this README. And after installing `just`, then you can run the command
to see all available recipes:

![just recipes](./docs/just-recipes.png 'just recipes')
(just recipes as of 18-Sep-2023)

Follow any extra instructions from the script and restart your terminal. Then run:

```bash
nvm install v16.13.2
nvm use
```

You don't need to specify the version since it is set by the `.nvmrc` file.

Install npm and then run:

```bash
npm install
# OR
just init
```

To start a web server that auto-reloads on changes, run:

```bash
npm start
# OR
just dev
```

## Test Release version

Run:

```bash
npm run start:dist
# OR
just start-dist
```

Note that you need Python 3.8 (2019 release) or newer,
else the mime type won't be correct for the wasm content.

## Release version

We release by pushing to the github pages branch with the help of the
[gh-pages](https://www.npmjs.com/package/gh-pages) package.

To publish/deploy run:

```bash
npm run deploy
# OR
just publish
```

## Test

### Easy way

Run all tests:

```bash
npm run test:node
# OR
just test-node
```

That command is an alias for `WASM_BINDGEN_TEST_TIMEOUT=60 wasm-pack test --node`.

Or run a specific test file:

```bash
npm run test:node -- --test tier_0 --test tier_1
# OR
just test-node tier_0 tier_1
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
