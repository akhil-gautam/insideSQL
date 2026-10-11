import {test,expect} from '@playwright/test';
test('real PostgreSQL operations are inspectable by graph, keyboard selector, tree and raw JSON',async({page})=>{
 await page.goto('/');
 await expect(page.getByRole('button',{name:/Chapter 7:/})).toBeDisabled();
 await expect(page.locator('.module-heading').filter({hasText:'Plans'})).toBeVisible();
 await page.getByRole('textbox',{name:'SQL query'}).fill('SELECT COUNT(*) FROM orders WHERE id = 99999');
 await page.getByRole('button',{name:'Analyze',exact:true}).click();
 await expect(page.getByText('Measured PostgreSQL execution')).toBeVisible();
 const details=page.getByRole('region',{name:'Selected plan operation'});
 const selector=page.getByRole('combobox',{name:'Inspect a plan operation'});
 await expect(selector).toBeVisible();
 const scanValue=await selector.locator('option').filter({hasText:'orders'}).first().getAttribute('value');
 await selector.focus();
 await selector.selectOption(scanValue!);
 await expect(details.getByText('Relation Name',{exact:true})).toBeVisible();
 await expect(details.getByText('orders',{exact:true}).first()).toBeVisible();
 await expect(details.locator('dl>div').filter({hasText:'Actual Rows'}).locator('dd')).toHaveText('0');
 await page.locator('.react-flow__node').first().click();
 await expect(selector).toHaveValue('0');
 await page.getByRole('button',{name:'tree',exact:true}).click();
 const tree=page.getByRole('list',{name:'Plan operations'});
 await expect(tree.getByText('Relation Name',{exact:true})).toBeVisible();
 await expect(tree.getByText('Plan Rows',{exact:true}).first()).toBeVisible();
 await page.getByRole('button',{name:'json',exact:true}).click();
 await expect(page.locator('.raw-plan')).toContainText('"Actual Rows": 0');
 await page.getByRole('button',{name:'Explain',exact:true}).click();
 await expect(page.getByText('PostgreSQL planner estimates')).toBeVisible();
 await expect(page.locator('.raw-plan')).not.toContainText('"Actual Rows"');
 await page.getByRole('button',{name:'graph',exact:true}).click();
 await expect(page.getByRole('combobox',{name:'Inspect a plan operation'})).toHaveValue('0');
 await page.evaluate(()=>window.scrollTo(0,0));
 await page.screenshot({path:'artifacts/plan-inspector.png',fullPage:true});
});

test('plan nodes stay visible through repeated equal-size plans and selection changes',async({page})=>{
 await page.goto('/');
 for(let i=0;i<8;i++){
  await page.getByRole('textbox',{name:'SQL query'}).fill(`SELECT COUNT(*) FROM orders WHERE id = ${99999+i}`);
  await page.getByRole('button',{name:'Analyze',exact:true}).click();
  await expect(page.getByText('Measured PostgreSQL execution')).toBeVisible();
  const picker=page.getByRole('combobox',{name:'Inspect a plan operation'});
  await picker.selectOption('1');
  await expect(page.locator('.react-flow__node[data-id="0"]')).toBeVisible();
  await picker.selectOption('0');
  await expect(page.locator('.react-flow__node[data-id="1"]')).toBeVisible();
  await page.getByRole('button',{name:'orders schema',exact:true}).click();
  await expect(page.locator('.react-flow__node[data-id="0"]')).toBeVisible();
 }
});

test('deletion keys preserve every measured plan node and edge',async({page})=>{
 await page.goto('/');
 await page.getByRole('textbox',{name:'SQL query'}).fill('SELECT COUNT(*) FROM orders WHERE id = 99999');
 await page.getByRole('button',{name:'Analyze',exact:true}).click();
 await expect(page.getByText('Measured PostgreSQL execution')).toBeVisible();
 const root=page.locator('.react-flow__node[data-id="0"]');
 await root.click();
 for(const key of ['Backspace','Delete']){
  await root.press(key);
  await expect(page.locator('.react-flow__node')).toHaveCount(2);
  await expect(page.locator('.react-flow__edge')).toHaveCount(1);
  await expect(root).toBeVisible();
  await expect(page.locator('.react-flow__node[data-id="1"]')).toBeVisible();
 }
});
