import { readdir } from 'node:fs/promises'

export type NetBytes = Readonly<{
    rx: number
    tx: number
    time: number
}>

const SYS_NET = '/sys/class/net'

const get_ifaces = async () => await readdir(SYS_NET)

const read_bytes = async (iface: string) =>
    Promise
        .all([
            Bun.file(`${SYS_NET}/${iface}/statistics/rx_bytes`).text(),
            Bun.file(`${SYS_NET}/${iface}/statistics/tx_bytes`).text(),
        ])
        .then(([rx, tx]) => ({
            rx: Number(rx.trim()),
            tx: Number(tx.trim()),
        }))

export const read_total_net_bytes = async (): Promise<NetBytes> => {
    const ifaces = (await get_ifaces()).filter((i) => i != 'lo')

    const totals = await Promise.all(ifaces.map(read_bytes))

    const sum = totals.reduce((acc, cur) => ({
        rx: acc.rx + cur.rx,
        tx: acc.tx + cur.tx,
    }))

    return Object.freeze({
        ...sum,
        time: Date.now(),
    })
}
