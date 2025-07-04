import test from 'ava'

import { nice, getCurrentProcessPriority, WindowsThreadPriority } from '../index.js'

test('should be able to call nice', (t) => {
  t.notThrows(() => {
    nice()
    nice(1)
  })
})

test('should be able to get current process priority', (t) => {
  t.is(typeof getCurrentProcessPriority(), 'number')
})

test('should be able to get WindowsThreadPriority', (t) => {
  t.is(WindowsThreadPriority.ThreadModeBackgroundBegin, 65536)
  t.is(WindowsThreadPriority.ThreadModeBackgroundEnd, 131072)
  t.is(WindowsThreadPriority.ThreadPriorityAboveNormal, 1)
  t.is(WindowsThreadPriority.ThreadPriorityBelowNormal, -1)
  t.is(WindowsThreadPriority.ThreadPriorityHighest, 2)
  t.is(WindowsThreadPriority.ThreadPriorityIdle, -15)
  t.is(WindowsThreadPriority.ThreadPriorityLowest, -2)
  t.is(WindowsThreadPriority.ThreadPriorityNormal, 0)
  t.is(WindowsThreadPriority.ThreadPriorityTimeCritical, 15)
})
