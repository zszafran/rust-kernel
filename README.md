# Experimenting with a Rust Kernel

More info here: https://os.phil-opp.com/

## VSCode

- `CTRL + SHIFT + B` - Build kernel

## QEMU

- `CTRL + ALT + 2` then `quit` to exit

> **Linux**: Run `xhost +local:` on the host machine

> **OSX**: Install `xQuartz`, enable `Allow connections from network clients`, and run `xhost + 127.0.01` on the host machine. Set env `DISPLAY=docker.for.mac.host.internal:0` inside the docker container.

## TODO

- [ ] [Naked Exceptions](https://os.phil-opp.com/first-edition/extra/naked-exceptions/)