import {useState} from 'react';
import * as Collapsible from '@radix-ui/react-collapsible';
export function Solution({unlocked,load}:{unlocked:boolean;load:()=>Promise<{sql:string;explanation:string}>}) {
  const [answer,setAnswer]=useState<{sql:string;explanation:string}|null>(null);
  const [error,setError]=useState('');
  return <Collapsible.Root className="solution" onOpenChange={open=>{if(open&&!answer)load().then(setAnswer).catch(e=>setError(String(e)));}}>
    <Collapsible.Trigger disabled={!unlocked}>{unlocked?'Show reference answer':'Try a query to unlock the answer'}</Collapsible.Trigger>
    <Collapsible.Content>{error?<p role="alert">{error}</p>:answer?<><pre>{answer.sql}</pre><p>{answer.explanation}</p></>:<p>Loading answer…</p>}</Collapsible.Content>
  </Collapsible.Root>;
}
