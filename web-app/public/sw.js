self.addEventListener('install', (event) => {
  console.log('Service worker installed.');
  self.skipWaiting();
});

self.addEventListener('activate', (event) => {
  console.log('Service worker activated.');
});

self.addEventListener('fetch', (event) => {
  // Pass through fetch requests for now.
  event.respondWith(fetch(event.request));
});
