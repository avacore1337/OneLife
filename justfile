# bash/justfile

# https://just.systems/man/en/chapter_1.html

[private]
@default:
    echo "View file 'justfile' to see internals of any recipe."
    just --list --unsorted

# ------------------------------------------------------------------------------
# BASICS
# ------------------------------------------------------------------------------

# TODO: add check for all installations (rust, npm, nvm/n, wasm-pack, etc.),
#       and auto-install anything missing.
# Initialize the repo. Necessary after initial git-clone.
@init:
    npm install
    echo "Note that this init script does not (yet) install Rust, nvm/n, wasm-pack, etc."
    echo "Please follow the guidance in the README.md file to install everything."

# Clean up the repo by removing all generated files.
@clean:
    rm -rf dist pkg

# ------------------------------------------------------------------------------
# LINTING
# ------------------------------------------------------------------------------

# Lint all code.
@lint: lint-js lint-rs

# Only lint the JavaScript code via eslint.
@lint-js:
    eslint --ext .js,.vue --ignore-path .gitignore --fix js

# Only lint the Rust code via Clippy.
@lint-rs:
    cargo clippy --all-targets --all-features -- -D warnings

# ------------------------------------------------------------------------------
# RUN LOCAL SERVERS
# ------------------------------------------------------------------------------

# Run the development server, which has hot-reloading and debug-mode.
@start-dev: clean
    webpack-dev-server --open -d
alias dev := start-dev

# Run the distribution server, which does NOT have hot-reloading or debug-mode.
@start-dist: build
    python3 -m http.server 8000 --directory dist

# ------------------------------------------------------------------------------
# BUILD & DEPLOY
# ------------------------------------------------------------------------------

# Build the deployable distribution.
@build: clean
    webpack

# Manually publish the app to GitHub Pages.
@publish: build && clean
    gh-pages -d dist

# ------------------------------------------------------------------------------
# TESTING
# ------------------------------------------------------------------------------

# Run all tests.
test: test-rs test-wasm test-node

# Run Rust tests.
@test-rs:
    cargo test

# Run headless Wasm tests. Optionally give specific browsers; ex: just test-wasm chrome safari
@test-wasm *BROWSERS='node chrome firefox safari':
    BROWSER_ARG=$(echo {{BROWSERS}} | sed 's/[^ ]* */--&/g') && \
    wasm-pack test --headless $BROWSER_ARG; \

# Run Wasm tests with Node.js. Optionally give specific tests; ex: just test-node tier_0 tier_1
@test-node *TESTS:
    if [ -z "{{TESTS}}" ]; then \
        WASM_BINDGEN_TEST_TIMEOUT=60 wasm-pack test --node; \
    else \
        TEST_ARG=$(echo {{TESTS}} | sed 's/[^ ]* */--test &/g') && \
        WASM_BINDGEN_TEST_TIMEOUT=60 wasm-pack test --node -- $TEST_ARG; \
    fi