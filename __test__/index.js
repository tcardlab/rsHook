const { rsHook } = require('../');

send = (e)=>console.log(e)
rsHook(send)
console.log('Not Blocked')
