const fs = require("fs")
const { resolve } = require("path")
const { exec } = require("child_process")


let dist = resolve(__dirname, "dist")
function prepDist() {
  if (fs.existsSync(dist)) {
    fs.rmSync(dist, {recursive: true})
  }
  fs.mkdirSync(dist);
}

let bin = resolve(__dirname, "target/release/rsHook")
function moveBinary() {
  if (!fs.existsSync(bin)) bin += '.exe' // off the top of my head, not sure if needed for win 
  fs.copyFileSync(bin, resolve(dist, 'rsHook'))
}

function repathIndex(code) {
  return code.replace('../target/release/rsHook', './rsHook')
}

function moveIndex() {  
  let code = fs.readFileSync("src/index.js").toString()
  code = repathIndex(code)

  fs.writeFileSync(resolve(dist, "index.js"), code)
}

function main() {
  exec("cargo build --release", (error, stdout, stderr) => {
    if (error) return console.log(`error: ${error.message}`);
    //if (stderr) return console.log(`stderr: ${stderr}`);
    console.log(stdout)
  })

  prepDist()
  moveBinary()
  moveIndex()
}

main()