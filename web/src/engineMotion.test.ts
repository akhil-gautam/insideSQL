import {expect,test} from 'vitest';
import {enginePose} from './engineMotion';
test('cinematic seeking is deterministic, isolated and motion can be stopped',()=>{
 const pose=enginePose(180,30,2,1,150,'cinematic-a');
 enginePose(320,30,5,4,280,'cinematic-b');
 enginePose(250,30,2,1,150,'cinematic-a');
 expect(enginePose(180,30,2,1,150,'cinematic-a')).toEqual(pose);
 expect(enginePose(240,30,2,1,150,'cinematic-a').blend).toBe(1);
 const still=enginePose(180,30,2,1,150,'cinematic-still',true);
 expect(still.orbit).toBe(0);expect(still.spread).toBe(1);expect(still.blend).toBe(1);
});
