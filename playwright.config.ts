import {defineConfig} from '@playwright/test';
export default defineConfig({testDir:'web/e2e',workers:1,use:{channel:'chrome',baseURL:'http://127.0.0.1:1420',viewport:{width:1440,height:1000},trace:'retain-on-failure'},reporter:'list',timeout:60000});
