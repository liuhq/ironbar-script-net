import { diff_speed } from './diff'
import { print } from './print'
import { type NetBytes, read_total_net_bytes } from './read'
import { format_bytes_per_sec } from './unit'
import { poll } from './utils'

const bytes_stream = poll(read_total_net_bytes, 1000)

let prev: NetBytes | null = null

for await (const curr of bytes_stream()) {
    if (prev) {
        const speed = diff_speed(prev, curr)
        console.log(
            print(
                format_bytes_per_sec(speed.tx),
                format_bytes_per_sec(speed.rx),
            ),
        )
    }
    prev = curr
}
