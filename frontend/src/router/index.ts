import { Router, RouteRecordRaw, createRouter, createWebHashHistory } from 'vue-router';

import HomepageView from '../components/HomepageView.vue';
import TableView from '../components/TableView.vue';

const routes: RouteRecordRaw[] = [
    {
        path: '/',
        component: HomepageView
    },
    {
        path: '/:id',
        component: TableView,
    }
];


const router: Router = createRouter({
    history: createWebHashHistory(),
    routes
})

export default router;
