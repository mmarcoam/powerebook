import { defineConfig } from 'vite';

export default defineConfig({
  // Diz ao Vite que os arquivos do site estão na pasta 'src'
  root: 'src',
  server: {
    watch: {
      // MANDA O VITE IGNORAR A PASTA DE COMPILAÇÃO DO RUST (EVITA O ERRO EBUSY)
      ignored: ['**/src-tauri/target/**'],
    },
  },
});