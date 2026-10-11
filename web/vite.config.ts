import {defineConfig} from 'vitest/config';
import react from '@vitejs/plugin-react';
export default defineConfig(({mode})=>({publicDir:mode==='native'?false:'public',root: 'web', plugins:[react()], server:{host:'127.0.0.1',port:1420,strictPort:true,proxy:{'/api':'http://127.0.0.1:4318'}}, test:{environment:'jsdom',include:['src/**/*.test.ts*']},build:{target:'es2022'}}));
