// Run with Node 22+ and a Chrome instance using --remote-debugging-port=19222.
import fs from 'node:fs';
const base = process.argv[2] || 'http://127.0.0.1:18080';
const debug = process.argv[3] || 'http://127.0.0.1:19222';
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
 await ready('main [data-dioxus-id]');
 await delay(500);
}
await call('Page.enable'); await call('Runtime.enable');
await call('Emulation.setDeviceMetricsOverride', {width:1280,height:900,deviceScaleFactor:1,mobile:false});
await navigate('/components/date-picker', 'main pre');
await evaluate(`document.querySelector('main button[aria-label="Choose date"]').click()`);
if (!await evaluate(`!!document.querySelector('.cn-composed-calendar')`)) { await delay(2500); await evaluate(`document.querySelector('main button[aria-label="Choose date"]').click()`); }
await ready('.cn-composed-calendar');
await evaluate(`document.querySelector('.cn-composed-calendar button[aria-label="2026-11-18"]').click()`);
await delay(150);
await check('date picker selects and closes', `document.querySelector('main').textContent.includes('Selected date: 2026-11-18') && !document.querySelector('.cn-composed-calendar')`);
await check('consumer imports and formatted usage code', `document.querySelector('main pre').textContent.includes('use conctr_ui_dioxus::') && document.querySelector('main pre').textContent.includes('    let mut selected')`);
await check('one or two additional variants', `document.querySelectorAll('main section').length >= 1 && document.querySelectorAll('main section').length <= 2`);
await check('catalog is a scrollable component explorer', `!!document.querySelector('.cn-component-explorer') && getComputedStyle(document.querySelector('.cn-component-explorer')).overflowY==='auto'`);
await navigate('/components/shimmer','[data-name="Shimmer"]');
await evaluate(`[...document.querySelectorAll('main button')].find(b=>b.textContent.includes('Toggle')).click()`);await delay(100);
await check('shimmer animates a loading mask', `(()=>{const e=document.querySelector('[data-name="Shimmer"]');return e.getAttribute('aria-busy')==='true' && getComputedStyle(e,'::after').animationName==='cn-shimmer-sweep' && e.firstElementChild.inert})()`);
await call('Emulation.setEmulatedMedia',{features:[{name:'prefers-reduced-motion',value:'reduce'}]});
await check('shimmer respects reduced motion', `getComputedStyle(document.querySelector('[data-name="Shimmer"]'),'::after').animationName==='none'`);
await call('Emulation.setEmulatedMedia',{features:[]});
await navigate('/components/input-otp','[data-slot="input-otp-slot"]');
await check('OTP has separated rounded slots', `(()=>{const s=getComputedStyle(document.querySelector('[data-slot="input-otp-slot"]'));return s.borderRadius==='10px' && s.width==='40px' && s.height==='48px' && s.borderLeftWidth==='1px'})()`);
await evaluate(`(()=>{const e=document.querySelector('input[autocomplete="one-time-code"]');e.focus();e.value='123456';e.dispatchEvent(new Event('input',{bubbles:true}));})()`);await delay(150);
await check('OTP accepts native input', `document.querySelector('[data-slot="input-otp"]').textContent.includes('1') && document.querySelector('input[autocomplete="one-time-code"]').value==='123456'`);
await navigate('/components/input-phone','[data-name="InputPhoneWrapper"]');
await check('phone field shares one aligned border', `(()=>{const r=document.querySelector('[data-name="InputPhoneWrapper"]');const b=r.querySelector('button');const i=r.querySelector('input[type=tel]');return b.getBoundingClientRect().bottom===i.getBoundingClientRect().bottom && getComputedStyle(i).borderBottomWidth==='0px' && getComputedStyle(r).borderBottomWidth==='1px'})()`);
await navigate('/components/toast','main button');
await evaluate(`[...document.querySelectorAll('main button')].find(b=>b.textContent.includes('Show success feedback')).click()`);await delay(150);
await check('toast uses feedback title and next action', `document.querySelector('.cn-toast-stack').textContent.includes('Connection created') && document.querySelector('.cn-toast-stack').textContent.includes('Run a test')`);
await evaluate(`document.querySelector('.cn-toast-stack button').click()`);await delay(100);
await check('toast dismisses', `!document.querySelector('.cn-toast-stack')`);
await navigate('/components/sidebar','.cn-sidebar');
await evaluate(`document.querySelector('main .cn-sidebar-toggle').click()`);await delay(150);
await check('sidebar component collapses', `document.querySelector('main .cn-sidebar').dataset.collapsed==='true'`);
await navigate('/components/data-table','main table');
await check('data table has premium rounded frame', `getComputedStyle(document.querySelector('.cn-data-panel')).borderRadius==='14px'`);
await navigate('/components/data-grid','[data-name="DataGrid"]');
await check('grid has rounded frame and quiet dividers', `getComputedStyle(document.querySelector('.cn-data-panel')).borderRadius==='14px' && getComputedStyle(document.querySelector('[data-name="GridCell"]')).borderBottomWidth==='1px'`);
await call('Emulation.setDeviceMetricsOverride', {width:390,height:844,deviceScaleFactor:1,mobile:false});
await check('grid and usage code stay inside mobile viewport', `document.documentElement.scrollWidth<=390`);
await call('Emulation.setDeviceMetricsOverride', {width:1280,height:900,deviceScaleFactor:1,mobile:false});
if(errors.length) throw Error(JSON.stringify(errors));
console.log(JSON.stringify({checks:checks.length,errors}));ws.close();
