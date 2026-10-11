import {useEffect,useRef} from 'react';
import {EditorView,basicSetup} from 'codemirror';
import {Compartment,EditorState} from '@codemirror/state';
import {sql,PostgreSQL} from '@codemirror/lang-sql';
export function SqlEditor({value,onChange,readOnly=false}:{value:string;onChange:(value:string)=>void;readOnly?:boolean}){
  const host=useRef<HTMLDivElement>(null);const view=useRef<EditorView|null>(null);const callback=useRef(onChange);callback.current=onChange;
  const editing=useRef(new Compartment());
  useEffect(()=>{
    const editor=new EditorView({parent:host.current!,doc:value,extensions:[basicSetup,editing.current.of([EditorView.editable.of(!readOnly),EditorState.readOnly.of(readOnly)]),sql({dialect:PostgreSQL,schema:{orders:['id','customer_id','status','total','ordered_at']}}),EditorView.lineWrapping,EditorView.contentAttributes.of({'aria-label':'SQL query'}),EditorView.updateListener.of(update=>{if(update.docChanged)callback.current(update.state.doc.toString());}),EditorView.theme({'&':{height:'210px',fontSize:'14px'},'.cm-scroller':{fontFamily:'ui-monospace, SFMono-Regular, Menlo, monospace'},'.cm-content':{padding:'16px 0'},'.cm-gutters':{background:'#f3f6fa',color:'#8997a9',border:'none'}})]});view.current=editor;return()=>{editor.destroy();view.current=null;};
  },[]);
  useEffect(()=>{const editor=view.current;if(editor&&editor.state.doc.toString()!==value)editor.dispatch({changes:{from:0,to:editor.state.doc.length,insert:value}});},[value]);
  useEffect(()=>{view.current?.dispatch({effects:editing.current.reconfigure([EditorView.editable.of(!readOnly),EditorState.readOnly.of(readOnly)])});},[readOnly]);
  return <div ref={host} className="sql-editor"/>;
}
