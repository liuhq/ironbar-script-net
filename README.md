# ironbar-script-net

A network speed monitoring script for Ironbar that displays real-time upload and download rates.

## Installation

### Archlinux

Get from AUR [ironbar-script-net](https://aur.archlinux.org/packages/ironbar-script-net)

### Build from source

**Requirements**:

- [bun](https://bun.com/)

Build steps:

```sh
git clone https://github.com/liuhq/ironbar-script-net.git
cd ironbar-script-net
bun build.ts
```

The built binary will be localed in `dist/ironbar-script-net`.

## Usage

Add the following code to `~/.config/ironbar/config.corn`:

```corn
let {
  $net = {
    type = "custom"
    class = "ironbar-script-net"
    bar = [
      {
        type = "label"
        label = "{{watch:ironbar-script-net}}"
      }
    ]
  }
} in {
  // add to anywhere if you want
  end = [ $net ]
}
```

## License

[MIT](./LICENSE)
