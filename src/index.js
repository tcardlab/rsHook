const { spawn } = require('child_process');
const { resolve } = require('path')

let child;
let cmd = '../target/release/rsHook'

function killSpawn() {
  if (!child) return // don't kill undef...

  // Safely closes existing child process
  child.stdin.end()
  child.kill()  // Filicide
}

function callback(s) {
  console.log(s)
  const [type, code, time, str] = s.split(',')
  return { type, code, time, str }
}

function onErr(s) {
  console.log("rdevErr:", s)
  killSpawn()
}

function initSpawn() {
  if (child) killSpawn(); // or just return, up to you.

  child = spawn(resolve(__dirname, cmd));  

  child.stdout.on('data', out => {
    callback(out.toString())
  });
  child.stderr.on('data', out => {
    onErr(out.toString())
  });
}

initSpawn()