# mber-rust

WARNING: THIS A WORK IN PROGRESS PROOF-OF-CONCEPT PHASE SOFTWARE.

```
cargo install mber
```
Build system consists of 2 packages:
- mber: a cli re-written in Rust, it provides an exceptionally fast user experience to already very fast mber build system.
- mber-rust: This is an npm package with neon/rust bindings. This package allows us to run a node.js runtime/JS execution context for fastboot while utilizing mber cli.
This npm library also exposes internal build functions written in rust to JavaScript via neon bindings.

Folders:
- ember-app-boilerplate: Boilerplate project that dynamically gets writen on $ mber new.
- lib: JS code used by mber-rust npm package.
- native: Rust code that runs inside node.js.
- src: Rust code used by mber cargo bin + package.
- tests: Integration tests for mber cargo bin + package.
- `_vendor`: holds certain addons that ship with mber build system, kept raw but gets injected and transpiled during compilation or runtime of mber binary.
- scripts: certain npm/node scripts to build ember packages from npm

Files:
- build.rs: Rust script that gets run before compilation
- index.js: Entrypoint for mber-rust npm package.
- package.json: holds dependencies for mber-rust npm package and devDependencies for mber development scripts

Required system software:
- Rust
- node.js
- festival(if you want your build system to warn you on failed builds via tts[text-to-speech])
