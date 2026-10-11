import type {ReactNode} from 'react';

const c={ink:'#2f3e32',muted:'#607255',line:'#b0c0a4',paper:'#ffffff',soft:'#edf3e6',green:'#6d9155',dark:'#426a35',pale:'#d8e6c8',side:'#dce6d0'};
function Text({x,y,children,size=12,muted=false,anchor='start'}:{x:number;y:number;children:ReactNode;size?:number;muted?:boolean;anchor?:'start'|'middle'|'end'}){return <text x={x} y={y} fill={muted?c.muted:c.ink} fontSize={Math.max(11,size)} textAnchor={anchor}>{children}</text>}
function Node({x,y,w=112,label,sub,active=false}:{x:number;y:number;w?:number;label:string;sub?:string;active?:boolean}){return <g><rect x={x-w/2} y={y-19} width={w} height={sub?52:38} rx="5" fill={active?c.soft:c.paper} stroke={active?c.green:c.line}/><Text x={x} y={y+4} anchor="middle" size={12}>{label}</Text>{sub&&<Text x={x} y={y+22} anchor="middle" size={11} muted>{sub}</Text>}</g>}
function Flow({d,time=0,active=false}:{d:string;time?:number;active?:boolean}){return <g><path d={d} fill="none" stroke={c.line} strokeWidth="1.3"/>{active&&<path d={d} fill="none" stroke={c.green} strokeWidth="2" strokeDasharray="3 65" strokeDashoffset={-time*15}/>}</g>}
function Table({x,y,title,rows,w=210,selected=-1}:{x:number;y:number;title:string;rows:string[][];w?:number;selected?:number}){return <g><rect x={x+5} y={y+5} width={w} height={30+rows.length*25} rx="5" fill="#e6eddf"/><rect x={x} y={y} width={w} height={30+rows.length*25} rx="5" fill={c.paper} stroke={c.line}/><path d={`M${x},${y+30}h${w}`} stroke={c.line}/><Text x={x+12} y={y+20} size={11}>{title}</Text>{rows.map((row,i)=><g key={i}>{selected===i&&<rect x={x+1} y={y+31+i*25} width={w-2} height={24} fill={c.soft}/>}<path d={`M${x+8},${y+55+i*25}h${w-16}`} stroke="#e9eee3"/>{row.map((v,j)=><Text key={j} x={x+12+j*(w-20)/row.length} y={y+48+i*25} size={10} muted={selected!==i}>{v}</Text>)}</g>)}</g>}
function Heap({x,y,time=0,compact=false}:{x:number;y:number;time?:number;compact?:boolean}){
 const cell=Math.floor(time*.6)%5;
 return <g transform={`translate(${x} ${y}) scale(${compact?.72:1})`}>
  {[3,2,1,0].map(i=><g key={i} transform={`translate(0 ${i*12})`}><path d="M0 36 110 0 262 47 152 87Z" fill={i===0?'#fbfdf7':'#e5eddc'} stroke="#aebe9f"/><path d="M0 36v7l152 51v-7Z" fill="#dce6d1" stroke="#aebe9f"/><path d="M152 87v7l110-40v-7Z" fill="#ceddbf" stroke="#aebe9f"/></g>)}
  <g transform="translate(13 37) matrix(1 .32 1 -.33 0 0)">
   <rect x="0" y="0" width="133" height="86" fill="#eef4e7" stroke="#adbe9e"/>
   <rect x="0" y="0" width="133" height="12" fill="#c9dabe"/>
   {[0,1,2,3,4].map(row=><g key={row}><rect x="8" y={19+row*12} width="117" height="8" fill={row===cell?'#88a66e':'#dce7d1'}/>{[0,1,2,3].map(col=><path key={col} d={`M${29+col*26} ${19+row*12}v8`} stroke="#f7faf1"/>)}</g>)}
  </g>
  {!compact&&<><path d="M0 76H-32v-39" fill="none" stroke={c.line}/><Text x={-35} y={24} size={10} muted>8 kB page</Text><path d="M224 72h49v-18" fill="none" stroke={c.line}/><Text x={276} y={46} size={10} muted>tuples</Text></>}
 </g>;
}
function Buffer({x,y,time=0}:{x:number;y:number;time?:number}){const active=Math.floor(time*.6)%8;return <g><Text x={x} y={y-12} size={11}>Shared buffers</Text>{Array.from({length:8},(_,i)=><g key={i}><rect x={x+(i%4)*29} y={y+Math.floor(i/4)*30} width="23" height="23" rx="3" fill={i===active?c.pale:c.paper} stroke={i===active?c.green:c.line}/><path d={`M${x+5+(i%4)*29} ${y+7+Math.floor(i/4)*30}h13m-13 4h13m-13 4h8`} stroke={i===active?c.dark:'#c8d5bb'} strokeWidth="1"/></g>)}</g>}
function Overview({time}:{time:number}){return <g>
 <Flow d="M92 82H237V114" time={time} active/><Flow d="M327 82H466V128" time={time} active/>
 <Node x={100} y={65} w={145} label="SELECT id, total" sub="WHERE status = 'paid'"/>
 <Node x={282} y={65} w={130} label="Query tree" sub="parse → rewrite"/>
 <Flow d="M282 98v26m0 0h-42v32m42-32h42v32" time={time} active/>
 <circle cx="240" cy="162" r="6" fill={c.pale} stroke={c.green}/><circle cx="324" cy="162" r="6" fill={c.pale} stroke={c.green}/>
 <Text x={240} y={186} anchor="middle" size={9} muted>target list</Text><Text x={324} y={186} anchor="middle" size={9} muted>predicate</Text>
 <Node x={473} y={160} w={131} label="Execution plan" sub="Sort ← Seq Scan" active/>
 <Flow d="M473 192v45h-49" time={time} active/><Buffer x={309} y={243} time={time}/>
 <Flow d="M310 270H255v25h-57" time={time} active/>
 <Heap x={45} y={246} time={time} compact/>
 <Text x={68} y={363} size={11}>orders</Text><Text x={127} y={363} size={10} muted>heap pages</Text>
 <Flow d="M424 272h48v43" time={time} active/>
 <Table x={429} y={314} title="Result rows" rows={[["id","total"],["…","…"]]} w={133}/>
 <Text x={35} y={204} size={10} muted>One statement.</Text><Text x={35} y={222} size={10} muted>Several representations.</Text>
 </g>}
function Sql({time}:{time:number}){const words=[['SELECT','id, total'],['FROM','orders'],['WHERE',"status = 'paid'"],['ORDER BY','id']];return <g>
 <rect x="64" y="43" width="468" height="208" rx="7" fill={c.paper} stroke={c.line}/>
 <path d="M64 76h468" stroke="#e5ecde"/><Text x={83} y={64} size={10} muted>query.sql</Text>
 {words.map(([key,value],i)=><g key={key}><Text x={84} y={111+i*37} size={11} muted>{i+1}</Text><text x="117" y={111+i*37} fontSize="16" fill={c.dark}>{key}<tspan dx="15" fill={c.ink}>{value}</tspan></text></g>)}
 <Flow d="M300 251v34" time={time} active/>
 {['keyword','identifier','operator','literal'].map((v,i)=><g key={v}><rect x={53+i*132} y="300" width="119" height="37" rx="4" fill={Math.floor(time*.7)%4===i?c.soft:c.paper} stroke={Math.floor(time*.7)%4===i?c.green:c.line}/><Text x={112+i*132} y={323} size={11} anchor="middle">{v}</Text></g>)}
 <Text x={300} y={375} anchor="middle" size={10} muted>Tokens preserve the meaning of the statement.</Text>
 </g>}
function Parser({time}:{time:number}){return <g>
 <Flow d="M300 80v42H96v28m204-28v28m0-28h202v28" time={time} active/>
 <Flow d="M96 195v54H64v30m32-30h51v30M300 195v84M502 195v84" time={time} active/>
 <Node x={300} y={61} w={151} label="SelectStmt" active/>
 <Node x={96} y={168} w={146} label="targetList" sub="result columns"/>
 <Node x={300} y={168} w={143} label="fromClause" sub="source relation"/>
 <Node x={502} y={168} w={143} label="whereClause" sub="qualification"/>
 <Node x={58} y={297} w={73} label="id"/><Node x={149} y={297} w={91} label="total"/>
 <Node x={300} y={297} w={145} label="RangeVar" sub="orders"/>
 <Node x={502} y={297} w={146} label="A_Expr" sub="status = 'paid'"/>
 <Text x={300} y={382} anchor="middle" size={10} muted>Selected branches of a PostgreSQL raw parse tree.</Text>
 </g>}
function Analyzer({time}:{time:number}){return <g>
 <Table x={33} y={38} title="pg_class" rows={[["relname","relkind"],["orders","r"]]} w={222} selected={1}/>
 <Table x={301} y={92} title="pg_attribute" rows={[["attname","atttypid"],["id","int4 OID"],["total","numeric OID"],["status","text OID"]]} w={266} selected={1+Math.floor(time*.5)%3}/>
 <Flow d="M255 105h22v71h24" time={time} active/>
 <Table x={47} y={235} title="pg_type" rows={[["typname","category"],["int4","numeric"],["numeric","numeric"],["text","string"]]} w={224} selected={1+Math.floor(time*.5)%3}/>
 <Flow d="M434 222v63H271" time={time} active/>
 <Text x={340} y={332} size={12}>Names become references.</Text><Text x={340} y={354} size={10} muted>Relations. Columns. Types.</Text>
 </g>}
function Rewriter({time}:{time:number}){return <g>
 <Node x={158} y={57} w={155} label="Analyzed Query"/>
 <Node x={441} y={57} w={157} label="Rewritten Query" active/>
 <Flow d="M236 57h127" time={time} active/>
 <Flow d="M158 76v34h-62v38m62-38h67v38M441 76v34h-62v38m62-38h67v38" time={time} active/>
 <Node x={96} y={167} w={120} label="orders"/><Node x={225} y={167} w={115} label="status" sub="= 'paid'"/>
 <Node x={379} y={167} w={120} label="orders"/><Node x={508} y={167} w={115} label="status" sub="= 'paid'"/>
 <rect x="83" y="263" width="435" height="75" rx="5" fill={c.soft} stroke="#d1dfc3"/>
 <Text x={300} y={290} anchor="middle" size={12}>This query needs no view expansion.</Text>
 <Text x={300} y={315} anchor="middle" size={10} muted>Rules and row security apply here when present.</Text>
 </g>}
function Planner({time}:{time:number}){return <g>
 <Text x={157} y={43} anchor="middle" size={11}>Scan, then sort</Text><Text x={443} y={43} anchor="middle" size={11}>Read in index order</Text>
 <Flow d="M157 184v-61M443 184v-61" time={time} active/>
 <Node x={157} y={91} w={160} label="Sort" sub="order by id" active/><Node x={157} y={209} w={185} label="Sequential scan" sub="filter: status = 'paid'"/>
 <Node x={443} y={91} w={158} label="Result rows" active/><Node x={443} y={209} w={184} label="Index scan" sub="orders_pkey + filter"/>
 <path d="M300 38v224" stroke="#d6e1cb" strokeDasharray="3 7"/>
 <Flow d="M157 242v43h143m143-43v43H300v24" time={time} active/>
 <rect x="103" y="309" width="394" height="59" rx="5" fill={c.soft} stroke="#c6d8b6"/>
 <Text x={300} y={333} anchor="middle" size={11}>Statistics + estimated cost → chosen path</Text>
 <Text x={300} y={354} anchor="middle" size={9} muted>No universal winner. Inspect your real plan.</Text>
 </g>}
function Executor({time}:{time:number}){return <g>
 <Heap x={103} y={237} time={time}/>
 <Text x={121} y={396} size={11}>orders</Text><Text x={191} y={396} size={10} muted>heap relation</Text>
 <Buffer x={71} y={99} time={time}/>
 <Flow d="M181 239V184H125v-28" time={time} active/>
 <Flow d="M188 128h74v-48h36" time={time} active/>
 <Node x={377} y={79} w={172} label="Visible tuples" sub="snapshot + predicate" active/>
 <Flow d="M377 113v52" time={time} active/>
 <Table x={293} y={171} title="Projection → sort" rows={[["id","total"],["…","…"]]} w={173} selected={1}/>
 <Text x={393} y={321} size={10} muted>Only matching rows</Text><Text x={393} y={340} size={10} muted>reach the client.</Text>
 </g>}
const scenes=[Sql,Parser,Analyzer,Rewriter,Planner,Executor];
export function DatabaseDiagram({stage,time,previous,blend=1}:{stage:number;time:number;previous:number;blend?:number}){
 const Current=scenes[stage]??Overview;const Previous=scenes[previous]??Overview;
 const showPrevious=previous!==stage&&blend<1;
 return <svg className="database-diagram" viewBox="0 0 600 425" role="img" aria-label={stage<0?'Database internals: query tree, execution plan, shared buffers and heap pages':`SQL journey: ${['SQL text','Parser','Analyzer','Rewriter','Planner','Executor'][stage]} is highlighted`}>
  <title>{stage<0?'The journey from a SQL statement to result rows':'Illustrated PostgreSQL internals'}</title>
  {showPrevious&&<g opacity={1-blend} aria-hidden="true"><Previous time={time}/></g>}
  <g opacity={showPrevious?blend:1} transform={`translate(0 ${(1-blend)*7})`}><Current time={time}/></g>
 </svg>;
}
