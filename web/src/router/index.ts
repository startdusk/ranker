import { createRouter, createWebHistory } from 'vue-router'
import { AppPage } from './page'

import Welcome from '../views/Welcome.vue'
const Create = () => import('../views/Create.vue')
const Join = () => import('../views/Join.vue')
const WaitingRoom = () => import('../views/WaitingRoom.vue')

const router = createRouter({
	history: createWebHistory(import.meta.env.BASE_URL),
	routes: [
		{
			path: '/welcome',
			name: AppPage.Welcome,
			component: Welcome
		},
		{
			path: '/create',
			name: AppPage.Create,
			component: Create
		},
		{
			path: '/join',
			name: AppPage.Join,
			component: Join
		},
		{
			path: '/waiting-room',
			name: AppPage.WaitingRoom,
			component: WaitingRoom
		},
		{
			path: '/',
			name: 'unknown',
			redirect: '/welcome'
		}
	]
})

export default router