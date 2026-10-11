import {createRafDriver,getProject,types} from '@theatre/core';
import state from '../../course/theatre/journey.json';
const scenes=new Map<string,{sheet:ReturnType<ReturnType<typeof getProject>['sheet']>;object:{readonly value:{progress:number}};driver:ReturnType<typeof createRafDriver>}>();
export function journeyAt(frame:number,fps:number,instance:string){
  let scene=scenes.get(instance);
  if(!scene){const project=getProject(`Journey-${instance}`,{state});const sheet=project.sheet('Journey');const object=sheet.object('Flow',{progress:types.number(0,{range:[0,1]})});scene={sheet,object,driver:createRafDriver({name:instance})};scenes.set(instance,scene);}
  scene.sheet.sequence.position=Math.max(0,Math.min(12,frame/fps));
  scene.driver.tick(frame/fps*1000);
  return {progress:scene.object.value.progress};
}
