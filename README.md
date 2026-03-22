# Zee

[![Zed](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/zed-industries/zed/main/assets/badge/v0.json)](https://zed.dev)
[![CI](https://github.com/zed-industries/zed/actions/workflows/run_tests.yml/badge.svg)](https://github.com/zed-industries/zed/actions/workflows/run_tests.yml)

`zee` is a workflow-focused fork of [Zed](https://github.com/zed-industries/zed) for people who keep multiple editors, terminals, and dock panels open at once and do not want project navigation constantly shoving the workspace around.

This fork currently focuses on two practical changes:

- dock panels can render as `overlay` or `push`, globally or per panel
- the Project Panel can sort by `name` or `modified_time`, with configurable direction and grouping

`main` stays aligned with upstream Zed. `zee` is the integrated fork branch where these workflow changes land.

## Why This Fork Exists

In stock Zed, opening dock-backed panels often resizes the workspace. That is fine in some layouts, but it becomes frustrating when you are repeatedly checking the project tree, switching between panels, and trying to keep multiple editor windows stable on screen.

`zee` explores a different default: panels should be available when needed without constantly reflowing code, terminals, and neighboring windows.

## What Is Already Different

### 1. Dock Panel Display Modes

You can keep the default push behavior, switch everything to overlay, or mix both behaviors by panel.

```jsonc
{
  "dock_panel_mode": "push",
  "dock_panel_modes": {
    "ProjectPanel": "overlay",
    "GitPanel": "push",
    "OutlinePanel": "overlay",
    "CollaborationPanel": "overlay",
    "TerminalPanel": "overlay"
  }
}
```

`dock_panel_mode` is the global fallback. `dock_panel_modes` overrides it per panel.

### 2. Project Panel Sorting

The Project Panel now supports grouping, sort field, and sort direction through settings JSON.

```jsonc
{
  "project_panel": {
    "sort_mode": "directories_first",
    "sort_by": "modified_time",
    "sort_direction": "descending"
  }
}
```

Current supported values:

- `sort_mode`: `directories_first`, `mixed`, `files_first`
- `sort_by`: `name`, `modified_time`
- `sort_direction`: `ascending`, `descending`

## Demos

README demo assets are in progress. The current plan is to add short animated captures for:

- overlay vs push behavior
- mixed panel modes in the same dock
- optional multi-window workflow use

Until those land, the settings examples above are the quickest way to try the fork behavior locally.

## Branch Model

- `main` tracks upstream Zed
- `zee` is the integrated fork branch
- short-lived feature branches merge into `zee`

This keeps the fork easy to update against upstream while still making the custom behavior visible in one branch.

## Building Zee

This fork builds the same way upstream Zed does.

- [Building Zed for macOS](./docs/src/development/macos.md)
- [Building Zed for Linux](./docs/src/development/linux.md)
- [Building Zed for Windows](./docs/src/development/windows.md)

If you want the official editor build, downloads and package manager installs are still available from [zed.dev](https://zed.dev/download).

## Contributing

This repository is a fork of [zed-industries/zed](https://github.com/zed-industries/zed). Upstream context still matters.

- Fork-specific workflow changes land on `zee`
- Upstream-compatible fixes can be evaluated separately for pull requests
- See [CONTRIBUTING.md](./CONTRIBUTING.md) for the base project contribution guidance

## Licensing

License information for third party dependencies must be correctly provided for CI to pass.

We use [`cargo-about`](https://github.com/EmbarkStudios/cargo-about) to automatically comply with open source licenses. If CI is failing, check the following:

- Is it showing a `no license specified` error for a crate you've created? If so, add `publish = false` under `[package]` in your crate's Cargo.toml.
- Is the error `failed to satisfy license requirements` for a dependency? If so, first determine what license the project has and whether this system is sufficient to comply with this license's requirements. If you're unsure, ask a lawyer. Once you've verified that this system is acceptable add the license's SPDX identifier to the `accepted` array in `script/licenses/zed-licenses.toml`.
- Is `cargo-about` unable to find the license for a dependency? If so, add a clarification field at the end of `script/licenses/zed-licenses.toml`, as specified in the [cargo-about book](https://embarkstudios.github.io/cargo-about/cli/generate/config.html#crate-configuration).
