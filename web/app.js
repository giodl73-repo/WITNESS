import {ReplaySession} from './replay-session.js';
const $=selector=>document.querySelector(selector);
let revision=0,completed=null,playing=null,debounce=null;

function stopPlayback(){clearInterval(playing);playing=null;$('#play').textContent='Play trace';}
function clearFrame(){
 completed=null;$('#export').disabled=true;$('#raw-export').disabled=true;
 for(const id of ['events','sources','receipts'])$('#'+id).textContent='—';
 $('#validation').textContent='No current frame';$('#checkpoint').textContent='';$('#timeline').replaceChildren();
 $('#empty').hidden=false;$('#empty').textContent='No current frame. Choose a replay step or Retry engine.';
}
function controls(state){
 const enabled=['ready','projecting'].includes(state),step=Number($('#step').value);
 $('#step-value').textContent=`${step} / 7`;
 for(const id of ['step','play','share'])$('#'+id).disabled=!enabled;
 $('#back').disabled=!enabled||step===0;$('#next').disabled=!enabled||step===7;
 $('#stop-engine').disabled=['failed','stopped'].includes(state);$('#retry').hidden=!['failed','stopped'].includes(state);
}
function error(message){clearTimeout(debounce);revision++;clearFrame();stopPlayback();$('#status').textContent=message;}
const session=new ReplaySession({
 onState(state){
  controls(state);
  if(state==='initializing')$('#status').textContent='Loading Rust replay engine…';
  if(state==='projecting')$('#status').textContent='Projecting the selected frame…';
 },
 onError:error,
 onResult(frame,raw,ms,input){
  if(input.revision!==revision||input.step!==Number($('#step').value))return;
  completed=Object.freeze({frame,raw,step:input.step,revision});
  $('#status').dataset.projectionMs=String(ms);render();
 }
});
function invalidate(message){
 revision++;clearTimeout(debounce);session.invalidate();clearFrame();controls(session.state);
 if(message)$('#status').textContent=message;
 return revision;
}
function calculate(){
 const current=invalidate('Projecting the selected frame…');
 debounce=setTimeout(()=>{if(current===revision)session.submit({step:Number($('#step').value),revision:current});},60);
}
function currentFrame(){return completed&&completed.revision===revision&&completed.step===Number($('#step').value)?completed:null;}
function render(){
 const current=currentFrame();if(!current)return;
 const result=current.frame;
 $('#events').textContent=result.event_count;$('#sources').textContent=result.source_pointer_count;$('#receipts').textContent=result.receipt_count;
 $('#validation').textContent=result.validation_owed?'Validation owed':result.validation_attached?'Validation attached':'No validation yet';
 $('#checkpoint').textContent=`Checkpoint: ${result.checkpoint}. Resume: ${result.rehydrated_cut}.`;
 $('#timeline').replaceChildren();let shown=0;const filter=$('#filter').value;
 for(const event of result.events){
  if(filter==='sources'&&!['file_read','file_edit'].includes(event.kind))continue;
  if(filter==='evidence'&&!['validation','checkpoint','rehydrate'].includes(event.kind))continue;
  const li=document.createElement('li'),title=document.createElement('strong'),summary=document.createElement('span'),detail=document.createElement('small');
  title.textContent=event.kind.replaceAll('_',' ');summary.textContent=event.summary;
  detail.textContent=`Receipt: ${event.receipt}${event.source_pointer?` · Source: ${event.source_pointer}`:''}`;
  li.append(title,summary,detail);$('#timeline').append(li);shown++;
 }
 $('#empty').hidden=shown>0;$('#empty').textContent=result.step===0?'No events have occurred yet.':'No events match this filter at this step.';
 $('#export').disabled=false;$('#raw-export').disabled=false;controls(session.state);
 $('#status').textContent=`Public synthetic trace · step ${result.step} of 7 · processing stays on your device`;
}
$('#step').oninput=()=>{stopPlayback();calculate();};$('#filter').onchange=render;
$('#back').onclick=()=>{stopPlayback();$('#step').value=Math.max(0,Number($('#step').value)-1);calculate();};
$('#next').onclick=()=>{stopPlayback();$('#step').value=Math.min(7,Number($('#step').value)+1);calculate();};
$('#play').onclick=()=>{
 if(playing){stopPlayback();return;}
 if(Number($('#step').value)===7)$('#step').value=0;
 // Install first, so a synchronous engine failure can cancel playback too.
 playing=setInterval(()=>{const next=Math.min(Number($('#step').value)+1,7);$('#step').value=next;if(next===7)stopPlayback();calculate();},900);
 $('#play').textContent='Pause trace';calculate();
};
$('#stop-engine').onclick=()=>{stopPlayback();invalidate();session.stop();$('#status').textContent='Replay engine stopped. Your selected step and filter are kept. Choose Retry engine.';};
$('#retry').onclick=()=>{session.stop();calculate();};
$('#share').onclick=async()=>{
 const current=revision,step=Number($('#step').value),url=new URL(location.href);
 url.search='';url.searchParams.set('step',String(step));history.replaceState(null,'',url);
 try{await navigator.clipboard.writeText(url.href);}catch{}
 if(current===revision&&Number($('#step').value)===step&&['ready','projecting'].includes(session.state))$('#status').textContent='Replay step link is in the address bar.';
};
function save(raw,name){const url=URL.createObjectURL(new Blob([raw],{type:'application/json'}));const a=document.createElement('a');a.href=url;a.download=name;a.click();setTimeout(()=>URL.revokeObjectURL(url),1000);}
$('#export').onclick=()=>{const value=currentFrame();if(value)save(JSON.stringify(value.frame,null,2),'witness-public-frame.json');};
$('#raw-export').onclick=()=>{const value=currentFrame();if(value)save(value.raw,'witness-public-frame.raw.json');};
const params=new URLSearchParams(location.search),step=Number(params.get('step'));
if(params.has('step')&&Number.isInteger(step)&&step>=0&&step<=7)$('#step').value=String(step);
const initialRevision=invalidate('Loading Rust replay engine…');
session.submit({step:Number($('#step').value),revision:initialRevision});
