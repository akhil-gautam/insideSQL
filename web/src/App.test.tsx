import {afterEach,expect,test,vi} from 'vitest';
import {cleanup,fireEvent,render,screen,waitFor} from '@testing-library/react';
import type {Progress} from './types';
import chapter01 from '../../course/chapter01.json';
import chapter02 from '../../course/chapter02.json';
import chapter03 from '../../course/chapter03.json';
import fixtureCatalog from '../../course/fixtures/catalog.json';
import App from './App';
import {call} from './api';

vi.mock('./api',()=>({call:vi.fn()}));
// jsdom has no layout engine; CodeMirror still requests range geometry.
Range.prototype.getClientRects=()=>[] as unknown as DOMRectList;
Range.prototype.getBoundingClientRect=()=>new DOMRect();
afterEach(()=>{cleanup();vi.clearAllMocks();vi.unstubAllGlobals();});

function start({published=true,passes=true,exerciseCount=6,nativeRoot,commerce=false}:{published?:boolean;passes?:boolean;exerciseCount?:number;nativeRoot?:string;commerce?:boolean}={}){
  const chapters=[{...chapter01,fixture:'orders'},{...chapter02,fixture:'orders',published,exercises:chapter02.exercises.slice(0,exerciseCount)},...(commerce?[{...chapter03,published:true}]:[])];
  const progress:Progress=Object.fromEntries(chapters.flatMap(c=>c.exercises.map(e=>[e.id,{unlocked:false,passed:passes&&(c.id==='01'||commerce&&c.id==='02'),sql:e.id==='02-01'?'':null}])));
  vi.mocked(call).mockImplementation(async(command,input)=>{
    if(command==='course')return {chapter:chapters[0],chapters,fixtures:fixtureCatalog,planned_chapters:84,planned_exercises:504,available_chapters:published?2:1,server_version:'18.3',media_directory:nativeRoot};
    if(command==='progress')return structuredClone(progress);
    if(command==='save'){progress[String(input!.id)].sql=String(input!.sql);return;}
    throw new Error(`Unexpected command: ${command}`);
  });
  render(<App/>);
  return progress;
}

test('published chapters need every prior pass and unpublished chapters stay locked',async()=>{
  start({passes:false});
  const second=await screen.findByRole('button',{name:/Chapter 2:/});
  expect(second.hasAttribute('disabled')).toBe(true);
  cleanup();vi.clearAllMocks();
  start({published:false});
  expect((await screen.findByRole('button',{name:/Chapter 2:/})).hasAttribute('disabled')).toBe(true);
});

test('an unlocked chapter uses its own lesson, counters and saved drafts',async()=>{
  const saved=start({exerciseCount:5});
  const second=await screen.findByRole('button',{name:/Chapter 2:/});
  expect(second.hasAttribute('disabled')).toBe(false);
  fireEvent.click(screen.getByRole('button',{name:'Continue to chapter 2'}));
  expect(screen.getByRole('heading',{name:chapter02.title})).toBeTruthy();
  expect(screen.getByLabelText('Chapter 2 narrated lesson').getAttribute('src')).toBe('/media/02/chapter.mp4');
  expect(screen.getByRole('textbox',{name:'SQL query'}).textContent).toBe('');
  const draft='SELECT id FROM orders -- chapter two draft';
  fireEvent.input(screen.getByRole('textbox',{name:'SQL query'}),{target:{textContent:draft}});
  await waitFor(()=>expect(saved['02-01'].sql).toBe(draft));
  fireEvent.click(screen.getByRole('button',{name:/Exercise 2:/}));
  fireEvent.click(screen.getByRole('button',{name:/Chapter 1:/}));
  fireEvent.click(screen.getByRole('button',{name:/Chapter 2:/}));
  expect(screen.getByRole('textbox',{name:'SQL query'}).textContent).toBe(draft);
  expect(screen.getByText('5 exercises remaining')).toBeTruthy();
  expect(screen.getByText('Exercise 1 of 5')).toBeTruthy();
  expect(screen.getByLabelText('Chapter completion').getAttribute('max')).toBe('5');
  fireEvent.click(screen.getByRole('tab',{name:'Transcript'}));
  expect(screen.getByText(chapter02.transcript[0])).toBeTruthy();
});

test('chapter and exercise navigation stay fixed until a query finishes',async()=>{
  start();
  await screen.findByRole('button',{name:/Chapter 2:/});
  let finish:()=>void=()=>{};
  const pending=new Promise<void>(resolve=>{finish=resolve;});
  const normal=vi.mocked(call).getMockImplementation()!;
  vi.mocked(call).mockImplementation(async(command,input)=>{
    if(command==='run'){await pending;return {columns:[],column_types:[],rows:[],plan:null,error:null,truncated:false,elapsed_ms:0};}
    return normal(command,input);
  });
  fireEvent.click(screen.getByRole('button',{name:'Run'}));
  expect(screen.getByRole('button',{name:/Chapter 2:/}).hasAttribute('disabled')).toBe(true);
  expect(screen.getByRole('button',{name:/Exercise 2:/}).hasAttribute('disabled')).toBe(true);
  finish();
  await waitFor(()=>expect(screen.getByRole('button',{name:/Chapter 2:/}).hasAttribute('disabled')).toBe(false));
});

test('native video paths include the selected chapter under the shared media root',async()=>{
  vi.stubGlobal('__TAURI_INTERNALS__',{convertFileSrc:(path:string)=>`asset://${path}`});
  start({nativeRoot:'/Application Resources/media'});
  fireEvent.click(await screen.findByRole('button',{name:/Chapter 2:/}));
  expect(screen.getByLabelText('Chapter 2 narrated lesson').getAttribute('src')).toBe('asset:///Application Resources/media/02/chapter.mp4');
  expect(document.querySelector('track')?.getAttribute('src')).toBe('asset:///Application Resources/media/02/captions.vtt');
});

test('commerce chapter exposes all of its tables in the schema panel and editor completion',async()=>{
  start({commerce:true});
  fireEvent.click(await screen.findByRole('button',{name:/Chapter 3:/}));
  fireEvent.click(screen.getByRole('button',{name:'commerce schema'}));
  expect(screen.getByText(fixtureCatalog.commerce.description)).toBeTruthy();
  for(const table of fixtureCatalog.commerce.tables){
    expect(screen.getByText(table.name,{selector:'.schema strong'})).toBeTruthy();
  }
  const editor=screen.getByRole('textbox',{name:'SQL query'});
  fireEvent.input(editor,{target:{textContent:'SELECT * FROM pay'}});
  editor.focus();fireEvent.keyDown(editor,{key:' ',code:'Space',ctrlKey:true});
  expect(await screen.findByRole('option',{name:'payments'})).toBeTruthy();
});
