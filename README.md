Note: this is a toy rust app, you might prefer awk or [fpp](https://facebook.github.io/PathPicker/) or w/e.

This app reads STDIN and returns a unique list of things that look like files. I use it to open files based on program output (e.g. stacktraces)

# Usage

- Build with `cargo build`
- Set up as a tmux binding or pipe STDIN into the app

## How I use it

I start an nvim session per tmux session that lives in window 0 pane 0 with:

```
nvim --listen /tmp/nvim-(tmux list-sessions | grep "(attached)" | cut -d: -f 1 )-socket $argv;
```

Then I can open files from tmux in that nvim instance using the following tmux binding:

```
bind e capture-pane \; save-buffer /tmp/tmux-buffer \; new-window -c "#{pane_current_path}" 'vim-send-maybe (cat /tmp/tmux-buffer | filepicker | fzf)'
```

That assumes `filepicker` is in your path and `vim-send-maybe` is
defined as:

```bash

if [ $# -eq 0 ]
  then exit 0
fi

session=$(tmux list-sessions | grep "(attached)" | cut -d":" -f 1)

nvim --server "/tmp/nvim-$session-socket" --remote "$@"

tmux selectw -t 0
tmux selectp -t 0
```
