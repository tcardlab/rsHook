# rsHook
Rudimentary ioHook replacement written in Rust for Node.


### Two Implementations:
**[Napi-RS Version:](https://github.com/tcardlab/rsHook/tree/napi-rs)**

You can use the Build-Free Package `npm i @tcardlab/rshook` (Win/Mac only).<br/>
Otherwise, you can build it yourself (See below for details). <br/>
*NOTE: First argument sent to callback is for errors. Structure callback arguments like so:*
```js
function callback (error, ...event) { console.log(event) }
```

**[Node-Bindgen Version:](https://github.com/tcardlab/rsHook/tree/node-bindgen)**

You have to build this one yourself (See below for details).

<br/>

### Building rsHook Yourself:
You may import it as "node module" from github:
 - Napi: `npm i https://github.com/tcardlab/rsHook/tarball/napi-rs`
 - Bindgen: `npm i https://github.com/tcardlab/rsHook/tarball/node-bindgen`

Then, add the following script to `package.json` to build with `npm run build-rs`:
```json
{
  "scripts": {
    "build-rs": "cd node_modules/@tcardlab/rshook && npm run build"
  }  // If this fails, you may need a cli tool. See relevant branch for info.
}
```
Otherwise, you may download/clone the repo via github and follow the relevant branch instructions.

<br/>

### Linux:
I have not tested linux nor been able to build it, so you are on your own. <br/>
These packages depend heavily upon the [rdev crate](https://github.com/Narsil/rdev). So, you may have to look there for info and trouble shooting. Their github CI implies `libxtst-dev` and  `libevdev-dev` are dependencies.

<br/>

### Output Parity:
Because it was easy, rsHook returns an array of strings `[eventType, scanCode, timestamp, inputName]`
Example:
```js
  [ 'keyup', '37', '1648357401244', 'KeyK' ]
  [ 'mousedown', '0', '1648357399405', 'Left' ]
```

If you want to create a simple wrapper to match ioHook:
```js
  const { rsHook: ogHook } = require('@tcardlab/rshook')
  function rsHook (callback) {
    //const wrapper = (ogEvent) {  // Node-Bindgen
    const wrapper = (Error, ...ogEvent) => { // Napi-RS
      let ioHookEvent = {
        type: ogEvent[0],
        [e[0].includes('key') ? 'keycode' : 'button']: +e[1]
        // etc.
      }
      callback(ioHookEvent)
    }

    ogHook(wrapper)
  }
```

<br/>

### To-Do:
**General**
 - [ ] Pass mouse location
 - [ ] ioHook parity
 - [ ] [Open-Trade/rdev](https://github.com/open-trade/rdev) fork has some interesting work
 - [ ] Use `@rsHook/napi` and `@rsHook/bindgen` org-scoping
 - [ ] [Consider run-on-arch-action for CI](https://github.com/uraimo/run-on-arch-action)
 - [ ] Fix mac `calling TIS/TSM in non-main thread`. [solution?](https://github.com/open-trade/rdev/commit/8962d0a27f70bc727e1e2dcd51b61d390ccfd6f3)
 - [ ] A termination feature (could cancel thread, but macs don't like running it in thread... so maybe [process::exit(1) or test if panic works](https://stackoverflow.com/questions/21569718/how-do-i-exit-a-rust-program-early-from-outside-the-main-function))

**Node-Bindgen**
 - [ ] CI for mac, win, and linux
 - [ ] Tests
 
**Napi-rs**
 - [ ] CI Linux builds 
 - [ ] Tests
