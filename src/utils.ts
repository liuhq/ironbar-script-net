export const poll = <T>(effect: () => Promise<T>, interval: number) =>
    async function*() {
        while (true) {
            yield await effect()
            await new Promise((r) => setTimeout(r, interval))
        }
    }
