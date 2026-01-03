# ironbar-script-net

A network speed monitoring script for Ironbar that displays real-time upload and download rates.

## Installation

### Download

Get from [release](https://github.com/liuhq/ironbar-script-net/releases)

### Build from source

Build steps:

```sh
git clone https://github.com/liuhq/ironbar-script-net.git
cd ironbar-script-net
cargo build --release
```

The built binary will be localed in `target/release/ironbar-script-net`.

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
