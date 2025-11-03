# Run With Dotenv `.env`

A small utility (`de`) that runs a program with environment variables loaded from a
`.env` file, without leaking them into your shell environment.

Sourcing `.env` files directly pollutes your shell:

```bash
# Avoid this – leaks vars into your shell session
source .env
my-program --foo bar

# Or switching files requires cleanup
unset $(grep '^[^#]' test.env | cut -d= -f1)  # Manual cleanup
source test.env
my-program --foo bar
```

Instead, use `de` for isolated execution:

```bash
# Loads from `.env` by default
de my-program --foo bar

# Specify a custom file
de -f test.env my-program --foo bar
```

## Install

```bash
cargo install --git https://github.com/Nachtalb/run-with-dotenv
```
