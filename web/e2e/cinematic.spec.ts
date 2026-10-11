import {test,expect} from '@playwright/test';
import {mkdir} from 'node:fs/promises';
import {readFileSync} from 'node:fs';
const output='.impeccable/review';
test('inspect all six cinematic layers, pause, responsive framing and graphics fallback',async({page,browser})=>{
 await mkdir(output,{recursive:true});const errors:string[]=[];page.on('pageerror',e=>errors.push(e.message));
 await page.goto('/');await page.getByRole('tab',{name:'Explore the journey'}).click();
 await expect(page.getByText('Assembling the engine…')).toHaveCount(0);
 await expect(page.getByText('Inside the database engine.')).toBeVisible();
 await page.getByRole('button',{name:'Pause motion',exact:true}).click();
 await page.locator('.journey-scene').first().screenshot({path:`${output}/overview-desktop.png`});
 for(const stage of ['SQL text','Parser','Analyzer','Rewriter','Planner','Executor']){
  await page.getByRole('button',{name:stage,exact:true}).click();
  await expect(page.getByRole('img',{name:`SQL journey: ${stage} is highlighted`})).toBeVisible();
  // Resume just long enough for the authored camera move, then hold the inspection.
  await page.getByRole('button',{name:'Resume motion',exact:true}).click();await page.waitForTimeout(1800);await page.getByRole('button',{name:'Pause motion',exact:true}).click();
  const held=await page.locator('canvas').first().screenshot();await page.waitForTimeout(150);
  expect((await page.locator('canvas').first().screenshot()).equals(held)).toBe(true);
  await page.locator('.journey-scene').first().screenshot({path:`${output}/${stage.replace(' ','-').toLowerCase()}-desktop.png`});
 }
 await page.screenshot({path:`${output}/workbench-desktop.png`,fullPage:true});expect(errors).toEqual([]);
 const mobile=await browser.newContext({viewport:{width:390,height:844},reducedMotion:'reduce'});const small=await mobile.newPage();await small.goto('/');await small.getByRole('tab',{name:'Explore the journey'}).click();
 await expect(small.getByText('Assembling the engine…')).toHaveCount(0);await expect(small.getByRole('button',{name:'Reduced motion'})).toBeDisabled();
 await small.locator('.journey-scene').first().screenshot({path:`${output}/overview-mobile.png`});await small.getByRole('button',{name:'Parser',exact:true}).click();
 await small.locator('.journey-scene').first().screenshot({path:`${output}/parser-mobile.png`});await small.screenshot({path:`${output}/workbench-mobile.png`,fullPage:true});
 const still=await small.locator('canvas').first().screenshot();await small.waitForTimeout(150);
 expect((await small.locator('canvas').first().screenshot()).equals(still)).toBe(true);
 expect(await small.evaluate(()=>document.documentElement.scrollWidth<=window.innerWidth)).toBe(true);await mobile.close();
 await page.locator('canvas').first().evaluate(canvas=>canvas.dispatchEvent(new Event('webglcontextlost',{cancelable:true})));
 await expect(page.getByText('The 3D view lost its graphics context.',{exact:false})).toBeVisible();await page.getByRole('button',{name:'Analyzer',exact:true}).click();await expect(page.getByText('Names acquire meaning.')).toBeVisible();
});

test('silent replay respects native media policy, advances the player clock and pauses on a stable frame',async({page})=>{
 const csp=JSON.parse(readFileSync('src-tauri/tauri.conf.json','utf8')).app.security.csp as string;
 const mediaPolicy=csp.split(';').map(d=>d.trim()).find(d=>d.startsWith('media-src '))!;
 const violations:string[]=[];
 await page.exposeFunction('recordMediaViolation',(directive:string)=>violations.push(directive));
 await page.addInitScript(()=>document.addEventListener('securitypolicyviolation',event=>{
  if(event.effectiveDirective==='media-src')void (window as unknown as {recordMediaViolation:(d:string)=>Promise<void>}).recordMediaViolation(event.effectiveDirective);
 }));
 await page.route('http://127.0.0.1:1420/',async route=>{
  const response=await route.fetch();await route.fulfill({response,headers:{...response.headers(),'content-security-policy':mediaPolicy}});
 });
 await page.goto('/');await page.getByRole('tab',{name:'Explore the journey'}).click();
 const replay=page.locator('details.replay');await replay.locator('summary').click();
 await expect(replay.locator('.engine-loading')).toHaveCount(0);
 await expect(replay.getByRole('button',{name:'Pause motion',exact:true})).toHaveCount(0);
 expect(violations).toEqual([]);
 await expect(replay.locator('audio')).toHaveCount(0);
 await replay.getByRole('button',{name:'Play video',exact:true}).click();
 await expect(replay.getByRole('button',{name:'Pause video',exact:true})).toBeVisible();
 await expect(replay.getByRole('img',{name:'SQL journey: Parser is highlighted'})).toBeVisible();
 expect(violations).toEqual([]);
 await replay.getByRole('button',{name:'Pause video',exact:true}).click();
 await expect(replay.getByRole('button',{name:'Play video',exact:true})).toBeVisible();
 const canvas=replay.locator('canvas');const first=await canvas.screenshot();await page.waitForTimeout(200);
 expect((await canvas.screenshot()).equals(first)).toBe(true);
});
