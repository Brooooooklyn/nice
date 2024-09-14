import test from 'ava'

import { nice, getCurrentProcessPriority } from '../index.js'

test('should be able to call nice', (t) => {
  t.notThrows(() => {
    nice(1)
  })
})

test('should be able to get current process priority', (t) => {
  t.is(typeof getCurrentProcessPriority(), 'number')
})
