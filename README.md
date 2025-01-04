# wrk-wrk

## Introduction

This repository is an educational side project completely written in [Rust](https://www.rust-lang.org/) to explore and learn capabilities of [tauri](https://tauri.app/) and [leptos](https://leptos.dev/) for native android app development.

Its purpose is to track your daily tasks as developer and provide some basic statistics.

All data is stored locally with [sqlite](https://www.sqlite.org/).

There will be no releases provided at all, but it should be _relatively_ easy to set up using the following steps.

## Prerequisites

- download and intall Rust: https://www.rust-lang.org/learn/get-started
- install the nightly toolchain: https://rust-lang.github.io/rustup/concepts/channels.html#working-with-nightly-rust
- install the webassembly target: `rustup target add wasm32-unknown-unknown`
- install the rust-src component: `rustup component add rust-src`
- install [trunk](https://trunkrs.dev/): `cargo install trunk`
- install [tauri-cli](https://v1.tauri.app/v1/api/cli/): `cargo install tauri-cli`
- install the latest JDK and [Android Studio](https://developer.android.com/studio) for your operation system: https://tauri.app/start/prerequisites/#configure-for-mobile-targets

## Setup

- clone this repository: `git clone https://github.com/mh84/wrk-wrk.git && cd wrk-wrk`
- set rust nightly toolchain as global default (`rustup default nightly`) OR use the directory override: `rustup override set nightly`
- init tauri android environment: `cargo tauri android init`
- set up signing: https://tauri.app/distribute/sign/android/

## Build

- build the android apk: `cargo tauri android build`
- deploy to your device using adb: `adb install ./src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release.apk`

## Final notes

Feel free to open any issue or pull request if you encounter any bugs or regressions and want to collaborate.

Feel free to fork this project and improve or adjust things at your will.
