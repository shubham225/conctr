//! Delegated pointer gestures survive node updates. Layout changes are emitted to
//! Rust through a scoped input; link semantics never depend on DOM coordinates.
pub(super) const SCRIPT: &str = r#"
requestAnimationFrame(()=>document.querySelectorAll('[data-visual-mapper]:not([data-editor-ready])').forEach(root=>{
 root.dataset.editorReady='true';
 let drag=null, ghost=null, hover=null, suppress=false;
 const clear=()=>{if(hover)delete hover.dataset.dropReady;hover=null;};
 const reset=()=>{clear();ghost?.remove();drag?.overlay?.remove();ghost=null;drag=null;};
 const emit=value=>{
   const input=root.querySelector('[data-editor-events]');
   Object.getOwnPropertyDescriptor(HTMLInputElement.prototype,'value').set.call(input,JSON.stringify(value));
   input.dispatchEvent(new Event('input',{bubbles:true}));
 };


 // Native popovers supply outside-click/Escape dismissal without global listeners.
 let menuReturn=null;
 const openActions=(element,x,y)=>{
   const card=element.closest('[data-node-path]'), menu=card?.querySelector('[data-field-menu]');if(!menu)return;
   menuReturn=card.querySelector('[data-field-actions]');
   const r=element.getBoundingClientRect();
   menu.showPopover();
   menu.style.left=Math.max(8,Math.min(x??r.left,innerWidth-menu.offsetWidth-8))+'px';
   menu.style.top=Math.max(8,Math.min(y??r.bottom,innerHeight-menu.offsetHeight-8))+'px';
   menu.querySelector('button')?.focus();
 };
 root.addEventListener('click',e=>{
   const button=e.target.closest('[data-field-actions]');if(button){e.preventDefault();openActions(button)}
   const menu=e.target.closest('[data-field-menu]');if(menu&&e.target.closest('[data-context-close]')){menu.hidePopover();menuReturn?.focus()}
 });
 root.addEventListener('contextmenu',e=>{
   if(!e.target.closest('[data-node-path]')?.querySelector('[data-field-menu]'))return;
   e.preventDefault();openActions(e.target,e.clientX,e.clientY);
 });
 root.addEventListener('keydown',e=>{
   const menu=e.target.closest('[data-field-menu]');
   if(menu){
     const buttons=[...menu.querySelectorAll('button:not(:disabled)')];let i=buttons.indexOf(document.activeElement);
     if(e.key==='ArrowDown')i=(i+1)%buttons.length;
     else if(e.key==='ArrowUp')i=(i-1+buttons.length)%buttons.length;
     else if(e.key==='Home')i=0;else if(e.key==='End')i=buttons.length-1;
     else if(e.key==='Escape'){menu.hidePopover();menuReturn?.focus();e.preventDefault();return;}else return;
     e.preventDefault();e.stopPropagation();buttons[i]?.focus();
   }else if(e.key==='ContextMenu'||(e.shiftKey&&e.key==='F10')){e.preventDefault();openActions(e.target);}
 });
 root.addEventListener('dragstart',e=>e.preventDefault());
 root.addEventListener('click',e=>{if(suppress){e.preventDefault();e.stopImmediatePropagation();suppress=false;}},true);
 root.addEventListener('pointerdown',e=>{
   if(e.button!==0||root.dataset.readOnly==='true'||e.target.closest('[data-field-menu],[data-panel-controls],[data-resize-handle],[data-field-actions]'))return;
   const card=e.target.closest('[data-node-path]');
   if(!card)return;
   const port=e.target.closest('[data-link-port]');
   if(port&&(port.disabled||card.dataset.side!=='source'))return;
   if(!port&&e.pointerType==='touch'&&!e.target.closest('[data-drag-handle]'))return;
   drag={card,kind:port?'link':'move',x:e.clientX,y:e.clientY,id:e.pointerId,active:false};
 });
 root.addEventListener('pointermove',e=>{
   if(!drag)return;
   if(!drag.active&&Math.hypot(e.clientX-drag.x,e.clientY-drag.y)<5)return;
   if(!drag.active){
     drag.active=true;root.setPointerCapture(drag.id);
     if(drag.kind==='link') {
       const svg=document.createElementNS('http://www.w3.org/2000/svg','svg');
       svg.style.cssText='position:fixed;inset:0;width:100vw;height:100vh;z-index:600;pointer-events:none;overflow:visible';
       ghost=document.createElementNS(svg.namespaceURI,'path');
       ghost.setAttribute('fill','none');ghost.setAttribute('stroke','var(--cn-primary)');ghost.setAttribute('stroke-width','2');ghost.setAttribute('stroke-dasharray','5 4');
       svg.append(ghost);root.append(svg);drag.overlay=svg;
     } else {
     ghost=drag.card.cloneNode(true);ghost.removeAttribute('data-node-path');
     ghost.querySelectorAll('[data-field-path]').forEach(n=>n.removeAttribute('data-field-path'));
     ghost.style.cssText='position:fixed;z-index:600;pointer-events:none;width:'+drag.card.offsetWidth+'px;opacity:.85;transform:rotate(1deg)';
     root.append(ghost);
     }
   }
   e.preventDefault();
   if(drag.kind==='link') {
     const r=drag.card.querySelector('[data-link-port]').getBoundingClientRect(), x=r.x+r.width/2, y=r.y+r.height/2;
     ghost.setAttribute('d',`M${x} ${y} C${x+90} ${y},${e.clientX-90} ${e.clientY},${e.clientX} ${e.clientY}`);
   } else {ghost.style.left=(e.clientX+12)+'px';ghost.style.top=(e.clientY+12)+'px';}
   clear();
   const card=document.elementFromPoint(e.clientX,e.clientY)?.closest('[data-node-path]');
   if(card&&card.closest('[data-visual-mapper]')===root&&card!==drag.card
      &&(drag.kind==='move'?card.dataset.side===drag.card.dataset.side:card.dataset.side==='target'&&card.dataset.ignored!=='true')){
      hover=card;card.dataset.dropReady='true';
   }
 });
 root.addEventListener('pointerup',e=>{
   if(!drag)return;
   const {card,kind,active}=drag;
   if(active){
     e.preventDefault();suppress=true;
     if(kind==='link'&&hover)emit({kind,path:card.dataset.nodePath,target:hover.dataset.nodePath});
     if(kind==='move'){
       const column=document.elementFromPoint(e.clientX,e.clientY)?.closest('[data-column]');
       if(column&&column.dataset.column===card.dataset.side){
         const rows=[...column.querySelectorAll('[data-node-path]')].filter(n=>n!==card);
         const before=rows.find(n=>e.clientY<n.getBoundingClientRect().top+n.offsetHeight/2);
         emit({kind,path:card.dataset.nodePath,side:card.dataset.side,before:before?.dataset.nodePath??null});
       }
     }
     setTimeout(()=>suppress=false,0);
   }
   if(root.hasPointerCapture(e.pointerId))root.releasePointerCapture(e.pointerId);
   reset();
 });
 root.addEventListener('pointercancel',reset);
}));
"#;
