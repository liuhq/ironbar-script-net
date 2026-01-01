await Bun.build({
    entrypoints: ['./src/main.ts'],
    compile: {
        target: 'bun-linux-x64',
        outfile: './dist/ironbar-script-net',
    },
    minify: true,
})
