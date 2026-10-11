import {test,expect} from '@playwright/test';
test('an unsubmitted draft survives switching questions and reopening',async({page})=>{
 await page.goto('/');
 const query=page.getByRole('textbox',{name:'SQL query'});
 await expect(query).toBeVisible();
 await query.fill('SELECT id FROM orders -- unsent draft');
 await expect.poll(async()=>{
  const response=await page.request.post('/api/progress',{data:{}});
  return (await response.json())['01-01'].sql;
 }).toBe('SELECT id FROM orders -- unsent draft');
 await page.getByRole('button',{name:/Exercise 2:/}).click();
 await page.getByRole('button',{name:/Exercise 1:/}).click();
 await expect(query).toHaveText('SELECT id FROM orders -- unsent draft');
 await page.reload();await expect(query).toHaveText('SELECT id FROM orders -- unsent draft');
});
test('cancel keeps the editor fixed during a run and usable afterwards',async({page})=>{
 await page.goto('/');const query=page.getByRole('textbox',{name:'SQL query'});
 await query.fill('SELECT pg_sleep(20)');await page.getByRole('button',{name:'Run',exact:true}).click();
 await expect(query).toHaveAttribute('contenteditable','false');
 await page.getByRole('button',{name:'Cancel',exact:true}).click();
 await expect(page.getByText('57014')).toBeVisible();
 await expect(page.getByText('Query cancelled',{exact:false})).toBeVisible();
 await expect(query).toHaveAttribute('contenteditable','true');
});
test('the authored lesson has playable local video and timed captions',async({page})=>{
 await page.goto('/');
 const video=page.locator('video');
 await expect(video).toBeVisible();
 await expect.poll(()=>video.evaluate((v:HTMLVideoElement)=>v.duration)).toBeGreaterThan(417);
 expect(await video.evaluate((v:HTMLVideoElement)=>[v.videoWidth,v.videoHeight])).toEqual([1920,1080]);
 await video.evaluate(async(v:HTMLVideoElement)=>{v.muted=true;await v.play();});
 await expect.poll(()=>video.evaluate((v:HTMLVideoElement)=>v.currentTime)).toBeGreaterThan(0.1);
 await expect.poll(()=>video.evaluate((v:HTMLVideoElement)=>v.textTracks[0]?.cues?.length||0)).toBe(110);
 await video.evaluate((v:HTMLVideoElement)=>{v.pause();v.currentTime=90;});
 await expect.poll(()=>video.evaluate((v:HTMLVideoElement)=>v.textTracks[0]?.activeCues?.length||0)).toBe(1);
});
const solutions=[
 "SELECT id, total FROM orders WHERE status = 'paid' ORDER BY id",
 "SELECT id, customer_id FROM orders WHERE status = 'pending' AND total IS NULL ORDER BY id",
 "SELECT id, customer_id, total FROM orders WHERE status = 'paid' AND total > 100 ORDER BY total DESC, id",
 "SELECT id, total FROM orders WHERE status = 'paid' AND total IS NOT NULL ORDER BY total DESC, id LIMIT 3",
 "SELECT id, ROUND(total * 0.9, 2) AS discounted_total FROM orders WHERE status = 'paid' ORDER BY id",
 "SELECT COUNT(*) AS paid_count, COALESCE(SUM(total), 0) AS paid_total FROM orders WHERE status = 'paid'"
];
test('six real SQL exercises pass, plans distinguish estimates and measurements, progress resumes',async({page})=>{
 await page.goto('/');await expect(page.getByRole('heading',{name:'A SELECT statement, end to end'})).toBeVisible();
 for(let i=0;i<solutions.length;i++){
  await page.getByRole('button',{name:new RegExp(`Exercise ${i+1}:`)}).click();
  await page.getByRole('textbox',{name:'SQL query'}).fill(solutions[i]);
  await page.getByRole('button',{name:'Check my query'}).click();
  await expect(page.getByText('Correct across all three fixture variants.')).toBeVisible();
 }
 await expect(page.getByText('Chapter 1 complete. Chapter 2 is being authored.')).toBeVisible();
 await page.reload();await expect(page.getByText('Chapter 1 complete. Chapter 2 is being authored.')).toBeVisible();
 await page.getByRole('button',{name:'Explain',exact:true}).click();await expect(page.getByText('PostgreSQL planner estimates')).toBeVisible();
 await page.getByRole('button',{name:'Analyze',exact:true}).click();await expect(page.getByText('Measured PostgreSQL execution')).toBeVisible();
 await page.getByRole('button',{name:'tree',exact:true}).click();await expect(page.getByText('Sort',{exact:true})).toBeVisible();
 await page.getByRole('tab',{name:'Explore the journey'}).click();await page.getByRole('button',{name:'Planner',exact:true}).click();
 await expect(page.getByRole('img',{name:'SQL journey: Planner is highlighted'})).toBeVisible();
 await page.screenshot({path:'artifacts/e2e-workbench.png',fullPage:true});
});
