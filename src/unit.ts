export type Speed = Readonly<{
    value: number
    unit: 'B/s' | 'KB/s' | 'MB/s' | 'GB/s'
}>

const UNITS = [
    { unit: 'B/s', factor: 1 },
    { unit: 'KB/s', factor: 1024 },
    { unit: 'MB/s', factor: 1024 ** 2 },
    { unit: 'GB/s', factor: 1024 ** 3 },
] as const

const to_human_speed = (bytes_per_sec: number): Speed => {
    const abs = Math.abs(bytes_per_sec)

    const chosen = [...UNITS].reverse().find((u) => abs >= u.factor) ?? UNITS[0]

    return Object.freeze({
        value: bytes_per_sec / chosen.factor,
        unit: chosen.unit,
    })
}

const format_speed = (speed: Speed, digits = 1): string =>
    `${speed.value.toFixed(digits)} ${speed.unit}`

export const format_bytes_per_sec = (
    bytes_per_sec: number,
    digits = 1,
): string => format_speed(to_human_speed(bytes_per_sec), digits)
