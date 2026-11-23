// Run with Node 22+ and a Chrome instance using --remote-debugging-port=19222.
import fs from 'node:fs';
const base = process.argv[2] || 'http://127.0.0.1:18083';
const debug = process.argv[3] || 'http://127.0.0.1:19224';
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
for (const slug of ['field-mapping','calendar','composed-date-picker','composed-input-group','badges-and-shapes','feedback','empty-state','connection-flow','tags','tag-input','time-picker','date-time-picker']) {
 await navigate('/components/'+slug,'main pre');
 await check(slug+' shows consumer usage, collapsed initially', `(()=>{const d=document.querySelector('main details');return !d.open && d.textContent.includes('use conctr_ui_dioxus') && d.textContent.includes('Demo')})()`);
}
await evaluate(`document.querySelector('main button[aria-label="Choose date"]').click()`);await ready('.cn-composed-calendar');
await evaluate(`document.querySelector('.cn-composed-calendar button[aria-label="2026-11-20"]').click()`);await delay(150);
await evaluate(`var timeInput=document.querySelector('main input[type="time"]');timeInput.value='14:35';timeInput.dispatchEvent(new Event('input',{bubbles:true}))`);await delay(150);
await check('date and time combine into caller value', `document.querySelector('main').textContent.includes('2026-11-20') && document.querySelector('main').textContent.includes('14:35') && document.querySelector('main').textContent.includes('(local time)')`);
await navigate('/components/tags','.cn-tag');
await check('kit-sized tags expose six semantic variants', `new Set([...document.querySelectorAll('main .cn-tag')].map(e=>e.dataset.variant)).size===6 && getComputedStyle(document.querySelector('main .cn-tag')).height==='24px' && getComputedStyle(document.querySelector('main .cn-tag')).borderRadius==='6px'`);
await evaluate(`document.querySelector('main button[aria-label="Remove Production"]').click()`);await delay(150);
await check('removable tags update caller state', `!document.querySelector('main button[aria-label="Remove Production"]')`);
await navigate('/components/tag-input','.cn-taginput');
await evaluate(`var tagInput=document.querySelector('.cn-taginput input:not(:disabled)');tagInput.focus();tagInput.value='Finance';tagInput.dispatchEvent(new Event('input',{bubbles:true}))`);await delay(100);
await evaluate(`document.querySelector('.cn-taginput input:not(:disabled)').dispatchEvent(new KeyboardEvent('keydown',{key:'Enter',bubbles:true}))`);await delay(150);
await check('tag input adds with Enter', `document.querySelector('.cn-taginput').textContent.includes('Finance')`);
await check('disabled tag input also disables removal', `[...document.querySelectorAll('.cn-taginput[data-disabled="true"] button')].every(b=>b.disabled)`);
await check('showcase uses a simple scrollable component explorer', `!document.querySelector('.cn-gallery-sidebar') && !document.querySelector('.cn-sidebar-provider') && !!document.querySelector('.cn-component-explorer') && getComputedStyle(document.querySelector('.cn-component-explorer')).overflowY==='auto'`);
await check('showcase has its new name', `document.title==='Conctr Design Kit' && document.querySelector('header').textContent.includes('Conctr Design Kit')`);
await navigate('/components/sidebar','main .cn-sidebar-provider[data-ready]');
await check('desktop trigger straddles the sidebar border', `Math.abs(document.querySelector('main .cn-sidebar-border-trigger').getBoundingClientRect().x + document.querySelector('main .cn-sidebar-border-trigger').offsetWidth / 2 - document.querySelector('main .cn-sidebar-desktop').getBoundingClientRect().right) <= 1`);
await evaluate(`document.querySelector('main .cn-sidebar-border-trigger').click()`);await delay(150);
await check('collapsed rail shows the compact Conctr logo and expand chevron', `document.querySelector('main .cn-sidebar-head > img[alt="Conctr"]').getBoundingClientRect().width===32 && !!document.querySelector('main .cn-sidebar-border-trigger [data-name="ChevronRight"]')`);
await check('border trigger collapses sidebar to icon rail', `document.querySelector('main .cn-sidebar-desktop').getBoundingClientRect().width===56`);
await evaluate(`document.querySelector('main .cn-sidebar-border-trigger').click()`);await delay(150);
await check('expanded sidebar shows the collapse chevron', `!!document.querySelector('main .cn-sidebar-border-trigger [data-name="ChevronLeft"]')`);
await check('border trigger expands sidebar labels', `document.querySelector('main .cn-sidebar-desktop').getBoundingClientRect().width===240`);
await call('Emulation.setDeviceMetricsOverride',{width:390,height:844,deviceScaleFactor:1,mobile:true});
await evaluate(`document.querySelector('main [data-sidebar-trigger]').focus();document.querySelector('main [data-sidebar-trigger]').click()`);await delay(100);
await check('mobile sidebar contains focus', `document.querySelector('main .cn-sidebar-drawer').matches(':modal') && document.querySelector('main .cn-sidebar-drawer').contains(document.activeElement)`);
await call('Input.dispatchKeyEvent',{type:'keyDown',key:'Escape',code:'Escape',windowsVirtualKeyCode:27});await call('Input.dispatchKeyEvent',{type:'keyUp',key:'Escape',code:'Escape',windowsVirtualKeyCode:27});await delay(100);
await check('mobile sidebar Escape restores main trigger focus', `!document.querySelector('main .cn-sidebar-drawer').open && document.activeElement.matches('section [data-sidebar-trigger]')`);
await call('Emulation.setDeviceMetricsOverride',{width:1280,height:1000,deviceScaleFactor:1,mobile:false});
await navigate('/components/button','main pre');
await check('button demo includes ghost and icon usage without ButtonAction', `!!document.querySelector('main button[aria-label="More options"]') && document.querySelector('main pre').textContent.includes('ButtonVariant::Ghost') && !document.querySelector('main').textContent.includes('ButtonAction') && document.querySelector('main button[aria-label="Add connection"]').getBoundingClientRect().width===40`);
await navigate('/components/command','.cn-command');
await check('command palette matches kit frame and keyboard hints', `(()=>{const e=document.querySelector('.cn-command');const s=getComputedStyle(e);return s.borderRadius==='20px' && s.boxShadow!=='none' && e.querySelectorAll('kbd').length>=5 && e.querySelector('input').placeholder==='Search or run a command'})()`);
await evaluate(`var commandInput=document.querySelector('.cn-command input');commandInput.value='Warehouse';commandInput.dispatchEvent(new Event('input',{bubbles:true}))`);await delay(200);
await check('command filters connection results', `[...document.querySelector('.cn-command').querySelectorAll('[data-name="CommandItemLink"]')].filter(e=>getComputedStyle(e).display!=='none').length===1`);
await call('Page.captureScreenshot',{format:'png'}).then(r=>fs.writeFileSync('/tmp/conctr-command-fixed.png',Buffer.from(r.data,'base64')));
await navigate('/components/drawer','.cn-drawer-root');
await check('drawer examples contain no separate Dialog demo', `document.querySelectorAll('main .cn-drawer-root').length===2 && !document.querySelector('main [data-name="Dialog"]') && !document.querySelector('main').textContent.includes('DemoDrawerDialog')`);
await evaluate(`document.querySelector('main [data-name="DrawerTrigger"]').click()`);await delay(100);
await check('primary drawer opens as modal', `!!document.querySelector('main .cn-drawer-surface:modal')`);
await evaluate(`document.querySelector('main .cn-drawer-surface[open] [data-name="DrawerClose"]').click()`);await delay(100);
await check('primary close button dismisses drawer', `!document.querySelector('main .cn-drawer-surface[open]')`);
await evaluate(`document.querySelectorAll('main [data-name="DrawerTrigger"]')[1].click()`);await delay(100);
await evaluate(`document.querySelector('main .cn-drawer-surface[open] .cn-drawer-icon-close').click()`);await delay(100);
await check('last drawer close icon dismisses drawer', `!document.querySelector('main .cn-drawer-surface[open]')`);
await navigate('/components/toast','.cn-toast-buttons');
await evaluate(`var successButton=[...document.querySelectorAll('.cn-toast-buttons button')].find(b=>b.textContent.includes('Show success'));successButton.click();successButton.click();successButton.click()`);await delay(200);
await check('repeated toast clicks stack three notifications', `document.querySelectorAll('.cn-toast-stack-item').length===3 && document.querySelectorAll('.cn-toast-stack').length===1`);
await evaluate(`[...document.querySelectorAll('.cn-toast-buttons button')].find(b=>b.textContent.includes('Show error')).click()`);await delay(100);
await check('different triggers share the notification stack', `document.querySelectorAll('.cn-toast-stack-item').length===4`);
await evaluate(`document.querySelector('.cn-toast-stack-item .cn-toast-close').click()`);await delay(100);
await check('closing one toast preserves other notifications', `document.querySelectorAll('.cn-toast-stack-item').length===3 && document.querySelector('.cn-toast-stack').textContent.includes('Stock sync failed')`);
await check('stacked toast rectangles do not overlap', `(()=>{const r=[...document.querySelectorAll('.cn-toast-stack-item')].map(e=>e.getBoundingClientRect());return r.every((e,i)=>i===0||e.top>=r[i-1].bottom)})()`);
await call('Page.captureScreenshot',{format:'png'}).then(r=>fs.writeFileSync('/tmp/conctr-toasts-stacked.png',Buffer.from(r.data,'base64')));
await evaluate(`document.querySelector('[aria-label="Switch to dark theme"]').click()`);await delay(100);
await check('sidebar and notifications support dark palette', `getComputedStyle(document.querySelector('.cn-body')).getPropertyValue('--cn-bg').trim().toUpperCase()==='#0B1020' && getComputedStyle(document.querySelector('.cn-toast-message')).backgroundColor!=='rgb(255, 255, 255)'`);
await call('Emulation.setDeviceMetricsOverride',{width:390,height:844,deviceScaleFactor:1,mobile:false});await delay(100);
await check('toast stack stays inside mobile viewport', `document.documentElement.scrollWidth<=390 && document.querySelector('.cn-toast-stack').getBoundingClientRect().right<=390`);
await call('Emulation.setDeviceMetricsOverride',{width:1440,height:1100,deviceScaleFactor:1,mobile:false});
await navigate('/components/field-mapping','.cn-field-mapping');
await check('request and response mapping variants render separately', `document.querySelectorAll('.cn-field-mapping').length===2 && document.querySelector('.cn-field-mapping').textContent.includes('Customer portal') && document.querySelectorAll('.cn-field-mapping')[1].textContent.includes('Create customer · response body')`);
await check('initial select labels match the supplied mapping draft', `(()=>{const e=document.querySelector('.cn-field-mapping');return [...e.querySelectorAll('[aria-label^="Source field"]')].map(s=>s.value).join(',')==='customerId,name,email' && [...e.querySelectorAll('[aria-label^="Transform for"]')].map(s=>s.value).join(',')===',trim,lowercase'})()`);
await check('mapping nodes have visible connectors and transforms', `document.querySelectorAll('.cn-field-mapping:first-of-type .cn-field-map-wire').length>=3 && getComputedStyle(document.querySelector('.cn-field-map-field')).borderRadius==='10px' && document.querySelector('.cn-field-map-transform-lane[data-transformed="true"]')`);
await evaluate(`var editor=document.querySelector('.cn-field-mapping');var source=editor.querySelector('[aria-label="Source field for mapping 1"]');source.value='creditLimit';source.dispatchEvent(new Event('change',{bubbles:true}))`);await delay(150);
await check('changing source updates caller draft and field metadata', `document.querySelector('.cn-field-map-row code').textContent==='$.customer.creditLimit' && document.querySelector('.cn-field-map-row .cn-field-map-type').textContent==='number'`);
await evaluate(`var transform=document.querySelector('[aria-label="Transform for mapping 1"]');transform.value='number';transform.dispatchEvent(new Event('change',{bubbles:true}))`);await delay(150);
await check('transform selection updates the mapping node', `document.querySelector('.cn-field-map-row .cn-field-map-transform-lane').dataset.transformed==='true' && document.querySelector('.cn-field-map-transform').title.includes('numeric conversion')`);
await evaluate(`var transform=document.querySelector('[aria-label="Transform for mapping 1"]');transform.value='';transform.dispatchEvent(new Event('change',{bubbles:true}))`);await delay(150);
await check('transform can return to direct copy', `document.querySelector('.cn-field-map-transform-lane').dataset.transformed==='false'`);
await check('targets used by other rows are disabled', `document.querySelector('[aria-label="Target field for mapping 1"] option[value="CustomerName"]').disabled`);
await evaluate(`var target=document.querySelector('[aria-label="Target field for mapping 1"]');target.value='CreditLimit';target.dispatchEvent(new Event('change',{bubbles:true}))`);await delay(150);
await check('changing target updates its displayed path', `document.querySelector('.cn-field-map-target code').textContent==='$.CreditLimit'`);
await evaluate(`document.querySelector('.cn-field-mapping button').click()`);await delay(150);
await check('add mapping uses a free target and stops when full', `document.querySelectorAll('.cn-field-mapping')[0].querySelectorAll('.cn-field-map-row').length===4 && document.querySelector('.cn-field-mapping button').disabled && document.querySelector('.cn-field-mapping .cn-field-map-footer').textContent.includes('4 mappings')`);
await evaluate(`document.querySelector('[aria-label="Remove mapping 2"]').click()`);await delay(150);
await check('removing a mapping preserves neighboring drafts', `document.querySelectorAll('.cn-field-mapping')[0].querySelectorAll('.cn-field-map-row').length===3 && document.querySelector('[aria-label="Source field for mapping 1"]').value==='creditLimit' && !document.querySelector('.cn-field-mapping button').disabled`);
for(let i=0;i<3;i++){await evaluate(`document.querySelector('.cn-field-mapping [aria-label^="Remove mapping"]').click()`);await delay(100);}
await check('empty mapping state remains actionable', `!!document.querySelector('.cn-field-map-empty') && !document.querySelector('.cn-field-mapping button').disabled`);
await evaluate(`document.querySelector('.cn-field-mapping button').click()`);await delay(150);
await check('mapping can be added after removing all rows', `document.querySelectorAll('.cn-field-mapping')[0].querySelectorAll('.cn-field-map-row').length===1`);
await navigate('/components/field-mapping','.cn-field-mapping');
await call('Page.captureScreenshot',{format:'png'}).then(r=>fs.writeFileSync('/tmp/conctr-mapping-light.png',Buffer.from(r.data,'base64')));
await evaluate(`document.querySelector('[aria-label="Switch to dark theme"]').click()`);await delay(150);
await check('mapping uses dark surfaces and visible connectors', `getComputedStyle(document.querySelector('.cn-field-map-field')).backgroundColor!=='rgb(255, 255, 255)' && getComputedStyle(document.querySelector('.cn-field-map-wire')).stroke!=='none'`);
await call('Page.captureScreenshot',{format:'png'}).then(r=>fs.writeFileSync('/tmp/conctr-mapping-dark.png',Buffer.from(r.data,'base64')));
await call('Emulation.setDeviceMetricsOverride',{width:390,height:844,deviceScaleFactor:1,mobile:true});await delay(150);
await check('mapping scrolls inside the mobile viewport', `document.documentElement.scrollWidth<=390 && document.querySelector('.cn-field-map-scroll').scrollWidth>document.querySelector('.cn-field-map-scroll').clientWidth`);
await check('mobile mapping controls retain touch target height', `document.querySelector('.cn-field-map-field select').getBoundingClientRect().height>=44`);
await call('Page.captureScreenshot',{format:'png'}).then(r=>fs.writeFileSync('/tmp/conctr-mapping-mobile.png',Buffer.from(r.data,'base64')));
if(errors.length)throw Error(JSON.stringify(errors));
console.log(JSON.stringify({checks:checks.length,errors}));ws.close();
