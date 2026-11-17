//! Delegated pointer gestures survive node updates. Layout changes are emitted to
//! Rust through a scoped input; link semantics never depend on DOM coordinates.
pub(super) const SCRIPT: &str = r#"
requestAnimationFrame(()=>document.querySelectorAll('[data-visual-mapper]:not([data-editor-ready])').forEach(root=>{
 root.dataset.editorReady='true';
 let drag=null, ghost=null, hover=null, suppress=false;
 const clear=()=>{if(hover)delete hover.dataset.dropReady;hover=null;};
 const reset=()=>{clear();ghost?.remove();ghost=null;drag=null;};
 const emit=value=>{
   const input=root.querySelector('[data-editor-events]');
   Object.getOwnPropertyDescriptor(HTMLInputElement.prototype,'value').set.call(input,JSON.stringify(value));
   input.dispatchEvent(new Event('input',{bubbles:true}));
 };
 root.addEventListener('dragstart',e=>e.preventDefault());
 root.addEventListener('click',e=>{if(suppress){e.preventDefault();e.stopImmediatePropagation();suppress=false;}},true);
 root.addEventListener('pointerdown',e=>{
   if(e.button!==0||root.dataset.readOnly==='true')return;
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
     ghost=drag.card.cloneNode(true);ghost.removeAttribute('data-node-path');
     ghost.querySelectorAll('[data-field-path]').forEach(n=>n.removeAttribute('data-field-path'));
     ghost.style.cssText='position:fixed;z-index:600;pointer-events:none;width:'+drag.card.offsetWidth+'px;opacity:.85;transform:rotate(1deg)';
     root.append(ghost);
   }
   e.preventDefault();
   ghost.style.left=(e.clientX+12)+'px';ghost.style.top=(e.clientY+12)+'px';
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
