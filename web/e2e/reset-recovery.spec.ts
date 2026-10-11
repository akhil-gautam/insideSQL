import {test,expect} from '@playwright/test';

test('a lost response after actual Reset clears former measured plans even when progress inspection fails',async({page})=>{
 await page.goto('/');
 const editor=page.getByRole('textbox',{name:'SQL query'});
 const sql='SELECT COUNT(*) AS recovered FROM orders';
 await editor.fill(sql);
 for(const unavailable of [false,true]){
  await page.getByRole('button',{name:'Analyze',exact:true}).click();
  await expect(page.getByText('Measured PostgreSQL execution')).toBeVisible();
  await expect(page.getByRole('button',{name:'Analyze',exact:true})).toBeEnabled();
  let failProgress=false;
  await page.route('**/api/progress',async route=>{
   if(failProgress)await route.fulfill({status:503,contentType:'application/json',body:JSON.stringify({error:'Progress inspection unavailable'})});
   else await route.continue();
  });
  await page.route('**/api/reset',async route=>{
   const response=await route.fetch();
   expect(response.ok()).toBe(true);
   failProgress=unavailable;
   await route.fulfill({status:503,contentType:'application/json',body:JSON.stringify({error:'Reset response lost'})});
  });
  await page.getByRole('button',{name:'Reset lab'}).click();
  await expect(page.getByRole('alert')).toContainText('Reset response lost');
  await expect(page.getByRole('button',{name:'Run',exact:true})).toBeEnabled();
  await expect(page.getByText('Measured PostgreSQL execution')).toHaveCount(0);
  await expect(editor).toHaveText(sql);
  if(unavailable)await expect(page.getByRole('alert')).toContainText('Progress inspection unavailable');
  await page.unroute('**/api/reset');
  await page.unroute('**/api/progress');
  await page.getByRole('button',{name:'Run',exact:true}).click();
  await expect(page.locator('.result-table th')).toHaveText('recovered');
 }
});
