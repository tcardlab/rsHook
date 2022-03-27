# rsHook
rudimentary ioHook replacement written in Rust for Node

THIS IS NOT THE MAIN README, THIS IS JUST THE `NODE-BINDGEN` BRANCH
[see main branch README here](https://github.com/tcardlab/rsHook/tree/main)

## Build rsHook Yourself:
Assuming you are already in this directory
```shell
  $cargo install nj-cli
  $npm run build
  $npm run demo
```

## Basic Use:
```js
  const { rsHook } = require('@tcardlab/rshook')
  const callback = (event) => console.log(event)
  rsHook(callback)
```
