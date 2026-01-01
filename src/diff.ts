import type { NetBytes } from './read'

export const diff_speed = (prev: NetBytes, curr: NetBytes) => {
    const dt = (curr.time - prev.time) / 1000

    return Object.freeze({
        rx: (curr.rx - prev.rx) / dt,
        tx: (curr.tx - prev.tx) / dt,
    })
}
