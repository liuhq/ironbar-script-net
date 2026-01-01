const span = (text: string, color?: string) =>
    `<span${color ? ` color='${color}'` : ''}>${text}</span>`
const small = (text: string) => `<small>${text}</small>`
const sup = (text: string) => `<sup>${text}</sup>`
const frag = (texts: string[]) =>
    texts.reduce((prev, curr) => prev.concat(curr))

export const print = (up: string, down: string): string => {
    return span(
        small(frag([
            up,
            ' ',
            sup(span('', '#A3BE8C')),
            '  ',
            down,
            sup(span('', '#BF616A')),
        ])),
    )
}
