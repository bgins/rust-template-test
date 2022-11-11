<div align="center">
  <a href="https://github.com/bgins/template-test" target="_blank">
    <img src="https://raw.githubusercontent.com/bgins/template-test/main/assets/a_logo.png" alt="template-test Logo" width="100"></img>
  </a>

  <h1 align="center">template-test</h1>

  <p>
    <a href="https://crates.io/crates/template-test">
      <img src="https://img.shields.io/crates/v/template-test?label=crates" alt="Crate">
    </a>
    <a href="https://npmjs.com/package/template-test">
      <img src="https://img.shields.io/npm/v/template-test" alt="Npm">
    </a>
    <a href="https://codecov.io/gh/bgins/template-test">
      <img src="https://codecov.io/gh/bgins/template-test/branch/main/graph/badge.svg?token=SOMETOKEN" alt="Code Coverage"/>
    </a>
    <a href="https://github.com/bgins/template-test/actions?query=">
      <img src="https://github.com/bgins/template-test/actions/workflows/tests_and_checks.yml/badge.svg" alt="Build Status">
    </a>
    <a href="https://github.com/bgins/template-test/blob/main/LICENSE-APACHE">
      <img src="https://img.shields.io/badge/License-Apache%202.0-blue.svg" alt="License-Apache">
    </a>
    <a href="https://github.com/bgins/template-test/blob/main/LICENSE-MIT">
      <img src="https://img.shields.io/badge/License-MIT-blue.svg" alt="License-MIT">
    </a>
    <a href="https://docs.rs/template-test">
      <img src="https://img.shields.io/static/v1?label=Docs&message=docs.rs&color=blue" alt="Docs">
    </a>
  </p>
</div>

<div align="center"><sub>:warning: Work in progress :warning:</sub></div>

##

## Outline

- [Crates](#crates)
- [Usage and Installation](#usage-and-installation)
- [Testing the Project](#testing-the-project)
- [Setting-up template-test-wasm](#setting-up-template-test-wasm)
- [Contributing](#contributing)
- [Getting Help](#getting-help)
- [External Resources](#external-resources)
- [License](#license)

## Crates

- [template-test](https://github.com/bgins/template-test/tree/main/template-test)
- [template-test-wasm](https://github.com/bgins/template-test/tree/main/)

## Usage and Installation

### Using `cargo`

This is just for the rust-only `template-test` binary application:

```console
$ cargo install template-test
```

### template-test-wasm Usage

Due to the reliance on [wasm-pack][wasm-pack], `template-test-wasm` is only
available as a library.

- Add the following to the `[dependencies]` section of your `Cargo.toml` file
  for using `template-test-wasm` crate/workspace:

```toml
template-test-wasm = "0.1.0"
```

## Testing the Project

- Run tests for crate/workspace `template-test`:

  ```console
  cd template-test && cargo test
  ```

- To run tests for crate/workspace `template-test-wasm`, follow
  the instructions in [template-test-wasm](./template-test-wasm#testing-the-project),
  which leverages [wasm-pack][wasm-pack].

## Setting-up template-test-wasm

The Wasm targetted version of this project relies on [wasm-pack][wasm-pack]
for building, testing, and publishing artifacts sutiable for
[Node.js][node-js], web broswers, or bundlers like [webpack][webpack].

Please read more on working with `wasm-pack` directly in
[template-test-wasm](./template-test-wasm#set-up).

## Contributing

:balloon: We're thankful for any feedback and help in improving our project!
We have a contributing guide to help you get involved. We also
adhere to our [Code of Conduct](./CODE_OF_CONDUCT.md).

### Nix
This repository contains a [Nix flake][nix-flake] that initiates both the Rust
toolchain set in [rust-toolchain.toml](./rust-toolchain.toml) and a
[pre-commit hook](#pre-commit-hook). It also installs helpful cargo binaries for
development. Please install [nix][nix] and [direnv][direnv] to get started.

Run `nix develop` or `direnv allow` to load the `devShell` flake output,
according to your preference.

### Formatting

For formatting Rust in particular, please use `cargo +nightly fmt` as it uses
specific nightly features we recommend by default.

### Pre-commit Hook

This library recommends using [pre-commit][pre-commit] for running pre-commit
hooks. Please run this before every commit and/or push.

- If you are doing interim commits locally, and for some reason if you _don't_
  want pre-commit hooks to fire, you can run
  `git commit -a -m "Your message here" --no-verify`.

### Recommended Development Flow

- We recommend leveraging [cargo-watch][cargo-watch],
  [cargo-expand][cargo-expand] and [irust][irust] for Rust development.
- We recommend using [cargo-udeps][cargo-udeps] for removing unused dependencies
  before commits and pull-requests.

### Conventional Commits

This project *lightly* follows the [Conventional Commits
convention][commit-spec-site] to help explain
commit history and tie in with our release process. The full specification
can be found [here][commit-spec]. We recommend prefixing your commits with
a type of `fix`, `feat`, `docs`, `ci`, `refactor`, etc..., structured like so:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

## Getting Help

For usage questions, usecases, or issues please open an issue in our repository.

We would be happy to try to answer your question or try opening a new issue on Github.

## External Resources

These are references to specifications, talks and presentations, etc.

## License

This project is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](./LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](./LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.


[apache]: https://www.apache.org/licenses/LICENSE-2.0
[cargo-expand]: https://github.com/dtolnay/cargo-expand
[cargo-udeps]: https://github.com/est31/cargo-udeps
[cargo-watch]: https://github.com/watchexec/cargo-watch
[commit-spec]: https://www.conventionalcommits.org/en/v1.0.0/#specification
[commit-spec-site]: https://www.conventionalcommits.org/
[direnv]:https://direnv.net/
[irust]: https://github.com/sigmaSd/IRust
[mit]: http://opensource.org/licenses/MIT
[nix]:https://nixos.org/download.html
[nix-flake]: https://nixos.wiki/wiki/Flakes
[node-js]: https://nodejs.dev/en/
[pre-commit]: https://pre-commit.com/
[wasm-pack]: https://rustwasm.github.io/docs/wasm-pack/
[webpack]: https://webpack.js.org/
