// Run with Node 22+ and a Chrome instance using --remote-debugging-port=19222.
import fs from 'node:fs';
const base = process.argv[2] || 'http://127.0.0.1:18082';
const debug = process.argv[3] || 'http://127.0.0.1:19223';
const targets = await (await fetch(`${debug}/json`)).json();
const ws = new WebSocket(targets.find(t => t.type === 'page').webSocketDebuggerUrl);
await new Promise(resolve => { ws.onopen = resolve; });
let sequence = 0;
const pending = new Map();
const errors = [];
let heldFontRequests = 0;
ws.onmessage = ({data}) => {
 const message = JSON.parse(data);
 // Hold the old CLI template's external font request to reproduce offline startup.
 if (message.method === 'Fetch.requestPaused') heldFontRequests++;
 if (message.method === 'Runtime.exceptionThrown') errors.push(message.params.exceptionDetails.text);
 if (message.id) { pending.get(message.id)?.(message); pending.delete(message.id); }
};
async function call(method, params = {}) {
 const id = ++sequence;
 const result = new Promise(resolve => pending.set(id, resolve));
 ws.send(JSON.stringify({id, method, params}));
 const message = await result;
 if (message.error) throw Error(JSON.stringify(message.error));
 return message.result;
}
async function evaluate(expression) {
 const result = await call('Runtime.evaluate', {expression, returnByValue: true, awaitPromise: true});
 if (result.exceptionDetails) throw Error(JSON.stringify(result.exceptionDetails));
 return result.result.value;
}
const delay = ms => new Promise(resolve => setTimeout(resolve, ms));
async function ready(selector, attempts = 120) {
 for (let i = 0; i < attempts; i++) {
  await delay(500);
  if (await evaluate(`!!document.querySelector(${JSON.stringify(selector)})`)) return;
 }
 throw Error(`Timed out waiting for ${selector}`);
}
const checks = [];
const upstreamIssues = [];
async function check(name, expression) {
 if (!await evaluate(expression)) throw Error(`Failed: ${name}`);
 checks.push(name); console.log(`PASS ${name}`);
}
async function navigate(path, selector) {
 errors.length = 0;
 await call('Page.navigate', {url: base + path});
 await ready(selector);
 await ready('nav a[data-dioxus-id]');
 await delay(500);
}
await call('Page.enable'); await call('Runtime.enable');
await call('Emulation.setFocusEmulationEnabled',{enabled:true});
await call('Emulation.setDeviceMetricsOverride',{width:1280,height:1000,deviceScaleFactor:1,mobile:false});
await navigate('/components/table','.cn-data-panel table');
await check('table has connection identity and status hierarchy', `document.querySelectorAll('.cn-data-panel tbody tr').length===6 && document.querySelectorAll('.cn-data-identity strong').length===6`);
await check('table has a single integrated frame', `!document.querySelector('.cn-data-panel').parentElement.classList.contains('border') && getComputedStyle(document.querySelector('.cn-data-panel')).borderRadius==='14px'`);
await navigate('/components/data-table','.cn-data-panel table');
await evaluate(`document.querySelector('.cn-data-filter').click()`);await delay(150);
await check('healthy filter narrows rows', `document.querySelectorAll('.cn-data-panel tbody tr').length===3 && document.querySelector('.cn-data-filter').getAttribute('aria-pressed')==='true'`);
await evaluate(`document.querySelector('button[aria-label="Select all visible connections"]').click()`);await delay(150);
await check('select all applies to visible rows', `document.querySelectorAll('.cn-data-panel tr[data-state="selected"]').length===3 && document.querySelector('.cn-data-footer').textContent.includes('3 selected')`);
await evaluate(`document.querySelector('.cn-data-sort').click()`);await delay(150);
await check('connection sort changes order', `document.querySelector('.cn-data-panel tbody .cn-data-identity strong').textContent==='Orders to ERP'`);
await evaluate(`var input=document.querySelector('#connection-search');input.value='no matching connection';input.dispatchEvent(new Event('input',{bubbles:true}))`);await delay(150);
await check('search has useful empty state', `document.querySelectorAll('.cn-data-panel tbody tr').length===0 && document.querySelector('.cn-data-empty').textContent.includes('No connections found')`);
await evaluate(`var input=document.querySelector('#connection-search');input.value='';input.dispatchEvent(new Event('input',{bubbles:true}));document.querySelector('.cn-data-filter').click()`);await delay(150);
await check('clearing search and filter restores rows', `document.querySelectorAll('.cn-data-panel tbody tr').length===6`);
await call('Page.captureScreenshot',{format:'png'}).then(shot=>fs.writeFileSync('/tmp/conctr-premium-table.png',Buffer.from(shot.data,'base64')));
await navigate('/components/data-grid','[data-name="DataGrid"]');
await check('mapping grid has semantic rows and readable spacing', `document.querySelectorAll('.cn-mapping-row').length===6 && document.querySelector('[role="gridcell"]').getBoundingClientRect().height===64`);
await evaluate(`document.querySelector('button[aria-label="Edit target path for Customer ID"]').click()`);await delay(100);
await check('target field enters focused edit mode', `document.activeElement.getAttribute('aria-label')==='Target path for Customer ID'`);
await evaluate(`var input=document.activeElement;input.value='account.customer_key';input.dispatchEvent(new Event('input',{bubbles:true}))`);await delay(150);
await evaluate(`document.querySelector('.cn-mapping-edit').blur()`);await delay(150);
await check('grid retains edited path and reports change', `document.querySelector('.cn-data-panel').textContent.includes('account.customer_key') && document.querySelector('.cn-data-footer').textContent.includes('1 edited')`);
await evaluate(`[...document.querySelectorAll('.cn-data-panel button')].find(b=>b.textContent.includes('Reset changes')).click()`);await delay(150);
await check('reset restores original mapping', `document.querySelector('.cn-data-footer').textContent.includes('No changes') && document.querySelector('.cn-data-panel').textContent.includes('account.external_id')`);
await call('Page.captureScreenshot',{format:'png'}).then(shot=>fs.writeFileSync('/tmp/conctr-premium-grid.png',Buffer.from(shot.data,'base64')));
await call('Emulation.setDeviceMetricsOverride',{width:390,height:844,deviceScaleFactor:1,mobile:false});await delay(100);
await check('mapping grid scrolls inside mobile frame', `document.documentElement.scrollWidth<=390 && document.querySelector('[data-name="DataGrid"]').scrollWidth>document.querySelector('[data-name="DataGrid"]').clientWidth`);
await call('Emulation.setDeviceMetricsOverride',{width:1280,height:1000,deviceScaleFactor:1,mobile:false});
await navigate('/components/toast','.cn-toast-demo');
await check('toast controls and previews have distinct sections', `!!document.querySelector('section[aria-label="Notification controls"]') && document.querySelectorAll('section[aria-label="Message previews"] .cn-toast-message').length===3`);
await evaluate(`[...document.querySelectorAll('.cn-toast-buttons button')].find(b=>b.textContent.includes('Show success feedback')).click()`);await delay(150);
await check('live toast keeps copy and dismissal inside one card', `!!document.querySelector('.cn-toast-region .cn-toast-message .cn-toast-close') && document.querySelector('.cn-toast-region').textContent.includes('Connection created')`);
await check('toast copy has no nested colored card', `getComputedStyle(document.querySelector('.cn-toast-region [role="alert"]')).backgroundColor==='rgba(0, 0, 0, 0)' && getComputedStyle(document.querySelector('.cn-toast-region [role="alert"]')).padding==='0px'`);
await call('Page.captureScreenshot',{format:'png'}).then(shot=>fs.writeFileSync('/tmp/conctr-premium-toast.png',Buffer.from(shot.data,'base64')));
await evaluate(`document.querySelector('.cn-toast-close').click()`);await delay(100);
await check('live toast dismisses', `!document.querySelector('.cn-toast-region')`);
await evaluate(`document.querySelector('button[aria-label="Switch to dark theme"]').click()`);await delay(100);
await check('toast and preview follow dark palette', `getComputedStyle(document.querySelector('.cn-body')).getPropertyValue('--cn-surface').trim().toUpperCase()==='#121933' && getComputedStyle(document.querySelector('.cn-toast-message')).backgroundColor!=='rgb(255, 255, 255)'`);
await call('Page.captureScreenshot',{format:'png'}).then(shot=>fs.writeFileSync('/tmp/conctr-premium-toast-dark.png',Buffer.from(shot.data,'base64')));
await call('Emulation.setDeviceMetricsOverride',{width:390,height:844,deviceScaleFactor:1,mobile:false});await delay(100);
await check('toast demo fits mobile viewport', `document.documentElement.scrollWidth<=390`);
if(errors.length)throw Error(JSON.stringify(errors));
console.log(JSON.stringify({checks:checks.length,errors}));ws.close();
