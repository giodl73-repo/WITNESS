import init,{frame_json} from './pkg/witness_web.js';
let generation;
self.onmessage=async({data})=>{
 if(data.type==='init'){
  generation=data.generation;
  try{await init();self.postMessage({type:'ready',generation});}
  catch{self.postMessage({type:'error',generation,message:'Replay engine could not load.'});}
  return;
 }
 if(data.type!=='frame'||data.generation!==generation)return;
 try{
  const started=performance.now();const raw=frame_json(data.step);
  self.postMessage({type:'frame',id:data.id,generation,raw,ms:performance.now()-started});
 }catch(error){self.postMessage({type:'error',id:data.id,generation,message:String(error),fatal:typeof error!=='string'});}
};
