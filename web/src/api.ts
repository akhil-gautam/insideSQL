import {invoke,isTauri} from '@tauri-apps/api/core';
export async function call<T>(command:string,input:Record<string,unknown>={}):Promise<T>{
  if(isTauri())return invoke<T>(command,['run','check'].includes(command)?{request:input}:input);
  const response=await fetch(`/api/${command}`,{method:'POST',headers:{'Content-Type':'application/json'},body:JSON.stringify(input)});
  const value=await response.json();if(!response.ok)throw new Error(value.error||'The database could not complete this action.');return value;
}
