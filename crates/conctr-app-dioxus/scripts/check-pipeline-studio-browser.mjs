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
await call('Page.navigate',{url:`${base}/pipelines`});
for(let n=0;n<120;n++){if(await evaluate('document.querySelectorAll("[data-pipeline-node]").length===8'))break;await new Promise(r=>setTimeout(r,500));}
await pause();
await assert('document.querySelectorAll("[data-pipeline-node]").length===8','Sample pipeline has eight workflow nodes');
await assert('document.querySelectorAll("[data-testid=source-handle]").length===6','Workflow source ports render');
await assert('document.querySelectorAll("[data-testid=target-handle]").length===7','Workflow target ports render');
await assert('document.querySelector("[aria-label=\\\"Step library\\\"]").textContent.includes("HTTP trigger")','Step library renders');
const mouse=async (type,x,y,button='left')=>call('Input.dispatchMouseEvent',{type,x,y,button,buttons:type==='mouseReleased'?0:1,clickCount:1});
const point=async selector=>evaluate(`(()=>{const r=document.querySelector(${JSON.stringify(selector)}).getBoundingClientRect();return {x:r.x+r.width/2,y:r.y+r.height/2}})()`);
const select=async id=>{const p=await point(`[data-pipeline-node="${id}"]`);await mouse('mousePressed',p.x,p.y);await mouse('mouseReleased',p.x,p.y);await pause();};
await select('step-2');
await assert('document.querySelector("[aria-label=\\\"Step inspector\\\"]").textContent.includes("Open mapping editor")','Selecting a mapping opens its inspector');
await select('step-0');
let p=await point('[data-pipeline-node="step-0"]');
const before=await evaluate('document.querySelector("[data-pipeline-node=step-0]").getBoundingClientRect().x');
await mouse('mousePressed',p.x,p.y);await mouse('mouseMoved',p.x+35,p.y+25);await mouse('mouseReleased',p.x+35,p.y+25);await pause();
await assert(`document.querySelector('[data-pipeline-node="step-0"]').getBoundingClientRect().x>${before+20}`,'Workflow node dragging moves the node');
await click('Delay');
await assert('document.querySelectorAll("[data-pipeline-node]").length===9','Palette adds a step');
await assert('document.querySelector("[aria-label=\\\"Step inspector\\\"]").textContent.includes("Delay")','New step is selected');
await click('Duplicate');
await assert('document.querySelectorAll("[data-pipeline-node]").length===10','Inspector duplicates selected step');
await click('Delete step');
await assert('document.querySelectorAll("[data-pipeline-node]").length===9','Deleting a step updates canvas');
await click('Check draft');
await assert('document.querySelector("[role=status]").textContent.includes("no incoming connection")','Draft checker reports disconnected steps');
await click('Fit canvas');

// Respond is terminal; connect API call to the newly added delay instead.

const source=await point('[data-testid="source-handle"][data-node-id="step-5"]');
const target=await point('[data-testid="target-handle"][data-node-id="step-100"]');
await mouse('mousePressed', source.x, source.y);
const targetBody = await point('[data-pipeline-node="step-100"]');
for (const cursor of [targetBody, target]) {
    await mouse('mouseMoved', cursor.x, cursor.y);
    await pause();
    await assert(`(() => {
        const guide = document.querySelector('path[stroke-dasharray="5 4"]');
        if (!guide) return false;
        const end = guide.getPointAtLength(guide.getTotalLength()).matrixTransform(guide.getScreenCTM());
        return Math.hypot(end.x - ${cursor.x}, end.y - ${cursor.y}) < 3;
    })()`, 'Connection preview follows the pointer over nested canvas elements');
}
await mouse('mouseReleased', target.x, target.y);
await pause();
await assert('document.querySelector("[aria-label=\\\"Pipeline actions\\\"]").textContent.includes("9 connections")','Dragging workflow ports creates a connection');
await evaluate(`document.querySelector('[aria-label="Minimize step inspector"]').click()`);await pause();
await assert('document.querySelector("[data-panel-mode]").dataset.panelMode==="minimized"','Inspector minimizes');
await evaluate(`document.querySelector('[aria-label="Restore step inspector"]').click()`);await pause();
await assert('document.querySelector("[data-panel-mode]").dataset.panelMode==="normal"','Inspector restores');
await evaluate(`document.querySelector('[aria-label="Resize step inspector"]').dispatchEvent(new KeyboardEvent('keydown',{key:'ArrowLeft',bubbles:true}))`);await pause();
await assert('document.querySelector("[aria-label=\\\"Step inspector\\\"]").getBoundingClientRect().width>304','Inspector keyboard resize works');
// Editable fields and branch labels use the shared Input facade.
const setInput=async (selector,value)=>{await evaluate(`(()=>{const e=document.querySelector(${JSON.stringify(selector)});Object.getOwnPropertyDescriptor(HTMLInputElement.prototype,'value').set.call(e,${JSON.stringify(value)});e.dispatchEvent(new Event('input',{bubbles:true}));})()`);await pause();};
await select('step-2');
await setInput('[aria-label="Step inspector"] input','Order mapping updated');
await assert(`document.querySelector('[data-pipeline-node="step-2"]').textContent.includes('Order mapping updated')`,'Inspector edits update node title');
await setInput('input[type="search"]','Delay');
await assert(`document.querySelector('[aria-label="Step library"]').textContent.includes('Delay')&&!document.querySelector('[aria-label="Step library"]').textContent.includes('HTTP trigger')`,'Step library search filters results');
await setInput('input[type="search"]','');
await evaluate(`document.querySelector('[aria-label="Pipeline workspace"] svg g path').dispatchEvent(new MouseEvent('click',{bubbles:true}))`);await pause();
await assert(`document.querySelector('[aria-label="Step inspector"]').textContent.includes('Branch label')`,'Selecting an edge opens connection details');
await setInput('[aria-label="Step inspector"] input','Next');
await assert(`[...document.querySelectorAll('svg text')].some(t=>t.textContent==='Next')`,'Branch label edits render on the connection');
await evaluate(`document.querySelector('[aria-label="Maximize step inspector"]').click()`);await pause();
await assert(`document.querySelector('[aria-label="Step inspector"]').getBoundingClientRect().width>1100`,'Inspector maximizes across the workspace');
await evaluate(`document.querySelector('[aria-label="Restore step inspector"]').click()`);await pause();
await evaluate(`document.querySelector('[aria-label="Switch to dark theme"]').click()`);await pause();
await assert(`getComputedStyle(document.querySelector('[aria-label="Pipeline workspace"]')).backgroundColor!=='rgb(255, 255, 255)'`,'Pipeline supports the dark theme');
await evaluate(`document.querySelector('[aria-label="Switch to light theme"]').click()`);await pause();
await call('Emulation.setDeviceMetricsOverride',{width:390,height:844,deviceScaleFactor:1,mobile:true});await pause();
await assert('document.documentElement.scrollWidth<=390','Mobile layout has no horizontal page overflow');
await call('Emulation.setDeviceMetricsOverride',{width:1600,height:1100,deviceScaleFactor:1,mobile:false});await pause();
console.log('Pipeline browser checks passed');
ws.close();
