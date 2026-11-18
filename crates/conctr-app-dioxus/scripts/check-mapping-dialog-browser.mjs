// Node 22+, with Chrome running with a remote debugging port.
const base = process.argv[2] || "http://127.0.0.1:8092";
const debug = process.argv[3] || "http://127.0.0.1:9225";
const tabs=await (await fetch(`${debug}/json`)).json();
const ws=new WebSocket(tabs[0].webSocketDebuggerUrl); await new Promise(r=>ws.onopen=r);
let id=0;const pending=new Map();ws.onmessage=e=>{const m=JSON.parse(e.data);if(m.id){pending.get(m.id)?.(m);pending.delete(m.id)}};
const call=(method,params={})=>new Promise(r=>{const n=++id;pending.set(n,r);ws.send(JSON.stringify({id:n,method,params}))});

const evaluate=async expression=>(await call('Runtime.evaluate',{expression,returnByValue:true})).result.result.value;
const pause=()=>new Promise(r=>setTimeout(r,350));
const click=async text=>{await evaluate(`Array.from(document.querySelectorAll('button')).find(b=>b.textContent.trim().endsWith(${JSON.stringify(text)}))?.click()`);await pause()};
const assert=async (expression,label)=>{if(!await evaluate(expression))throw Error(label);console.log('PASS',label)};

await call('Emulation.setDeviceMetricsOverride',{width:1600,height:1100,deviceScaleFactor:1,mobile:false});
await call('Page.navigate',{url:`${base}/mappings`});
for(let n=0;n<120;n++){if(await evaluate('!!document.querySelector("[data-editor-ready]")'))break;await new Promise(r=>setTimeout(r,500));}
const openEdit=async()=>{await evaluate(`document.querySelector('[data-field-actions]').click()`);await pause();await evaluate(`[...document.querySelector('[data-field-menu]:popover-open').querySelectorAll('button')].find(b=>b.textContent.includes('Edit field')).click()`);await pause()};
const centered=`(()=>{const r=document.querySelector('[data-node-dialog]').getBoundingClientRect();return Math.abs(r.x+r.width/2-innerWidth/2)<2 && Math.abs(r.y+r.height/2-innerHeight/2)<2})()`;
const contained=`(()=>{const r=document.querySelector('[data-node-dialog]').getBoundingClientRect();return r.x>=15 && r.y>=15 && r.right<=innerWidth-15 && r.bottom<=innerHeight-15})()`;
const fill=async(label,value)=>{await evaluate(`var e=document.querySelector('[aria-label="${label}"]');Object.getOwnPropertyDescriptor(HTMLInputElement.prototype,'value').set.call(e,${JSON.stringify(value)});e.dispatchEvent(new Event('input',{bubbles:true}))`);await pause()};
await openEdit();
await assert(centered,'Edit dialog centered in desktop viewport');
await assert(`document.activeElement?.getAttribute('aria-label')==='Node name'`,'Name field receives initial focus');
await assert(`document.querySelector('[aria-label="Node datatype"]').value==='string'`,'Existing data type selected');
await fill('Node path',"$['email']");await evaluate(`document.querySelector('[data-node-dialog] form').requestSubmit()`);await pause();
await assert(`document.querySelector('[data-node-dialog] [role="alert"]').textContent.includes('already exists')`,'Duplicate path rejected within dialog');
await assert(centered,'Dialog stays centered after validation');
await fill('Node path',"$['order_id']");await fill('Node name','Order reference');await click('Save changes');
await assert(`!document.querySelector('[data-node-dialog]') && document.querySelector('[data-field-path]').textContent.includes('Order reference')`,'Save changes updates field and closes dialog');
await openEdit();await evaluate(`document.querySelector('[aria-label="Close field dialog"]').click()`);await pause();
await assert(`!document.querySelector('[data-node-dialog]')`,'Close button dismisses dialog');
await click('New target');await assert(centered,'Create dialog also centered');await click('Cancel');
await evaluate(`document.querySelector('[aria-label="Switch to dark theme"]').click()`);await pause();await openEdit();
await assert(`getComputedStyle(document.querySelector('[data-node-dialog]')).getPropertyValue('--cn-bg').trim().toLowerCase()==='#0b1020'`,'Dialog inherits dark theme');
await call('Input.dispatchKeyEvent',{type:'keyDown',key:'Escape',code:'Escape',windowsVirtualKeyCode:27});await call('Input.dispatchKeyEvent',{type:'keyUp',key:'Escape',code:'Escape',windowsVirtualKeyCode:27});await pause();
await assert(`!document.querySelector('[data-node-dialog]')`,'Escape dismisses dialog');
await call('Emulation.setDeviceMetricsOverride',{width:390,height:844,deviceScaleFactor:1,mobile:true});await pause();await openEdit();
await assert(centered,'Dialog centered on mobile');await assert(contained,'Mobile dialog fits viewport with margins');
await call('Emulation.setDeviceMetricsOverride',{width:390,height:480,deviceScaleFactor:1,mobile:true});await pause();
await assert(contained,'Short viewport keeps dialog contained');
await assert(`(()=>{const d=document.querySelector('[data-node-dialog]');return d.scrollHeight>d.clientHeight && getComputedStyle(d).overflowY==='auto'})()`,'Short viewport allows dialog scrolling');
ws.close();
