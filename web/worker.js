import init,{frame_json} from './pkg/witness_web.js';
self.onmessage=async({data})=>{try{if(data.type==='init'){await init();self.postMessage({type:'ready'});}else{self.postMessage({type:'result',id:data.id,result:JSON.parse(frame_json(data.step))});}}catch(e){self.postMessage({type:'error',id:data.id,message:String(e)});}};
