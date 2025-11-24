import test from 'ava'
import { KzgWrapper } from '../index'

test('KzgWrapper loads and exposes methods', (t) => {
  t.truthy(KzgWrapper)
  t.true(typeof KzgWrapper.loadKzg === 'function')
})
