export const routes = [
    { name: "Startseite", path: '/', component: () => import('./views/HomeView.vue'), hidden: true },
    { name: "Datenimport", path: '/import', component: () => import('./views/ImportView.vue'), hidden: true },
    { name: "Datenexport", path: '/export', component: () => import('./views/ExportView.vue'), hidden: true },
    { name: "Datenbankverwaltung", path: '/add-database', component: () => import('./views/AddDatabaseView.vue'), hidden: true },
    { name: "Administrator", path: '/admin' }
]
