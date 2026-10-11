import {expect,test} from 'vitest';
import {journeyAt} from './journey';
test('seeking backward reproduces values and another player cannot change them',()=>{
  const first=journeyAt(90,30,'first');
  expect(first.progress).toBeGreaterThan(0);
  journeyAt(330,30,'second');
  journeyAt(240,30,'first');
  expect(journeyAt(90,30,'first')).toEqual(first);
  expect(journeyAt(0,30,'first').progress).toBe(0);
});
