import {useEffect,useId,useRef,useState} from 'react';
import {continueRender,delayRender,getRemotionEnvironment} from 'remotion';
import {Pause,Play} from 'lucide-react';
import {enginePose} from './engineMotion';
import {engineStages,stages} from './engineStages';
import {DatabaseDiagram} from './DatabaseDiagram';
type Props={frame?:number;fps?:number;stage?:number;previousStage?:number;stageStartFrame?:number;video?:boolean};
export function QueryJourney({frame,fps=30,stage:chosen=-1,previousStage,stageStartFrame,video=false}:Props){
 const id=useId().replace(/[^a-z0-9]/gi,'');
 const [tick,setTick]=useState(0),[paused,setPaused]=useState(false),[reduced,setReduced]=useState(false);
 const heldFrame=useRef(0);
 const clock=frame??(paused?heldFrame.current:tick);const transition=useRef({stage:chosen,previous:chosen,start:clock});
 if(transition.current.stage!==chosen)transition.current={stage:chosen,previous:transition.current.stage,start:clock};
 const pose=enginePose(clock,fps,chosen,previousStage??transition.current.previous,stageStartFrame??transition.current.start,id,reduced||paused);
 useEffect(()=>{if(!window.matchMedia)return;const media=window.matchMedia('(prefers-reduced-motion: reduce)');const update=()=>setReduced(media.matches);update();media.addEventListener('change',update);return()=>media.removeEventListener('change',update);},[]);
 useEffect(()=>{if(frame!==undefined||paused||reduced)return;let request=0,last=performance.now();
  const advance=(now:number)=>{const elapsed=now-last;if(elapsed>=1000/fps){setTick(t=>t+elapsed*fps/1000);last=now;}request=requestAnimationFrame(advance);};
  request=requestAnimationFrame(advance);return()=>cancelAnimationFrame(request);
 },[frame,paused,reduced,fps]);
 useEffect(()=>{if(frame===undefined||!document.fonts||!getRemotionEnvironment().isRendering)return;const handle=delayRender('Loading diagram fonts');document.fonts.ready.then(()=>continueRender(handle));},[]);
 const stage=engineStages[chosen];
 return <div className={`journey-scene${video?' journey-video':''}`}>
  <div className="engine-heading"><div><h2>{stage?.title??'A query, from the inside.'}</h2><p>{stage?.subtitle??'Follow the path from a statement to the rows it returns.'}</p></div><span className="illustration-label">Illustrated mechanism</span></div>
  {!video&&<p className="engine-scroll-hint">Scroll across to explore the diagram.</p>}
  <div className="engine-viewport" tabIndex={!video?0:undefined} aria-label={!video?"Database diagram; scroll horizontally on narrow screens":undefined}><DatabaseDiagram stage={chosen} previous={pose.previous} blend={pose.blend} time={reduced?0:pose.seconds}/>
   {!video&&frame===undefined&&<button className="engine-pause" onClick={()=>{if(!paused)heldFrame.current=tick;setPaused(p=>!p);}} disabled={reduced} aria-pressed={paused||reduced}>{paused||reduced?<Play size={9}/>:<Pause size={9}/>} {reduced?'Reduced motion':paused?'Resume motion':'Pause motion'}</button>}
  </div>
  <div className="engine-details"><div className="engine-contract"><span>{stage?'Input → output':'The statement'}</span><code>{stage?`${stage.input} → ${stage.output}`:"SELECT id, total FROM orders WHERE status = 'paid' ORDER BY id;"}</code></div>{stage?<><div className="engine-internals">{stage.details.map(detail=><span key={detail}>{detail}</span>)}</div><p className="engine-explanation">{stage.explanation}</p></>:<p className="engine-explanation">Choose a stage below. Look inside its structures, then investigate your own query in the workspace.</p>}</div>
 </div>;
}
export {stages};
