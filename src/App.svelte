<script lang="ts">
 import { invoke } from '@tauri-apps/api/core';
 type Entry = { role:string; content:string; created_at?:string };
 let items: Entry[] = []; let input = ''; let busy = false; let listening = false; let err = '';
 async function load() { try { items = await invoke<Entry[]>('history'); items.reverse(); } catch(e) { err=String(e); } }
 async function send() { const text=input.trim(); if(!text||busy)return; input=''; busy=true; err=''; items=[...items,{role:'user',content:text}];
  try { await invoke('save_message',{role:'user',content:text}); const reply=await invoke<string>('ask_hermes',{prompt:text}); await invoke('save_message',{role:'assistant',content:reply}); items=[...items,{role:'assistant',content:reply}]; }
  catch(e) { err=String(e); } finally { busy=false; }
 }
 function voice() { const SR=(window as any).SpeechRecognition||(window as any).webkitSpeechRecognition; if(!SR){err='Reconocimiento de voz no disponible en este navegador.';return;} const rec=new SR(); rec.lang='es-ES';rec.interimResults=false;rec.onresult=(e:any)=>{input=e.results[0][0].transcript;listening=false;};rec.onerror=()=>{listening=false;err='No se pudo reconocer la voz.'};rec.onend=()=>listening=false;listening=true;rec.start(); }
 async function action(which:string){try{await invoke('open_app',{target:which})}catch(e){err=String(e)}}
 load();
</script>
<svelte:head><title>Jarvis · Hermes Agent</title></svelte:head>
<div class="shell">
 <header><div class="brand"><span class="mark">H</span><div><b>HERMES</b><small>PERSONAL INTELLIGENCE</small></div></div><span class="status"><i></i> {busy?'PROCESANDO':'SISTEMA EN LÍNEA'}</span></header>
 <main><div class="intro"><span class="eyebrow">INTERFAZ DE AGENTE</span><h1>¿En qué puedo<br/><em>ayudarte?</em></h1><p>Tu asistente local, conectado a Hermes Agent.</p></div>
 <section class="conversation">{#each items as item}<article class:human={item.role==='user'}><span>{item.role==='user'?'TÚ':'HERMES'}</span><p>{item.content}</p></article>{/each}{#if busy}<article><span>HERMES</span><p class="pulse">Pensando…</p></article>{/if}</section>
 {#if err}<div class="error">{err}</div>{/if}
 <form on:submit|preventDefault={send}><textarea bind:value={input} placeholder="Escribe un mensaje…" rows="1" on:keydown={(e)=>{if(e.key==='Enter'&&!e.shiftKey){e.preventDefault();send()}}}></textarea><button type="button" class:active={listening} class="mic" aria-label="Dictar" on:click={voice}>◉</button><button class="send" disabled={busy||!input.trim()} aria-label="Enviar">↑</button></form>
 <div class="tools"><span>ACCIONES RÁPIDAS</span><button on:click={()=>action('browser')}>↗ Abrir navegador</button><button on:click={()=>action('terminal')}>⌘ Terminal</button></div>
 </main><footer>PRIVADO POR DISEÑO <span>·</span> HISTORIAL LOCAL CIFRADO POR SISTEMA</footer>
</div>
