import {test,expect} from '@playwright/test';
import {mkdir} from 'node:fs/promises';
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
  await page.locator('.journey-scene').first().screenshot({path:`${output}/${stage.replace(' ','-').toLowerCase()}-desktop.png`});
 }
 await page.screenshot({path:`${output}/workbench-desktop.png`,fullPage:true});expect(errors).toEqual([]);
 const mobile=await browser.newContext({viewport:{width:390,height:844},reducedMotion:'reduce'});const small=await mobile.newPage();await small.goto('/');await small.getByRole('tab',{name:'Explore the journey'}).click();
 await expect(small.getByText('Assembling the engine…')).toHaveCount(0);await expect(small.getByRole('button',{name:'Reduced motion'})).toBeDisabled();
 await small.locator('.journey-scene').first().screenshot({path:`${output}/overview-mobile.png`});await small.getByRole('button',{name:'Parser',exact:true}).click();
 await small.locator('.journey-scene').first().screenshot({path:`${output}/parser-mobile.png`});await small.screenshot({path:`${output}/workbench-mobile.png`,fullPage:true});
 expect(await small.evaluate(()=>document.documentElement.scrollWidth<=window.innerWidth)).toBe(true);await mobile.close();
 await page.locator('canvas').first().evaluate(canvas=>canvas.dispatchEvent(new Event('webglcontextlost',{cancelable:true})));
 await expect(page.getByText('The 3D view lost its graphics context.',{exact:false})).toBeVisible();await page.getByRole('button',{name:'Analyzer',exact:true}).click();await expect(page.getByText('Names acquire meaning.')).toBeVisible();
});

test('shared replay uses the player clock and pauses on a stable frame',async({page})=>{
 await page.goto('/');await page.getByRole('tab',{name:'Explore the journey'}).click();
 const replay=page.locator('details.replay');await replay.locator('summary').click();
 await expect(replay.locator('.engine-loading')).toHaveCount(0);
 await expect(replay.getByRole('button',{name:'Pause motion',exact:true})).toHaveCount(0);
 await replay.getByRole('button',{name:'Play video',exact:true}).click();
 await expect(replay.getByRole('button',{name:'Pause video',exact:true})).toBeVisible();
 await page.waitForTimeout(1000);await replay.getByRole('button',{name:'Pause video',exact:true}).click();
 await expect(replay.getByRole('button',{name:'Play video',exact:true})).toBeVisible();
 const canvas=replay.locator('canvas');const first=await canvas.screenshot();await page.waitForTimeout(200);
 expect((await canvas.screenshot()).equals(first)).toBe(true);
});
