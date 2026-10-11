import {useEffect,useState} from 'react';
import {call} from './api';
import type {ResourceSnapshot} from './types';

const mib=(bytes:number)=>`${(bytes/1024/1024).toFixed(1)} MiB`;
export function LabResources({id}:{id:string}){
 const [open,setOpen]=useState(false);
 const [data,setData]=useState<ResourceSnapshot|null>(null);
 const [error,setError]=useState('');
 useEffect(()=>{
  if(!open)return;
  let stale=false;setData(null);setError('');
  call<ResourceSnapshot>('resources',{id}).then(value=>{if(!stale)setData(value);}).catch(e=>{if(!stale)setError(String(e));});
  return()=>{stale=true;};
 },[id,open]);
 return <details className="lab-resources" onToggle={e=>setOpen(e.currentTarget.open)}><summary>Lab resources</summary>{open&&<div>
  {error?<p role="alert">{error} Close and reopen to try again.</p>:data?<>
   <dl className="operation-fields">{[
    ['Database storage',mib(data.database_bytes)],['Available disk space',mib(data.available_disk_bytes)],
    ['Allocation floor',mib(data.minimum_free_disk_bytes)],['Shared buffers',data.shared_buffers],
    ['Work memory allowance',data.work_mem],['Temporary spill allowance',data.temp_file_limit]
   ].map(([name,value])=><div key={name}><dt>{name}</dt><dd>{value}</dd></div>)}</dl>
   {data.available_disk_bytes<data.minimum_free_disk_bytes&&<p role="status">Free up disk space before running or resetting this lab. Saved scripts and existing lab data are kept when the preflight rejects the operation.</p>}
   <p>Database storage excludes WAL, logs and temporary files.</p>
   <p>Work memory is per operation; spill allowance is per PostgreSQL process.</p>
   <p>Observed when opened. These settings are allowances, not total memory usage or a performance score. The disk check is a preflight, not a storage reservation.</p>
  </>:<p role="status">Reading lab resources…</p>}
 </div>}</details>;
}
