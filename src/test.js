const { initSpawn, killSpawn, parseInputs } = require('./index')

// type "quit" to terminate spawn
let quitArr = [16, 22, 23, 20]
let downRegister = []
function callback (e) {
  // Log event
  let obj = parseInputs(e)
  console.log(obj)

  // Update register
  if (obj.type==="keydown") downRegister.push(+obj.code)
  if (downRegister.length > 4) downRegister.shift()
  
  // Kill on "quit"
  let isQuit = quitArr.every((v, i) => v === downRegister[i])
  if (isQuit) killSpawn()
}

initSpawn(callback)
