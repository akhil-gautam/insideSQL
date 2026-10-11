import {Fragment,useEffect,useMemo,useRef,useState} from 'react';
import {Player} from '@remotion/player';
import {useCurrentFrame} from 'remotion';
import {BookOpen,Check,ChevronRight,Database,Lock,Play,RotateCcw,Square,Terminal} from 'lucide-react';
import {call} from './api';
import type {Course,Progress,QueryResult,CheckResult} from './types';
import {QueryJourney,stages} from './QueryJourney';
import {SqlEditor} from './SqlEditor';
import {Results} from './Results';
import {Solution} from './Solution';
import {LabResources} from './LabResources';
import {convertFileSrc} from '@tauri-apps/api/core';
function Replay(){const frame=useCurrentFrame();const step=Math.floor(frame/60);return <QueryJourney frame={frame} stage={step} previousStage={Math.max(-1,step-1)} stageStartFrame={step*60}/>;}
const modules=['The SQL journey','Plans','Storage','B-tree','Access methods','Transformations','Advanced SQL','Planner','Executor','Concurrency','Durability','Scaling','Observability','Capstones'];
export default function App(){
  const [course,setCourse]=useState<Course|null>(null);const [progress,setProgress]=useState<Progress>({});const [selectedChapter,setSelectedChapter]=useState(0);const [selected,setSelected]=useState(0);
  const [sql,setSql]=useState('');const [result,setResult]=useState<QueryResult|null>(null);const [grade,setGrade]=useState<CheckResult|null>(null);const [busy,setBusy]=useState(false);const [runId,setRunId]=useState('');const [error,setError]=useState('');
  const [curriculumOpen,setCurriculumOpen]=useState(false);
  const [lessonTab,setLessonTab]=useState('explore');const [stage,setStage]=useState(-1);const [schema,setSchema]=useState(false);const [replay,setReplay]=useState(false);
  const drafts=useRef(Promise.resolve());
  useEffect(()=>{Promise.all([call<Course>('course'),call<Progress>('progress')]).then(([c,p])=>{setCourse(c);setProgress(p);const first=c.chapters[0].exercises[0];setSql(p[first.id]?.sql??first.starter);}).catch(e=>setError(String(e)));},[]);
  const chapter=course?.chapters[selectedChapter];const exercise=chapter?.exercises[selected];const completed=chapter?.exercises.filter(e=>progress[e.id]?.passed).length||0;
  const fixture=chapter&&course?.fixtures[chapter.fixture];
  const editorSchema=useMemo(()=>Object.fromEntries(fixture?.tables.map(t=>[t.name,Object.keys(t.columns)])||[]),[fixture]);
  const chapterUnlocked=(index:number)=>!!course?.chapters[index]?.published&&course.chapters.slice(0,index).every(c=>c.published&&c.exercises.every(e=>progress[e.id]?.passed));
  const refresh=async()=>{
    try{const current=await call<Progress>('progress');setProgress(current);return current;}
    catch(e){setProgress(p=>Object.fromEntries(Object.entries(p).map(([id,value])=>[id,{...value,analyzed:false}])));setResult(null);throw e;}
  };
  function edit(value:string){setSql(value);if(!exercise)return;const id=exercise.id;
    setProgress(p=>({...p,[id]:{unlocked:p[id]?.unlocked||false,passed:p[id]?.passed||false,sql:value,analyzed:p[id]?.sql===value&&p[id]?.analyzed||false}}));
    drafts.current=drafts.current.catch(()=>{}).then(()=>call<void>('save',{id,sql:value})).catch(e=>setError(`Draft could not be saved: ${String(e)}`));
  }
  async function submit(mode:string){if(!exercise)return;setBusy(true);setError('');setGrade(null);const id=crypto.randomUUID();setRunId(id);const input={exercise_id:exercise.id,sql,mode:mode==='check'?'run':mode,run_id:id};
    try{await drafts.current;if(mode==='check')setGrade(await call<CheckResult>('check',input));else setResult(await call<QueryResult>('run',input));await refresh();}catch(e){setError(String(e));}finally{setBusy(false);}}
  async function resetLab(){if(!exercise)return;setBusy(true);setError('');let failure='';
    try{await call('reset',{id:exercise.id});setResult(null);}catch(e){failure=String(e);}
    try{const current=await refresh();if(!current[exercise.id]?.analyzed)setResult(null);}
    catch(e){failure=[failure,`Lab status could not be refreshed: ${String(e)}`].filter(Boolean).join(' ');}
    finally{setError(failure);setBusy(false);}
  }
  function choose(i:number){if(busy||!chapter?.exercises[i])return;setSelected(i);const next=chapter.exercises[i];setSql(progress[next.id]?.sql??next.starter);setResult(null);setGrade(null);setError('');}
  function chooseChapter(i:number){if(busy||!course||!chapterUnlocked(i))return;setSelectedChapter(i);setSelected(0);const next=course.chapters[i].exercises[0];setSql(progress[next.id]?.sql??next.starter);setResult(null);setGrade(null);setError('');setStage(-1);setReplay(false);}
  if(!course||!chapter||!fixture)return <div className="boot"><Database size={32}/><h1>Preparing your database lab</h1><p>{error||'Starting the local PostgreSQL runtime…'}</p></div>;
  const chapterNumber=Number(chapter.id);const moduleName=modules[Math.floor((chapterNumber-1)/6)];const total=chapter.exercises.length;const nextChapter=course.chapters[selectedChapter+1];
  const media=(file:string)=>course.media_directory?convertFileSrc(`${course.media_directory}/${chapter.id}/${file}`):`/media/${chapter.id}/${file}`;
  return <div className={`academy${curriculumOpen?' curriculum-open':''}`}>
    <aside className="sidebar" aria-label="Course curriculum">
      <a className="brand" href="#workspace" aria-label="insideSQL workspace"><span className="brand-mark"><Database size={21} strokeWidth={1.5}/></span><span>inside<span className="brand-sql">SQL</span></span></a>
      <div className="course-summary"><h2>The PostgreSQL course</h2><p>From query to understanding.</p></div>
      <nav aria-label="Chapters">{course.chapters.map((c,i)=><Fragment key={c.id}>{i%6===0&&<div className="module-heading"><span>{String(Math.floor(i/6)+1).padStart(2,'0')}</span>{modules[Math.floor(i/6)]}</div>}<button className={`chapter${selectedChapter===i?' active':''}`} aria-label={`Chapter ${Number(c.id)}: ${c.title}`} aria-current={selectedChapter===i?'page':undefined} disabled={busy||!chapterUnlocked(i)} title={!c.published?'This chapter is being authored':!chapterUnlocked(i)?'Pass every exercise in preceding chapters':undefined} onClick={()=>{chooseChapter(i);setCurriculumOpen(false);}}><span className="chapter-number">{String(Number(c.id)).padStart(2,'0')}</span><span>{c.title}<small>{c.published?`${c.exercises.filter(e=>progress[e.id]?.passed).length}/${c.exercises.length} exercises passed`:'Being authored'}</small></span>{chapterUnlocked(i)?<ChevronRight size={13}/>:<Lock size={12}/>}</button></Fragment>)}</nav>
      <div className="sidebar-bottom"><span className="ready-dot"/><span>PostgreSQL {course.server_version}<small>Your local learning lab</small></span></div>
    </aside>
    <div className="app-main">
      <header className="app-header"><div><button className="curriculum-toggle" aria-label={curriculumOpen?'Hide curriculum':'Show curriculum'} aria-expanded={curriculumOpen} onClick={()=>setCurriculumOpen(o=>!o)}><BookOpen size={17}/></button><span>{moduleName}</span><ChevronRight size={13}/><strong>Chapter {chapterNumber}</strong></div><span className="offline"><span className="ready-dot"/> Saved on this device</span></header>
      <main id="workspace"><div className="chapter-heading"><div><h1>{chapter.title}</h1><p>{chapter.subtitle}</p><span className="sr-only">Chapter {chapterNumber} of {moduleName}</span></div><div className="chapter-progress"><span>{completed} <small>/ {total}</small></span><p>practices complete</p></div></div>
        <div className="workbench">
          <section className="lesson-lab" aria-label="Lesson and SQL lab">
            <div className="pane-heading"><span><BookOpen size={16} strokeWidth={1.6}/>Understand</span><span className="pane-note">Follow the query</span></div>
            <div className="lesson-tabs" role="tablist" aria-label="Lesson format">{['explore','video','transcript'].map(tab=><button role="tab" aria-selected={lessonTab===tab} key={tab} onClick={()=>setLessonTab(tab)}>{tab==='video'?'Watch lesson':tab==='explore'?'Explore the journey':'Transcript'}</button>)}</div>
            <div className="lesson-content" key={chapter.id}>{lessonTab==='video'?<video controls preload="metadata" src={media('chapter.mp4')} crossOrigin="anonymous" aria-label={`Chapter ${chapterNumber} narrated lesson`}><track kind="captions" src={media('captions.vtt')} srcLang="en" label="English" default/></video>:lessonTab==='explore'?<><QueryJourney stage={stage}/><div className="stage-buttons"><button aria-pressed={stage===-1} onClick={()=>setStage(-1)}>Overview</button>{stages.map((name,i)=><button key={name} aria-pressed={stage===i} onClick={()=>setStage(i)}>{name}</button>)}</div><details className="replay" onToggle={e=>setReplay(e.currentTarget.open)}><summary><Play size={12}/> Play the query journey</summary>{replay&&<Player component={Replay} numberOfSharedAudioTags={0} durationInFrames={360} compositionWidth={900} compositionHeight={780} fps={30} controls style={{width:'100%'}}/>}</details></>:<div className="transcript">{chapter.transcript.map((p,i)=><p key={i}>{p}</p>)}</div>}</div>
            <div className="lesson-footnote"><span className="footnote-mark"/>Illustrations explain the mechanism. Analyze shows what your query actually did.</div>
          </section>
          <section className="practice-pane" aria-label="SQL practice">
            <div className="pane-heading"><span><Terminal size={16} strokeWidth={1.6}/>Practice</span><span className="pane-note">A real PostgreSQL database</span></div>
            <div className="exercise-panel" aria-label="Chapter exercises">
              <div className="exercise-topline"><span className="exercise-position">Exercise {selected+1} of {total}</span><div className="exercise-nav">{chapter.exercises.map((e,i)=><button key={e.id} disabled={busy} aria-label={`Exercise ${i+1}: ${e.title}`} aria-current={selected===i?'step':undefined} className={progress[e.id]?.passed?'passed':''} onClick={()=>choose(i)}>{progress[e.id]?.passed?<Check size={13}/>:i+1}</button>)}</div></div>
              <div className="exercise-body"><h2>{exercise!.title}</h2><p>{exercise!.prompt}</p></div>
            </div>
            <section className="editor-section" aria-label="Query workspace"><div className="editor-heading"><span><span className="editor-file-dot"/> query.sql</span><button onClick={()=>setSchema(!schema)} aria-expanded={schema}><Database size={12}/>{fixture.title} schema</button></div>{schema&&<div className="schema">{fixture.tables.map(t=><div key={t.name}><strong>{t.name}</strong><code>{Object.entries(t.columns).map(([name,type])=>`${name} ${type}`).join(' · ')}</code></div>)}<p>{fixture.description}</p></div>}
              <SqlEditor value={sql} onChange={edit} schema={editorSchema} readOnly={busy}/>
              <div className="query-actions"><button className="run-query" disabled={busy} onClick={()=>submit('run')}><Play size={13} fill="currentColor"/>Run</button><button disabled={busy} onClick={()=>submit('explain')}>Explain</button><button disabled={busy} onClick={()=>submit('analyze')}>Analyze</button><span className="action-spacer"/>{busy?<button onClick={()=>call('cancel',{id:runId}).catch(e=>setError(String(e)))}><Square size={12}/>Cancel</button>:null}<button className="check-answer primary" disabled={busy} onClick={()=>submit('check')}><Check size={14}/>{busy?'Working…':'Check my query'}</button></div>
              {!!fixture.writable_tables?.length&&<p className="lab-notice">Run changes this disposable lab. Analyze measures and rolls back. Check uses fresh copies. Reset restores starting data.</p>}{exercise!.require_analyze&&<p className="lab-notice" role="status">{progress[exercise!.id]?.analyzed?'Current SQL has measured execution evidence.':'Analyze the current SQL before Check. After editing or resetting, measure it again.'}</p>}
              {error&&<p className="action-error" role="alert">{error}</p>}{grade&&<div className={`grade ${grade.passed?'success':''}`} role="status"><strong>{grade.passed?<><Check size={16}/>Passed</>:'Keep working'}</strong><p>{grade.message}</p></div>}
              <div className="results-area"><Results result={result}/></div>
            </section>
            <div className="practice-utilities"><LabResources key={exercise!.id} id={exercise!.id}/><button className="reset-lab" disabled={busy} title="Restore this exercise's fixtures" onClick={resetLab}><RotateCcw size={12}/>Reset lab</button></div>
            <Solution key={exercise!.id} unlocked={!!progress[exercise!.id]?.unlocked} load={()=>call('solution',{id:exercise!.id})}/>
            <div className="mastery-note"><progress value={completed} max={total} aria-label="Chapter completion"/>{completed===total?<><p className="complete" role="status">Chapter {chapterNumber} complete.{nextChapter&&!nextChapter.published?` Chapter ${Number(nextChapter.id)} is being authored.`:''}</p>{nextChapter&&chapterUnlocked(selectedChapter+1)&&<button className="continue-chapter" disabled={busy} onClick={()=>chooseChapter(selectedChapter+1)}>Continue to chapter {Number(nextChapter.id)}<ChevronRight size={15}/></button>}</>:<p>{total-completed} exercises remaining<span>Answers unlock after your first attempt.</span></p>}</div>
          </section>
        </div>
      </main>
    </div>
  </div>;
}
