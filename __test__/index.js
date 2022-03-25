const { rsHook } = require('../index.js')
console.log(rsHook)
 

function tmp (Error, ...args) {
  console.log(args)
}
rsHook(tmp) 


const cb = testid => (Error, ...args) => console.log(`${testid}: ${args}`)

/* console.log("test 1")
rsHook(cb("test 1"))
console.log("Thread not blocked!\n") */


/* 
console.log("test 2")
setTimeout(rsHook, 0 , (e)=>console.log("hit!") , false);
console.log("Still not blocked! but prob no execution... \n")

// keeps counting, showing main is alive
while (true) {
  console.log(Date.now())
}
*/

/* console.log("test 3")
rsHook(cb(3), false)
console.log("got blocked!...\n") */


//test 4
/* const EventEmitter = require('events');
const myEmitter = new EventEmitter();

let i = 0
myEmitter.on('start', async () => {
  rsHook(()=>myEmitter.emit('event'), false)
});
myEmitter.on('event', () => console.log(i++) )

myEmitter.emit('start'); */
