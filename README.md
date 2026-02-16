# cef-rs message_router

Rust port of CEF's official ["message_router" example](https://github.com/chromiumembedded/cef-project/tree/master/examples/message_router)
implementation.
It demonstrates how to create JavaScript bindings using CefMessageRouter
while providing a practical sample for the cef-rs ecosystem.

Currently, this port has only been confirmed to work on macOS and Linux with Xorg.

## Usage

### macOS

1. [Install Shared CEF Binaries](https://github.com/tauri-apps/cef-rs/tree/dev?tab=readme-ov-file#install-shared-cef-binaries)
2. [Set Environment Variables](https://github.com/tauri-apps/cef-rs/tree/dev?tab=readme-ov-file#set-environment-variables)
3. `cargo run --bin bundle-cef-app -- message_router -o target` will create the application bundle on `./target/message_router.app`

### Linux

1. [Install Shared CEF Binaries](https://github.com/tauri-apps/cef-rs/tree/dev?tab=readme-ov-file#install-shared-cef-binaries)
2. [Set Environment Variables](https://github.com/tauri-apps/cef-rs/tree/dev?tab=readme-ov-file#set-environment-variables)
3. Verify that the `chrome-sandbox` in the CEF binaries folder has the appropriate permissions.
   Make sure that the `chrome-sandbox` is owned by root and has mode 4755.
4. `cargo run --bin message_router` will open this application.

## Acknowledgments

This software is based on these works.

- [Original C++ implementation](https://github.com/chromiumembedded/cef-project/tree/master/examples/message_router)
- [cef-rs' cefsimple example](https://github.com/tauri-apps/cef-rs/tree/dev/examples/cefsimple)
- [cef-rs' tests_shared utilities](https://github.com/tauri-apps/cef-rs/tree/dev/examples/tests_shared)
- [cef-rs' bundle tool](https://github.com/tauri-apps/cef-rs/tree/dev/cef/src/bin/bundle-cef-app)

## License

This port is provided under the [Unlicense](LICENSE).

However, this project contains code and assets derived from the
[CEF project](https://github.com/chromiumembedded/cef-project)
and [cef-rs](https://github.com/tauri-apps/cef-rs),
which is subject to the following license:

- **cef-project**: [BSD 3-Clause License](third_party/cef-project-license.txt)
- **cef-rs**
    - [MIT License](third_party/cef-rs-license-mit.txt)
    - [Apache-2.0 License](third_party/cef-rs-license-apache.txt)
