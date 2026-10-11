import {expect,test} from 'vitest';
import {renderToStaticMarkup} from 'react-dom/server';
import {DatabaseDiagram} from './DatabaseDiagram';
test('each database stage exposes the structures it teaches without a graphics context',()=>{
 const expected=['SELECT','SelectStmt','pg_attribute','Rewritten Query','Index scan','Visible tuples'];
 for(let stage=0;stage<6;stage++){
  const svg=renderToStaticMarkup(<DatabaseDiagram stage={stage} previous={stage} time={0}/>);
  expect(svg).toContain(expected[stage]);expect(svg).toContain('role="img"');expect(svg).not.toContain('<canvas');
 }
 const overview=renderToStaticMarkup(<DatabaseDiagram stage={-1} previous={-1} time={0}/>);
 expect(overview).toContain('Shared buffers');expect(overview).toContain('heap pages');expect(overview).toContain('Result rows');
});
test('diagram seeking restores the same data flow and page state',()=>{
 const draw=(time:number)=>renderToStaticMarkup(<DatabaseDiagram stage={5} previous={4} time={time} blend={1}/>);
 const held=draw(3);expect(draw(8)).not.toEqual(held);expect(draw(3)).toEqual(held);
});
