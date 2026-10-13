# Jarvis Hermes

Frontend de escritorio Tauri v2 para Hermes Agent. Para empezar: instala Node.js 20+, Rust estable y las dependencias del sistema de Tauri 2; ejecuta `npm install`, `npm run tauri dev`. Requiere que `hermes` esté instalado/configurado y accesible en PATH. El chat invoca `hermes chat -q` (modo de una consulta; el guardado del historial oficial depende de cómo Hermes registre ese modo en la versión instalada); el historial local es SQLite bajo el directorio app-data de Tauri.

## Compilar

```sh
npm install
npm run tauri dev
npm run tauri build
```

Linux (Debian/Ubuntu, paquetes de desarrollo habituales):

```sh
sudo apt install build-essential curl wget file libssl-dev libayatana-appindicator3-dev librsvg2-dev libwebkit2gtk-4.1-dev libgtk-3-dev
```

Windows: instala Visual Studio Build Tools con la carga **Desktop development with C++**, WebView2 Runtime y Rust MSVC (`rustup default stable-x86_64-pc-windows-msvc`). En PowerShell:

```powershell
npm install
npm run tauri build
```

Los instaladores/salida se generan en `src-tauri/target/release/bundle/`. Cross-compilar de Windows a Linux no es el flujo recomendado; compila en cada SO.

## Límites de seguridad

`run_local_command` solo admite `pwd` y `date`; no ejecuta shell arbitrario. `open_app` es una lista fija. No expongas herramientas arbitrarias de ejecución al frontend sin autorización explícita, aislamiento y confirmación del usuario. La UI requiere Web Speech API disponible para voz; el reconocimiento usa el motor del navegador/plataforma y puede necesitar conexión.

## Integración Hermes

El historial local está implementado; no se escriben a mano archivos internos de sesión de Hermes. Para garantía de sincronización nativa, conecta la UI con una API soportada (p. ej. proxy/API configurada) o usa las opciones de sesión/transcript expuestas por la versión de Hermes; no asumas que `hermes chat -q` persiste en el historial de sesiones. El ejemplo mantiene almacenamiento local separado hasta validar esa integración.
