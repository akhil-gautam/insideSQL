import {AbsoluteFill,Audio,staticFile,useCurrentFrame,useVideoConfig} from 'remotion';
import {QueryJourney} from '../QueryJourney';
import narration from '../generated/narration.json';
import '../style.css';
const stagesBySection=[0,0,1,2,3,4,4,5,5,5,2,5];
const transitions=narration.captions.filter((caption,i,all)=>i===0||stagesBySection[caption.section]!==stagesBySection[all[i-1].section]);
export function ChapterVideo(){
 const frame=useCurrentFrame();const {fps}=useVideoConfig();const time=frame/fps;
 const caption=narration.captions.find(c=>time>=c.start&&time<c.end);
 const index=Math.max(0,transitions.reduce((last,c,i)=>c.start<=time?i:last,0));const transition=transitions[index];
 const stage=stagesBySection[transition.section];const previous=index===0?-1:stagesBySection[transitions[index-1].section];
 return <AbsoluteFill style={{background:'#201e1d',color:'#fff0df'}}>
  <Audio src={staticFile('media/01/narration.m4a')}/>
  <div style={{flex:1,minHeight:0}}><QueryJourney frame={frame} fps={fps} stage={stage} previousStage={previous} stageStartFrame={Math.round(transition.start*fps)} video/></div>
  <div className="video-stage-strip" aria-hidden>{Array.from({length:6},(_,i)=><span key={i} className={stage===i?'active':''}/>)}</div>
  <div className="video-caption">{caption?.text||'Define the result. Observe the engine. Verify the change.'}</div>
 </AbsoluteFill>;
}
export function EngineCinematic(){
 const frame=useCurrentFrame();const step=Math.floor(frame/90)-1;
 return <AbsoluteFill><QueryJourney frame={frame} stage={step} previousStage={Math.max(-1,step-1)} stageStartFrame={(step+1)*90} video/></AbsoluteFill>;
}
