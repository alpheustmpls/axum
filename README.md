# Rust Axum

This template provides a setup for a Rust axum project.

## Dependencies

Please install the following dependencies:

| Dependencies                                    | Description                            |
| ----------------------------------------------- | -------------------------------------- |
| [Rust](https://www.rust-lang.org)               | Programming language                   |
| [mkcert](https://github.com/FiloSottile/mkcert) | HTTPS certificate generator            |
| [just](https://just.systems)                    | Command runner                         |
| [ls-lint](https://ls-lint.org/)                 | Linting tool for directories and files |
| [typos-cli](https://github.com/crate-ci/typos)  | Spell checker                          |

## Commands

The following commands are available:

### Formatting

This command will format the code.

```sh
just fmt
```

### Linting

This command will lint the code.

```sh
just lint
```

### Checking

This command will do formatting and linting.

```sh
just check
```

### Development (HTTPS)

This command will start the development server in HTTPS mode.

```sh
just cert
just dev
```

### Development (HTTP)

This command will start the development server in HTTP mode.

```sh
just http
```

### Production

This command will build the code and start the production server.

```sh
just build
just start
```

### Cleaning

This command will clean the unnecessary files.

```sh
just clean
```
