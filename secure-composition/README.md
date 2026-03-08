# Secure Composition

A Temper library that allows secure composition of complex content
types including HTML, SQL, file paths and shell strings.

Since Temper translates to other languages, it allows securely
composing content in those langauges too.

See [config.temper.md](./config.temper.md)

## To regenerate transition functions from transition tables

After committing any uncommitted changes:

```sh
$ cd scripts
$ poetry run python3 -m regen_temper ../src/html/*.temper.md
```

## Running tests

Running tests in the JS backend is actually the easiest way.

```sh
$ temper test -b js
Tests passed: 27 of 27
```
