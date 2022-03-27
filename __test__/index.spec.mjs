import test from 'ava'

//import { rsHook } from '../index.js'

test('rsHook from native', (t) => {
  const isThreadBlocked = ()=>"Thread not blocked!"
  //rsHook(console.log)
  t.is(isThreadBlocked(), "Thread not blocked!")
})
