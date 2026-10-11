import {expect,test} from 'vitest';
import {buildDatabaseEngine} from './DatabaseEngine';
test('every stage has distinct dense internal geometry and semantic annotations',()=>{
 const engine=buildDatabaseEngine();expect(engine.layers).toHaveLength(6);
 const counts=engine.layers.map(layer=>{let count=0;layer.traverse(object=>{if(object.type==='Mesh'||object.type==='InstancedMesh')count++;});return count;});
 expect(counts.every(count=>count>20)).toBe(true);
 expect(new Set(counts).size).toBeGreaterThanOrEqual(5);
 expect(engine.labels.filter(label=>label.layer===1).map(l=>l.text)).toContain('SelectStmt');
 expect(engine.labels.filter(label=>label.layer===2).map(l=>l.text)).toContain('pg_attribute');
 expect(engine.labels.filter(label=>label.layer===5).map(l=>l.text)).toContain('visible tuples → filter');
});
