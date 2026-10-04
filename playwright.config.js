import {defineConfig} from '@playwright/test';
export default defineConfig({
 testDir:'./tests/browser',timeout:60000,
 use:{baseURL:'http://127.0.0.1:8770',browserName:'chromium',launchOptions:process.env.WITNESS_BROWSER_PATH?{executablePath:process.env.WITNESS_BROWSER_PATH}:{}},
 webServer:{command:'python -m http.server 8770 --bind 127.0.0.1 --directory dist',url:'http://127.0.0.1:8770/WITNESS/',reuseExistingServer:!process.env.CI},
});
