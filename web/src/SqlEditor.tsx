import {useEffect,useRef} from 'react';
import {EditorView,basicSetup} from 'codemirror';
import {Compartment,EditorState} from '@codemirror/state';
import {HighlightStyle,syntaxHighlighting} from '@codemirror/language';
import {tags} from '@lezer/highlight';
import {sql,PostgreSQL} from '@codemirror/lang-sql';
export function SqlEditor({value,onChange,schema,readOnly=false}:{value:string;onChange:(value:string)=>void;schema:Record<string,string[]>;readOnly?:boolean}){
  const host=useRef<HTMLDivElement>(null);const view=useRef<EditorView|null>(null);const callback=useRef(onChange);callback.current=onChange;
  const editing=useRef(new Compartment());
  const dialect=useRef(new Compartment());
  useEffect(()=>{
    const editor=new EditorView({parent:host.current!,doc:value,extensions:[basicSetup,syntaxHighlighting(HighlightStyle.define([{tag:tags.keyword,color:'#537245'},{tag:tags.string,color:'#8a6845'},{tag:tags.number,color:'#5c7d64'},{tag:tags.comment,color:'#7c8971',fontStyle:'italic'},{tag:tags.operator,color:'#66775d'},{tag:tags.name,color:'#2c3c31'}])),editing.current.of([EditorView.editable.of(!readOnly),EditorState.readOnly.of(readOnly)]),dialect.current.of(sql({dialect:PostgreSQL,schema})),EditorView.lineWrapping,EditorView.contentAttributes.of({'aria-label':'SQL query'}),EditorView.updateListener.of(update=>{if(update.docChanged)callback.current(update.state.doc.toString());})]});view.current=editor;return()=>{editor.destroy();view.current=null;};
  },[]);
  useEffect(()=>{const editor=view.current;if(editor&&editor.state.doc.toString()!==value)editor.dispatch({changes:{from:0,to:editor.state.doc.length,insert:value}});},[value]);
  useEffect(()=>{view.current?.dispatch({effects:editing.current.reconfigure([EditorView.editable.of(!readOnly),EditorState.readOnly.of(readOnly)])});},[readOnly]);
  useEffect(()=>{view.current?.dispatch({effects:dialect.current.reconfigure(sql({dialect:PostgreSQL,schema}))});},[schema]);
  return <div ref={host} className="sql-editor"/>;
}
