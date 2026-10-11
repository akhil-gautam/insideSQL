import * as T from 'three';
import type {enginePose} from './engineMotion';
type Pose=ReturnType<typeof enginePose>;
type Label={text:string;position:T.Vector3;layer:number};
const graphite='#4c4946',edge='#161312',copper='#ffc59e',steel='#8b8179';

export function buildDatabaseEngine(){
 const scene=new T.Scene();scene.background=new T.Color('#201e1d');
 const assembly=new T.Group();scene.add(assembly);
 const layers:T.Group[]=[];const accents:T.MeshStandardMaterial[]=[];const rotors:T.Object3D[]=[];const signals:T.Mesh[]=[];const labels:Label[]=[];
 const ink=new T.LineBasicMaterial({color:edge,transparent:true,opacity:.75});
 const body=new T.MeshStandardMaterial({color:graphite,roughness:.64,metalness:.25});
 const dark=new T.MeshStandardMaterial({color:'#292624',roughness:.76,metalness:.15});
 const metal=new T.MeshStandardMaterial({color:steel,roughness:.45,metalness:.35});
 function solid(parent:T.Object3D,geometry:T.BufferGeometry,position:number[],material:T.Material=body,outlined=false){
  const mesh=new T.Mesh(geometry,material);mesh.position.set(position[0],position[1],position[2]);parent.add(mesh);
  if(outlined)mesh.add(new T.LineSegments(new T.EdgesGeometry(geometry,28),ink));
  return mesh;
 }
 function block(parent:T.Object3D,size:number[],position:number[],material:T.Material=body){return solid(parent,new T.BoxGeometry(...size as [number,number,number]),position,material,true);}
 function ring(parent:T.Object3D,radius:number,tube:number,x:number,material:T.Material=body){const m=solid(parent,new T.TorusGeometry(radius,tube,5,64),[x,0,0],material,true);m.rotation.y=Math.PI/2;return m;}
 function wire(parent:T.Object3D,points:number[][],material:T.Material=metal,radius=.018){
  const path=new T.CatmullRomCurve3(points.map(p=>new T.Vector3(...p as [number,number,number])));
  return solid(parent,new T.TubeGeometry(path,24,radius,5,false),[0,0,0],material);
 }
 function tag(text:string,layer:number,position:number[]){labels.push({text,layer,position:new T.Vector3(...position as [number,number,number])});}
 function repeatAround(parent:T.Object3D,count:number,radius:number,x:number,geometry:T.BufferGeometry,material:T.Material){
  const instances=new T.InstancedMesh(geometry,material,count);const temp=new T.Object3D();
  for(let n=0;n<count;n++){const a=n/count*Math.PI*2;temp.position.set(x,Math.cos(a)*radius,Math.sin(a)*radius);temp.rotation.set(a,0,0);temp.updateMatrix();instances.setMatrixAt(n,temp.matrix);}
  parent.add(instances);return instances;
 }
 for(let i=0;i<6;i++){
  const layer=new T.Group();layer.position.x=(i-2.5)*2.35;layer.name=['SQL token assembly','Parser syntax tree','Analyzer catalog banks','Rewriter expansion gates','Planner path selector','Executor heap and buffers'][i];layers.push(layer);assembly.add(layer);
  const accent=new T.MeshStandardMaterial({color:copper,roughness:.4,metalness:.3,emissive:copper,emissiveIntensity:.16});accents.push(accent);
  const radius=[1.2,1.37,1.55,1.3,1.65,1.5][i];
  ring(layer,radius,.11,-.43);ring(layer,radius,.11,.43);ring(layer,radius+.035,.024,-.53,accent);
  repeatAround(layer,[24,8,16,12,8,20][i],radius,0,new T.BoxGeometry(.72,.11,.075),dark);
  repeatAround(layer,16,radius+.02,.48,new T.CylinderGeometry(.04,.04,.09,6),metal);
  const rotor=new T.Group();layer.add(rotor);ring(rotor,radius-.14,.035,.38,metal);rotors.push(rotor);
  for(let n=0;n<4;n++){const a=n*Math.PI/2+.3;wire(layer,[[-.42,Math.cos(a)*radius,Math.sin(a)*radius],[0,Math.cos(a)*(radius+.12),Math.sin(a)*(radius+.12)],[.43,Math.cos(a)*radius,Math.sin(a)*radius]],metal,.027);}
 }
 // Each chamber carries different structures: the casing is only the visual metaphor.
 const sql=layers[0];
 for(let row=0;row<6;row++){
  block(sql,[.68,.11,1.6],[0,.85-row*.31,0],row===0?accents[0]:metal);
  for(let cell=0;cell<3+(row%3);cell++)block(sql,[.16,.09,.18],[.41,.85-row*.31,-.62+cell*.29],row%2===0?dark:body);
 }
 tag('SELECT  id, total',0,[.6,1.05,0]);tag('WHERE  status = \'paid\'',0,[.6,.07,0]);tag('ORDER BY  id',0,[.6,-.82,0]);
 const parser=layers[1];
 const tree=[[0,1.03,0],[0,.35,-.65],[0,.35,.65],[0,-.45,-.95],[0,-.45,-.35],[0,-.45,.35],[0,-.45,.95]];
 for(let n=0;n<tree.length;n++){
  solid(parser,new T.OctahedronGeometry(n===0?.21:.14),tree[n],n===0?accents[1]:metal);
  const mount=new T.Group();mount.position.set(...tree[n] as [number,number,number]);parser.add(mount);
  ring(mount,n===0?.28:.21,.045,-.12,metal);block(mount,[.28,.1,.1],[.12,0,0],dark);
  for(let screw=0;screw<3;screw++){const a=screw*Math.PI*2/3;block(mount,[.09,.035,.035],[-.17,Math.cos(a)*.2,Math.sin(a)*.2],body);}
  if(n>0){const parent=n<3?0:n<5?1:2;wire(parser,[tree[parent],[.12,(tree[parent][1]+tree[n][1])/2,tree[n][2]],tree[n]],accents[1],.024);}
 }
 repeatAround(parser,3,1.4,0,new T.BoxGeometry(.9,.22,.3),dark);
 tag('SelectStmt',1,[.35,1.34,0]);tag('targetList',1,[.35,.35,-.95]);tag('whereClause',1,[.35,.35,.95]);tag('ColumnRef / A_Expr',1,[.35,-.87,0]);
 const analyzer=layers[2];
 for(let bank=0;bank<3;bank++){
  block(analyzer,[.11,1.85,.64],[-.23+bank*.26,0,-.85+bank*.85],dark);
  for(let row=0;row<7;row++)for(let col=0;col<2;col++)block(analyzer,[.15,.12,.2],[-.39+bank*.26,.77-row*.25,-1.02+bank*.85+col*.32],(row+col)%4===0?accents[2]:metal);
  wire(analyzer,[[.4,.85,-.85+bank*.85],[.65,1.24,-.85+bank*.85],[.65,1.24,0]],accents[2]);
 }
 tag('pg_class',2,[.8,1.12,-1]);tag('pg_attribute',2,[.8,-1.05,0]);tag('pg_type',2,[.8,1.12,1]);
 const rewriter=layers[3];
 for(let gate=0;gate<3;gate++){
  const frame=new T.Group();frame.position.x=(gate-1)*.38;rewriter.add(frame);
  ring(frame,.84,.08,0,gate===1?accents[3]:metal);
  for(let n=0;n<8;n++){const a=n*Math.PI/4;const vane=block(frame,[.15,.23,.13],[0,Math.cos(a)*.78,Math.sin(a)*.78],dark);vane.rotation.x=a;}
 }
 for(let track=0;track<4;track++){
  const a=track*Math.PI/2;wire(rewriter,[[-.66,Math.cos(a)*.65,Math.sin(a)*.65],[0,Math.cos(a+.65)*.65,Math.sin(a+.65)*.65],[.66,Math.cos(a+1.3)*.65,Math.sin(a+1.3)*.65]],accents[3],.025);
 }
 tag('Query',3,[-.66,1.13,0]);tag('rules / views / RLS',3,[.48,-1.02,0]);
 const planner=layers[4];
 for(let branch=0;branch<3;branch++){
  const a=branch/3*Math.PI*2;const y=Math.cos(a)*1.08,z=Math.sin(a)*1.08;
  const route=new T.Group();planner.add(route);
  wire(route,[[-.66,0,0],[-.08,y*.65,z*.65],[.55,y,z]],branch===0?accents[4]:metal,.04);
  block(route,[.42,.25,.32],[.55,y,z],branch===0?accents[4]:body);
  for(let step=0;step<2;step++){
   const t=.34+step*.3;const pod=block(route,[.38,.3,.36],[-.66+t*1.2,y*t,z*t],dark);
   block(pod,[.045,.19,.25],[-.21,0,0],branch===0?accents[4]:metal);
   for(let pin=0;pin<3;pin++)block(pod,[.09,.025,.07],[-.25,-.07+pin*.07,0],body);
  }
  for(let n=0;n<4;n++)block(route,[.08,.035,.22],[.45+n*.07,y+.19,z],dark);
 }
 const selector=solid(planner,new T.CylinderGeometry(.36,.36,.3,8),[-.6,0,0],dark,true);selector.rotation.z=Math.PI/2;
 ring(planner,.38,.045,-.78,metal);
 repeatAround(planner,3,1.67,0,new T.BoxGeometry(.95,.25,.4),dark);
 solid(planner,new T.IcosahedronGeometry(.3,0),[-.6,0,0],accents[4]);
 tag('Seq Scan → Sort',4,[.8,1.43,0]);tag('Index Scan + filter',4,[.8,-.85,1.03]);tag('cost / rows / ordering',4,[.8,-.85,-1.03]);
 const executor=layers[5];
 for(let page=0;page<5;page++){
  const board=block(executor,[.13,1.65,1.65],[-.48+page*.22,0,0],dark);
  for(let row=0;row<4;row++)for(let slot=0;slot<4;slot++)block(board,[.08,.22,.24],[-.12,.57-row*.36,-.56+slot*.36],(row+slot+page)%5===0?accents[5]:body);
 }
 for(let n=0;n<4;n++){const a=n*Math.PI/2;wire(executor,[[.6,Math.cos(a)*.85,Math.sin(a)*.85],[1,Math.cos(a)*.65,Math.sin(a)*.65],[1.35,Math.cos(a)*.35,Math.sin(a)*.35]],accents[5]);}
 tag('buffer page',5,[.85,1.22,0]);tag('visible tuples → filter',5,[.85,-1.2,0]);
 const context=new T.Group();assembly.add(context);
 for(let rail=0;rail<3;rail++){const a=rail/3*Math.PI*2;wire(context,[[-7.3,Math.cos(a)*.47,Math.sin(a)*.47],[0,Math.cos(a)*.47,Math.sin(a)*.47],[7.6,Math.cos(a)*.47,Math.sin(a)*.47]],dark,.035);}
 for(let i=0;i<12;i++){const packet=solid(context,new T.BoxGeometry(.1,.075,.075),[0,0,0],new T.MeshBasicMaterial({color:i%3===0?'#fff1d2':copper}));signals.push(packet);}
 const cap=solid(context,new T.CylinderGeometry(1.17,1.17,.17,64),[-7.22,0,0],dark,true);cap.rotation.z=Math.PI/2;
 ring(context,1.22,.07,-7.33,metal);ring(context,1.45,.1,7.37);ring(context,1.49,.028,7.5,accents[5]);
 scene.add(new T.HemisphereLight('#f3efea','#302c2a',1.6));
 const key=new T.DirectionalLight('#fff3e9',2.6);key.position.set(-4,9,8);scene.add(key);
 const fill=new T.DirectionalLight('#b4c2d0',1.2);fill.position.set(6,2,-8);scene.add(fill);
 const rim=new T.DirectionalLight('#ffc59e',3);rim.position.set(-8,2,-3);scene.add(rim);
 const layerMaterials=layers.map((layer,i)=>{
  const copies=new Map<T.Material,T.MeshStandardMaterial>();
  layer.traverse(object=>{const mesh=object as T.Mesh;if(mesh.material instanceof T.MeshStandardMaterial&&mesh.material!==accents[i]){
   let material=copies.get(mesh.material);if(!material){material=mesh.material.clone();copies.set(mesh.material,material);}mesh.material=material;
  }});
  return [...copies.values()].map(material=>({material,color:material.color.clone()}));
 });
 return {scene,assembly,layers,accents,rotors,signals,labels,layerMaterials,context};
}

export function createDatabaseRenderer(canvas:HTMLCanvasElement,labelRoot:HTMLDivElement,synchronize=false){
 const world=buildDatabaseEngine();
 const renderer=new T.WebGLRenderer({canvas,antialias:true,alpha:false,preserveDrawingBuffer:true,powerPreference:'high-performance'});
 renderer.outputColorSpace=T.SRGBColorSpace;renderer.toneMapping=T.ACESFilmicToneMapping;renderer.toneMappingExposure=1.05;
 const camera=new T.OrthographicCamera(-8,8,5,-5,.1,100);
 const annotations=world.labels.map(label=>{const element=document.createElement('span');element.className='engine-annotation';element.textContent=label.text;labelRoot.append(element);return {element,label};});
 let width=0,height=0;
 function draw(pose:Pose){
  const rect=canvas.getBoundingClientRect();const w=Math.max(1,Math.round(rect.width)),h=Math.max(1,Math.round(rect.height));
  if(w!==width||h!==height){width=w;height=h;renderer.setPixelRatio(Math.min(window.devicePixelRatio||1,1.5));renderer.setSize(w,h,false);}
  const aspect=w/h;const focus=T.MathUtils.lerp(pose.previous<0?0:1,pose.stage<0?0:1,pose.blend);const spacing=T.MathUtils.lerp(2.35,3.8,focus);
  function view(stage:number){
   if(stage<0)return {target:new T.Vector3(.3,0,0),extent:aspect<1.4?7.5:Math.max(3.25,7.5/aspect)};
   return {target:new T.Vector3((stage-2.5)*3.8+.2,0,0),extent:Math.max(1.95,2.25/aspect)};
  }
  const prior=view(pose.previous),next=view(pose.stage);const target=prior.target.lerp(next.target,pose.blend);
  const extent=T.MathUtils.lerp(prior.extent,next.extent,pose.blend);
  camera.left=-extent*aspect;camera.right=extent*aspect;camera.top=extent;camera.bottom=-extent;camera.updateProjectionMatrix();
  camera.position.copy(target).add(new T.Vector3(-11+pose.orbit*12,6,17));camera.lookAt(target);
  world.assembly.rotation.x=pose.orbit;world.assembly.rotation.z=Math.max(0,Math.min(1,(1.4-aspect)/.4))*.78*(1-focus);world.context.scale.x=spacing/2.35;
  for(let i=0;i<6;i++){
   world.layers[i].position.x=(i-2.5)*spacing*pose.spread;
   world.layers[i].position.y=i===pose.stage?.14:0;
   world.accents[i].emissiveIntensity=i===pose.stage?.32:.04;
   for(const {material,color}of world.layerMaterials[i])material.color.copy(color).multiplyScalar(i===pose.stage?1:1-.7*focus);
   world.rotors[i].rotation.x=pose.seconds*(i%2===0?.12:-.08);
  }
  for(let i=0;i<world.signals.length;i++){
   const packet=world.signals[i];const phase=(pose.seconds*.14+i/world.signals.length)%1;
   packet.position.set(-7.2+phase*14.7,Math.cos(i%3/3*Math.PI*2)*.47,Math.sin(i%3/3*Math.PI*2)*.47);
  }
  renderer.render(world.scene,camera);
  for(const {element,label}of annotations){
   element.style.display=label.layer===pose.stage?'block':'none';if(label.layer!==pose.stage)continue;
   const point=world.layers[label.layer].localToWorld(label.position.clone()).project(camera);
   const x=T.MathUtils.clamp((point.x+1)*w/2,element.offsetWidth/2+8,w-element.offsetWidth/2-8);
   const y=T.MathUtils.clamp((1-point.y)*h/2,element.offsetHeight/2+8,h-element.offsetHeight/2-8);
   element.style.left=`${x}px`;element.style.top=`${y}px`;
  }
  if(synchronize)renderer.getContext().finish();
 }
 function dispose(){
  const geometries=new Set<T.BufferGeometry>(),materials=new Set<T.Material>();
  world.scene.traverse(object=>{const renderable=object as T.Mesh;if(renderable.geometry)geometries.add(renderable.geometry);if(renderable.material)(Array.isArray(renderable.material)?renderable.material:[renderable.material]).forEach(m=>materials.add(m));});
  annotations.forEach(({element})=>element.remove());geometries.forEach(g=>g.dispose());materials.forEach(m=>{if('map'in m)(m as T.SpriteMaterial).map?.dispose();m.dispose();});renderer.dispose();
 }
 return {draw,dispose,world};
}
