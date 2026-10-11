import {bundle} from '@remotion/bundler';
import {selectComposition,renderMedia,renderStill} from '@remotion/renderer';
import {mkdir,readFile,writeFile,rename} from 'node:fs/promises';
import {createHash} from 'node:crypto';
const serveUrl=await bundle({entryPoint:'web/src/video/entry.tsx',publicDir:'web/public'});
const browserExecutable='/Applications/Google Chrome.app/Contents/MacOS/Google Chrome';
const cinematic=process.argv.includes('--cinematic');
const composition=await selectComposition({serveUrl,id:cinematic?'EngineCinematic':'Chapter01',browserExecutable});
await mkdir('artifacts/media-verification',{recursive:true});
const prefix=cinematic?'cinematic-':'';
const frames=cinematic?[[`${prefix}overview`,60],...[150,240,330,420,510,600].map((frame,i)=>[`${prefix}stage-${i}`,frame]),[`${prefix}090-a`,90],[`${prefix}330`,330],[`${prefix}090-b`,90]]:[['090-a',90],['330',330],['090-b',90]];
for(const [name,frame]of frames){
 await renderStill({serveUrl,composition,frame,browserExecutable,output:`artifacts/media-verification/${name}.png`,imageFormat:'png'});
}
const digest=async name=>createHash('sha256').update(await readFile(`artifacts/media-verification/${name}.png`)).digest('hex');
if(await digest(`${prefix}090-a`)!==await digest(`${prefix}090-b`))throw new Error('Repeated frame was not deterministic');
await writeFile(`artifacts/media-verification/${prefix}result.json`,JSON.stringify({repeatedFrame:90,intermediateFrame:330,sha256:await digest(`${prefix}090-a`),passed:true},null,2));
console.log('Deterministic rendered frame check passed.');
if(process.argv.includes('--stills-only'))process.exit(0);
let last=0;
await renderMedia({serveUrl,composition,browserExecutable,codec:'h264',outputLocation:cinematic?'artifacts/engine-cinematic.mp4':'artifacts/chapter01-next.mp4',concurrency:4,crf:22,onProgress:({progress})=>{const percent=Math.floor(progress*100);if(percent>=last+5){last=percent;console.log(`Chapter render ${percent}%`);}}});
if(!cinematic)await rename('artifacts/chapter01-next.mp4','web/public/media/01/chapter.mp4');
console.log(cinematic?'Rendered 21-second engine preview.':'Rendered complete narrated 1080p chapter.');
