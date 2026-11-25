# RUUI
RUUI is a Rust framework that is currently a heavy WIP.

I am currently experimenting with for creating cross-platform native UI apps.

If you have any suggestions or want to get involved by all means feel free to jump in and raise a ticket or discussion!

I have only lightly touched on Svelte but the purpose of this project is to be a Svelte-inspired UI framework for Rust.
If you have any suggestions on how to improve the syntax this is still in the early days.

If I release any apps with this in use I will link this. If you decide to make anything also please raise a ticket and ill link it :)

## Why do this?
I couldn't find anything that could go cross platform with native components also to wearables especially.
Also, I have a tendency to do things from scratch or re-invent the wheel a bit even when its not needed <.<
but it's for the sake of fun and learning! 
Also, it's an excuse to put a learn a lot and put a lot of the newer skills ive learnt over the years into practice.

I have been asked why not just use AI, you'll save a lot of time. And while I won't stop people using AI to assist their contributions,
the whole point of this project is to make it as human friendly as possible rather than vibe coding something that works.
I think there are a lot of basic apps that really don't need AI to be made, but nowadays the barrier to entry in a lot of projects can very quickly get overwhelming.

## Raising Issues
Never be afraid to open a ticket asking for help, if you can't find something in the documentation (once its made) then either it's possibly not documented, or could be made clearer as to where it is.
Please outline where you have tried to look and what info you need and this can open the door to some proper discussion on what we can do better!

## Project Structure
The repository is a Cargo workspace organized around a platform-independent UI
contract and separate rendering backends:

```text
ruui/
├── crates/
│   ├── ruui/                        Public API
│   ├── ruui-macros/                 tbd (need to explain this better)
│   ├── ruui-(platform)-(framework)/ Specific platform and framework bindings
│   ├── ruui-build/                  Project and Cargo tooling
│   └── ruui-cli/                    Cli tool to trigger ruui-build and other assistive tools
├── examples/                        Examples of how to use
└── Cargo.toml                       Workspace members and shared dependencies
```

## Testing Locally
Just create a new example :) or point to the repo locally. I'll add an example here later.

## Getting Started
While you may be able to cross compile for different platforms, 
for now the instructions will include getting them working on each base platform.

For all platforms you will need [Rust](https://rustup.rs/) to be installed.

Install the `ruui` cli tool via
```bash
cargo install --path crates/ruui-cli
```
To check if everything is set up you can run `ruui doctor` and it will the tools you need are detected.

### Desktop
#### macOS
Make sure that xcode is installed. Running `xcode-select --install` Should state its already installed.

### Mobile

### Wearable
