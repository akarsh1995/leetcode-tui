# Leetcode TUI

# Use Leetcode in your terminal.

![Demo](https://vhs.charm.sh/vhs-44GmR9FJgOHtWRU0UPVJam.gif)

### Why this TUI:

My motivation for creating leetcode-tui stemmed from my preference for tools that are lightweight and consume fewer system resources. When I explored existing leetcode CLI tools on GitHub, I came across a few raw command-line interfaces, but they lacked the interactivity I desired.
To address this, I decided to develop leetcode-tui, a Text-based User Interface, that provides an interactive and user-friendly experience for solving LeetCode problems.

> **Warning**
> This TUI is currently under active development. Please feel free to open an issue if you find errors.

## Installation

```sh
cargo install leetcode-tui-rs
```

Post installation

```sh
leetui

# This is going to create a config file depending on your os.

# Get the Cookies from the browser `LEETCODE_SESSION` and `csrftoken` and paste it in the config file

# run the command again to populate db
leetui
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
