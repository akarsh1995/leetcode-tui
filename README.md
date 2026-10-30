# Leetcode TUI

# Use Leetcode in your terminal.

![Demo](https://vhs.charm.sh/vhs-44GmR9FJgOHtWRU0UPVJam.gif)

### Why this TUI:

My motivation for creating leetcode-tui stemmed from my preference for tools that are lightweight and consume fewer system resources. When I explored existing leetcode CLI tools on GitHub, I came across a few raw command-line interfaces, but they lacked the interactivity I desired.
To address this, I decided to develop leetcode-tui, a Text-based User Interface, that provides an interactive and user-friendly experience for solving LeetCode problems.

> **Warning**
> This TUI is currently under active development. Please feel free to open an issue if you find errors.

## Installation

### Using Cargo

```sh
cargo install leetcode-tui-rs
```

### Using [Nix](https://nix.dev/)

You can use leetcode-tui in several ways with Nix:

#### Quick Run (No Installation)
```sh
# Run directly without installing
nix run github:akarsh1995/leetcode-tui
```

#### Imperative Installation
```sh
# Install imperatively in your user profile
nix profile install github:akarsh1995/leetcode-tui
```

#### Development Environment
```sh
# Get a development shell with all dependencies
nix shell github:akarsh1995/leetcode-tui
```

## Post Installation

```sh
leetui
```

On first run a config file is created (its path is printed) and a LeetCode
login window opens. Sign in there — the session cookies are captured and saved
to the config file (`chmod 600`), so you only need to do this again when the
session expires.

```sh
# Sign in again / refresh an expired session
leetui login

# Open the login window even though the stored session is still valid
leetui login --force

# Switch accounts: forget the stored session and the embedded browser profile
leetui login --reset

# Sign in using the session of a browser you are already signed in with
leetui login --from-browser
leetui login --from-browser --browser firefox

# Check the stored session
leetui auth
```

The window captures the session as soon as it changes, and closing it also
finishes the flow. Cookies that match the session you already have are ignored,
so an already signed-in window stays open until you actually sign in or close
it.

Inside the TUI, `L` re-opens the login window.

> **Note**
> Google refuses to complete sign-in inside embedded browsers. If you normally
> sign in with Google or GitHub, set a password via "Forgot Password?" on
> leetcode.com first, then use the login window. Alternatively, use
> `leetui login --from-browser` to reuse the session from your regular browser.

Credentials can also be provided through the `LEETCODE_SESSION` and
`LEETCODE_CSRFTOKEN` environment variables, which take precedence over the
config file.

The login behaviour can be tuned in the config file:

```toml
[auth]
# Open the embedded login window when no valid session is found.
webview = true
# Browsers used as a fallback when the login window cannot be used, in order.
browsers = ["chrome", "firefox", "safari"]
```

## Features

- Question grouped by categories
- Read Question
- Open question in `EDITOR`
- Solve question in multiple languages
- Submit and run solution in multiple languages
- Read Stats of your performance (Ctrl+s)
- Solved questions are marked with "👑"
- Neetcode 75
- For Fuzzy search the question list use `/` -- searchable by question id, topic and question_title.

Few related projects:

- [https://github.com/skygragon/leetcode-cli](https://github.com/skygragon/leetcode-cli)
- [https://github.com/clearloop/leetcode-cli](https://github.com/clearloop/leetcode-cli)
