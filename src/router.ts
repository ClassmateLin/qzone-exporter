import { createRouter, createWebHashHistory, RouterOptions, Router, RouteRecordRaw } from 'vue-router'

const routes: RouteRecordRaw[] = [
    {
        path: '/',
        name: 'Home',
        component: () => import("./views/Home.vue")
    },
    { 
        path: '/layout', 
        name: 'Layout', 
        component: () => import("./views/Layout.vue"), 
        meta: {
            keepAlive: true,
        },
        children: [
            { 
                path: '/task', 
                name: 'Task', 
                component: () => import("./views/Task.vue"),
                meta: {
                    keepAlive: true,
                }
            },
            { 
                path: '/photo', 
                name: 'Photo', 
                component: () => import("./views/Photo.vue"),
                meta: {
                    keepAlive: true,
                }
            },
        ]
    },
    
    {
        path: "/login",
        name: "Login",
        component: () => import("./views/Login.vue"),
    }
]

// RouterOptions是路由选项类型
const options: RouterOptions = {
 history: createWebHashHistory(),
 routes,
}

// Router是路由对象类型
const router: Router = createRouter(options)

export default router