import {test,expect} from '@playwright/test';

test('a pending frame cannot regain a stale export when its filter changes',async({page})=>{
 await page.route('**/worker.js',async route=>{
  const response=await route.fetch();const original=await response.text();
  await route.fulfill({response,body:original+`\nconst originalHandler=self.onmessage;self.onmessage=async event=>{if(event.data.step===4)return new Promise(()=>{});return originalHandler(event);};\n`});
 });
 await page.goto('/WITNESS/');await expect(page.locator('#export')).toBeEnabled();
 await page.locator('#step').evaluate(e=>{e.value='4';e.dispatchEvent(new Event('input'));});
 await expect(page.locator('#export')).toBeDisabled();
 await page.locator('#filter').selectOption('sources');
 await expect(page.locator('#export')).toBeDisabled();
 await expect(page.locator('#events')).toHaveText('—');
 await expect(page.locator('#timeline li')).toHaveCount(0);
});
