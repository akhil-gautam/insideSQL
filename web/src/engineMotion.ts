import {eases} from 'animejs';
import {journeyAt} from './journey';
export function enginePose(frame:number,fps:number,stage:number,previous:number,start:number,instance:string,reduced=false){
  const seconds=Math.max(0,frame)/fps;
  const {progress}=journeyAt(Math.max(0,frame)%360,fps,instance);
  const blend=reduced?1:eases.outExpo(Math.min(1,Math.max(0,(frame-start)/fps/1.6)));
  return {seconds,progress,stage,previous,blend,orbit:reduced?0:Math.sin(progress*Math.PI*2)*0.045,spread:reduced?1:0.94+0.06*eases.inOutSine((Math.sin(progress*Math.PI*2)+1)/2)};
}
