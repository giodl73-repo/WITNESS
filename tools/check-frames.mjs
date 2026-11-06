// Execute the actual release module; formatting never replaces raw Rust frames.
import fs from 'node:fs/promises';
import path from 'node:path';
import {pathToFileURL} from 'node:url';
import {createHash} from 'node:crypto';

const [site,controlFile,outputFile,abiFile]=process.argv.slice(2);
if(!site||!controlFile||!outputFile||!abiFile)throw Error('Use SITE CONTROLS OUTPUT ABI');
let runtime=site;
try{
 const manifest=JSON.parse(await fs.readFile(path.join(site,'BUILD-MANIFEST.json'),'utf8'));
 if(!/^assets\/[a-f0-9]{40}$/.test(manifest.runtime))throw Error('Invalid runtime revision');
 runtime=path.join(site,manifest.runtime);
}catch(error){if(error.code!=='ENOENT')throw error;}
const bytes=await fs.readFile(path.join(runtime,'pkg/witness_web_bg.wasm'));
const {default:init,frame_json}=await import(pathToFileURL(path.resolve(runtime,'pkg/witness_web.js')).href);
await init({module_or_path:bytes});
const input=await fs.readFile(controlFile,'utf8');
if(!input.endsWith('\n'))throw Error('Incomplete control stream');
const steps=input.slice(0,-1).split('\n').map(JSON.parse);
function execute(step){try{return JSON.stringify({Ok:frame_json(step)});}catch(error){return JSON.stringify({Err:String(error)});}}
const output=await fs.open(outputFile,'wx');
try{for(const step of steps)await output.write(execute(step)+'\n');}finally{await output.close();}
const abi=[-1,1.9,4294967296,NaN,Infinity].map(step=>({raw_number:String(step),wasm_u32_transport:step>>>0,raw_result:execute(step)}));
await fs.writeFile(abiFile,JSON.stringify(abi,null,2)+'\n',{flag:'wx'});
console.log(JSON.stringify({controls:steps.length,raw_abi_observations:abi.length,wasm_bytes:bytes.length,wasm_sha256:createHash('sha256').update(bytes).digest('hex')}));
