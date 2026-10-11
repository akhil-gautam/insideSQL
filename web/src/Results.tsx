import {useEffect,useMemo,useState} from 'react';
import {ReactFlow,Background,Controls,type Node,type Edge,MarkerType,Position} from '@xyflow/react';
import '@xyflow/react/dist/style.css';
import {flexRender,getCoreRowModel,getPaginationRowModel,useReactTable} from '@tanstack/react-table';
import type {QueryResult,PlanNode} from './types';
function operationTitle(plan:PlanNode){return [plan['Node Type'],plan['Relation Name'],plan['Alias'],plan['Subplan Name']].filter(v=>v!==undefined).join(' · ');}
function OperationDetails({plan}:{plan:PlanNode}){
  return <dl className="operation-fields">{Object.entries(plan).filter(([name])=>name!=='Plans'&&name!=='Node Type').map(([name,value])=><div key={name}><dt>{name}</dt><dd>{typeof value==='object'?<pre>{JSON.stringify(value,null,2)}</pre>:String(value)}</dd></div>)}</dl>;
}
function PlanGraph({plan}:{plan:PlanNode}){
  const [selected,setSelected]=useState('0');
  useEffect(()=>setSelected('0'),[plan]);
  const {nodes,edges,operations}=useMemo(()=>{
    const nodes:Node[]=[];const edges:Edge[]=[];const operations:{id:string;plan:PlanNode}[]=[];let leaf=0;
    const walk=(operation:PlanNode,depth:number,parent?:string):number=>{
      const id=String(nodes.length);operations.push({id,plan:operation});
      const measured=operation['Actual Rows']!==undefined;
      const label=`${operationTitle(operation)}\nEstimated rows: ${operation['Plan Rows']??'—'}${measured?`\nActual rows: ${operation['Actual Rows']} · loops: ${operation['Actual Loops']??'—'}`:''}`;
      const target:Node={id,data:{label},sourcePosition:Position.Top,targetPosition:Position.Bottom,position:{x:0,y:depth*160},style:{width:200,fontSize:12,padding:12,border:'1px solid #bbcae0',borderRadius:8,whiteSpace:'pre-line'}};
      nodes.push(target);
      if(parent)edges.push({id:`${parent}-${id}`,source:id,target:parent,markerEnd:{type:MarkerType.ArrowClosed}});
      const xs=(operation.Plans||[]).map(child=>walk(child,depth+1,id));
      const x=xs.length?xs.reduce((a,b)=>a+b,0)/xs.length:leaf++*240;
      target.position.x=x;return x;
    };walk(plan,0);return {nodes,edges,operations};
  },[plan]);
  const current=operations.find(n=>n.id===selected)||operations[0];
  return <><div className="plan-graph"><ReactFlow nodes={nodes.map(n=>({...n,selected:n.id===current.id}))} edges={edges} onNodeClick={(_,node)=>setSelected(node.id)} fitView nodesDraggable={false} nodesConnectable={false} aria-label="Execution plan graph"><Background gap={20} color="#e1e7f0"/><Controls showInteractive={false}/></ReactFlow></div><section className="operation-inspector" aria-label="Selected plan operation"><label>Inspect a plan operation<select value={current.id} onChange={e=>setSelected(e.target.value)}>{operations.map(n=><option key={n.id} value={n.id}>{Number(n.id)+1}. {operationTitle(n.plan)}</option>)}</select></label><h3>{current.plan['Node Type']}</h3><OperationDetails plan={current.plan}/></section></>;
}
function PlanTree({plan}:{plan:PlanNode}){return <li><details open><summary>{operationTitle(plan)}</summary><OperationDetails plan={plan}/>{plan.Plans&&<ul>{plan.Plans.map((child,i)=><PlanTree key={i} plan={child}/>)}</ul>}</details></li>;}
export function Results({result}:{result:QueryResult|null}){
  const [tab,setTab]=useState('graph');
  const columns=useMemo(()=>result?.columns.map((name,i)=>({id:String(i),header:name,accessorFn:(row:(string|null)[])=>row[i],cell:(info:{getValue:()=>unknown})=>info.getValue()===null?<i className="null">NULL</i>:String(info.getValue())}))||[],[result]);
  const table=useReactTable({data:result?.rows||[],columns,getCoreRowModel:getCoreRowModel(),getPaginationRowModel:getPaginationRowModel(),initialState:{pagination:{pageSize:50}}});
  if(!result)return <div className="result-empty"><strong>See what your query does.</strong><p>Run SQL for results, or choose Analyze for measured execution evidence.</p></div>;
  if(result.error)return <div className="query-error" role="alert"><strong>{result.error.code||'Query error'}</strong><p>{result.error.message}</p>{result.error.position&&<small>At character {result.error.position}</small>}</div>;
  if(result.plan){const plan=result.plan[0];const measured=plan.Plan['Actual Rows']!==undefined;return <><div className="result-toolbar"><span className="measured">{measured?'Measured PostgreSQL execution':'PostgreSQL planner estimates'}</span><div className="segmented">{['graph','tree','json'].map(t=><button key={t} aria-pressed={tab===t} onClick={()=>setTab(t)}>{t}</button>)}</div><span>{result.elapsed_ms} ms client elapsed</span></div>{tab==='graph'?<PlanGraph plan={plan.Plan}/>:tab==='tree'?<ul className="plan-tree" aria-label="Plan operations"><PlanTree plan={plan.Plan}/></ul>:<pre className="raw-plan">{JSON.stringify(plan,null,2)}</pre>}<p className="plan-caveat">Actual rows and node times are per-loop averages; zero loops mean the operation did not execute. Costs, times and buffer counters can include child work; do not add them as independent totals. Cost units are not milliseconds. Graph arrows point from child operations to parents.</p></>;}
  return <><div className="result-toolbar"><span className="measured">{result.rows.length} rows{result.truncated?' · display limit reached':''}</span><span>{result.elapsed_ms} ms client elapsed</span></div><div className="result-table"><table><thead>{table.getHeaderGroups().map(group=><tr key={group.id}>{group.headers.map(header=><th key={header.id}>{flexRender(header.column.columnDef.header,header.getContext())}</th>)}</tr>)}</thead><tbody>{table.getRowModel().rows.map(row=><tr key={row.id}>{row.getVisibleCells().map(cell=><td key={cell.id}>{flexRender(cell.column.columnDef.cell,cell.getContext())}</td>)}</tr>)}</tbody></table></div>{result.rows.length>50&&<div className="pagination"><button disabled={!table.getCanPreviousPage()} onClick={()=>table.previousPage()}>Previous rows</button><button disabled={!table.getCanNextPage()} onClick={()=>table.nextPage()}>Next rows</button></div>}</>;
}
