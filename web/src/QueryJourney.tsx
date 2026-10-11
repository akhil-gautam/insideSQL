import {useEffect,useId,useLayoutEffect,useRef,useState} from 'react';
import {cancelRender,continueRender,delayRender,getRemotionEnvironment} from 'remotion';
import {enginePose} from './engineMotion';
import {engineStages,stages} from './engineStages';
import '@fontsource/space-grotesk/400.css';
import '@fontsource/space-grotesk/500.css';
import '@fontsource/ibm-plex-mono/400.css';
type Props={frame?:number;fps?:number;stage?:number;previousStage?:number;stageStartFrame?:number;video?:boolean};
export function QueryJourney({frame,fps=30,stage:chosen=-1,previousStage,stageStartFrame,video=false}:Props){
 const id=useId().replace(/[^a-z0-9]/gi,'');const canvas=useRef<HTMLCanvasElement>(null);const labels=useRef<HTMLDivElement>(null);
 const renderer=useRef<ReturnType<typeof import('./DatabaseEngine').createDatabaseRenderer>|null>(null);
 const [tick,setTick]=useState(0),[paused,setPaused]=useState(false),[reduced,setReduced]=useState(false),[error,setError]=useState(''),[ready,setReady]=useState(false);
 const clock=frame??tick;const transition=useRef({stage:chosen,previous:chosen,start:clock});
 if(transition.current.stage!==chosen)transition.current={stage:chosen,previous:transition.current.stage,start:clock};
 const pose=enginePose(clock,fps,chosen,previousStage??transition.current.previous,stageStartFrame??transition.current.start,id,reduced||paused);
 const latest=useRef(pose);latest.current=pose;
 useEffect(()=>{const media=window.matchMedia('(prefers-reduced-motion: reduce)');const update=()=>setReduced(media.matches);update();media.addEventListener('change',update);return()=>media.removeEventListener('change',update);},[]);
 useEffect(()=>{if(frame!==undefined||paused||reduced||error)return;let request=0,last=performance.now();
  const advance=(now:number)=>{const elapsed=now-last;if(elapsed>=1000/fps){setTick(t=>t+elapsed*fps/1000);last=now;}request=requestAnimationFrame(advance);};
  request=requestAnimationFrame(advance);return()=>cancelAnimationFrame(request);
 },[frame,paused,reduced,error,fps]);
 useLayoutEffect(()=>{let stopped=false;const rendering=getRemotionEnvironment().isRendering;const handle=rendering?delayRender('Preparing database engine geometry and fonts'):null;
  const element=canvas.current!;
  const lost=(event:Event)=>{event.preventDefault();setError('The 3D view lost its graphics context. The layer details remain available below.');};element.addEventListener('webglcontextlost',lost);
  let observer:ResizeObserver|undefined;
  (async()=>{try{
   const [{createDatabaseRenderer}]=await Promise.all([import('./DatabaseEngine'),document.fonts.load('400 16px "IBM Plex Mono"'),document.fonts.load('500 16px "Space Grotesk"')]);
   if(stopped)return;
   renderer.current=createDatabaseRenderer(element,labels.current!,rendering);renderer.current.draw(latest.current);setReady(true);
   observer=new ResizeObserver(()=>renderer.current?.draw(latest.current));observer.observe(element);
  }catch(e){if(rendering)cancelRender(e instanceof Error?e:new Error(String(e)));else setError('Your graphics device could not open the 3D view. You can still explore every layer using the controls and details below.');}
  finally{if(handle!==null)continueRender(handle);}})();
  return()=>{stopped=true;observer?.disconnect();element.removeEventListener('webglcontextlost',lost);renderer.current?.dispose();renderer.current=null;};
 },[]);
 useLayoutEffect(()=>{if(!error)renderer.current?.draw(pose);},[clock,chosen,previousStage,stageStartFrame,reduced,paused,error]);
 const stage=engineStages[chosen];
 return <div className={`journey-scene${video?' journey-video':''}`}>
  <div className="engine-heading"><div><h2>{stage?.title??'Inside the database engine.'}</h2><p>{stage?.subtitle??'Six mechanisms. One SQL statement. Explore the machinery within each layer.'}</p></div><span className="illustration-label">Illustrated mechanism</span></div>
  <div className="engine-viewport"><canvas ref={canvas} role="img" aria-label={stage?`SQL journey: ${stage.name} is highlighted`:'Exploded database engine with six detailed mechanical layers'}/><div className="engine-annotations" ref={labels} aria-hidden="true"/>{!ready&&!error&&<span className="engine-loading" role="status">Assembling the engine…</span>}{error&&<p className="engine-fallback" role="status">{error}</p>}
   {!video&&frame===undefined&&<button className="engine-pause" onClick={()=>setPaused(p=>!p)} disabled={reduced} aria-pressed={paused||reduced}>{reduced?'Reduced motion':paused?'Resume motion':'Pause motion'}</button>}
  </div>
  <div className="engine-details"><div className="engine-contract"><span>{stage?'Input → output':'Statement under inspection'}</span><code>{stage?`${stage.input} → ${stage.output}`:"SELECT id, total FROM orders WHERE status = 'paid' ORDER BY id;"}</code></div>{stage?<><div className="engine-internals">{stage.details.map(detail=><span key={detail}>{detail}</span>)}</div><p className="engine-explanation">{stage.explanation}</p></>:<p className="engine-explanation">Select a layer to move inside it. Follow tokens, trees, catalogs, paths and pages; then inspect your own query in the SQL workspace.</p>}</div>
 </div>;
}
export {stages};
