# rsHook
rudimentary ioHook replacement

**This is a demo, it is not intended for production use.**

# Getting Started From Git:
this has no js deps aside from node itself.
However, it does have rust deps, so make sure you have rust installed.

```sh
# Test
npm run rs:build
npm test # type "quit" to end

# Start
npm run rs:build
npm start

# build
npm run build
npm run build:test
```


# Getting Started From NPM:
If you downloaded from npm, you will have to build it from within node_modules
Otherwise, the binary will *probably* not work... (built on old mac)

```sh
cd node_modules/@tcardlab/rshook-spawn
npm run build

npm run build:test # to ensure it worked
```
