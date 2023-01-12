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

function parseInputs(s) {
  const [type, code, time, str] = s.split(',')
  return { type, code, time, str }
}

function onErr(s) {
  console.log("rdevErr:", s)
  killSpawn()
}

function initSpawn(cd= (e)=>{ console.log(parseInputs(e)) }, err=onErr) {
  if (child) killSpawn(); // or just return, up to you.

  child = spawn(resolve(__dirname, cmd));  

  child.stdout.on('data', out => {
    cd(out.toString().trim('\n'))
  });
  child.stderr.on('data', out => {
    err(out.toString())
  });
}

module.exports = {
  initSpawn,
  killSpawn,
  parseInputs, // incase you just wanna wrap it
  //onErr      // prob doesn't need to be shared
}