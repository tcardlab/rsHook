# rsHook
rudimentary ioHook replacement written in Rust for Node

THIS IS NOT THE MAIN README, THIS IS JUST THE `NAPI-RS` BRANCH
[see main branch README here](https://github.com/tcardlab/rsHook/tree/main)

<br/>

## Build-Free Package (Win/Mac only)
    npm i @tcardlab/rshook


## Build rsHook Yourself:
Assuming you are already in this directory
```shell
  $npm install -g @napi-rs/cli
  $npm i
  $npm run build
  $npm run demo
```


## Basic Use:
```js
  const { rsHook } = require('@tcardlab/rshook')
  const callback = (error, ...event) => console.log(event)
  rsHook(callback)
```
