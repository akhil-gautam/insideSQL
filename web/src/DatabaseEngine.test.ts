import {expect,test} from 'vitest';
import {buildDatabaseEngine,updateDatabaseEngine} from './DatabaseEngine';
import * as T from 'three';
import {enginePose} from './engineMotion';
test('every stage has distinct dense internal geometry and semantic annotations',()=>{
 const engine=buildDatabaseEngine();expect(engine.layers).toHaveLength(6);
 const counts=engine.layers.map(layer=>{let count=0;layer.traverse(object=>{if(object.type==='Mesh'||object.type==='InstancedMesh')count++;});return count;});
 expect(counts.every(count=>count>20)).toBe(true);
 expect(new Set(counts).size).toBeGreaterThanOrEqual(5);
 expect(engine.labels.filter(label=>label.layer===1).map(l=>l.text)).toContain('SelectStmt');
 expect(engine.labels.filter(label=>label.layer===2).map(l=>l.text)).toContain('pg_attribute');
 expect(engine.labels.filter(label=>label.layer===5).map(l=>l.text)).toContain('visible tuples → filter');
});

test('each chamber has axial depth and moving internals seek without accumulating transforms',()=>{
 const engine=buildDatabaseEngine();
 expect(engine.layers.every(layer=>new T.Box3().setFromObject(layer).getSize(new T.Vector3()).x>1.4)).toBe(true);
 const transforms=()=>engine.layers.map(layer=>{const values:number[]=[];layer.traverse(object=>values.push(...object.position.toArray(),object.rotation.x,object.rotation.y,object.rotation.z));return values;});
 updateDatabaseEngine(engine,enginePose(90,30,-1,-1,0,'mechanisms'));const first=transforms();
 updateDatabaseEngine(engine,enginePose(180,30,-1,-1,0,'mechanisms'));const next=transforms();
 expect(next.every((values,i)=>JSON.stringify(values)!==JSON.stringify(first[i]))).toBe(true);
 updateDatabaseEngine(engine,enginePose(90,30,-1,-1,0,'mechanisms'));expect(transforms()).toEqual(first);
});
