import {render,screen,fireEvent,cleanup} from '@testing-library/react';
import {afterEach,expect,test} from 'vitest';
import {Solution} from './Solution';
afterEach(cleanup);
test('answer stays hidden until an attempt, then opens only on request',async()=>{
  const answer=()=>Promise.resolve({sql:'SELECT id FROM orders',explanation:'Return identifiers.'});
  const view=render(<Solution unlocked={false} load={answer}/>);
  expect(screen.getByRole('button').hasAttribute('disabled')).toBe(true);
  expect(screen.queryByText('SELECT id FROM orders')).toBeNull();
  view.rerender(<Solution unlocked load={answer}/>);
  expect(screen.queryByText('SELECT id FROM orders')).toBeNull();
  fireEvent.click(screen.getByRole('button'));
  expect(await screen.findByText('SELECT id FROM orders')).toBeTruthy();
});
