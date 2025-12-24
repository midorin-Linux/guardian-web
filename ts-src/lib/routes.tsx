import type { ComponentType } from 'react';
import {  type LucideIcon, Home, List, Server } from 'lucide-react';
import { Dashboard } from '@/components/dashboard';
import { ServerList } from '@/components/server-list';
import { DeviceInfo } from '@/components/device_info';
// import { Monitor } from '@/components/monitor';

export interface RouteConfig {
    path: string;
    element: ComponentType;
    title: string;
    icon: LucideIcon;
    showInSidebar: boolean;
}

export const routes: RouteConfig[] = [
    {
        path: '/',
        element: Dashboard,
        title: 'Dashboard',
        icon: Home,
        showInSidebar: true,
    },
    {
        path: '/list',
        element: ServerList,
        title: 'Server List',
        icon: List,
        showInSidebar: true,
    },
    {
        path: '/servers',
        element: ServerList,
        title: 'Servers',
        icon: Server,
        showInSidebar: true,
    },
    {
        path: '/servers/:serverId',
        element: DeviceInfo,
        title: 'Server Details',
        icon: Server,
        showInSidebar: false,
    },
    // {
    //     path: '/info',
    //     element: DeviceInfo,
    //     title: 'Device Info',
    //     icon: Computer,
    //     showInSidebar: true,
    // },
    // {
    //     path: '/monitor',
    //     element: Monitor,
    //     title: 'Monitor',
    //     icon: Activity,
    //     showInSidebar: true,
    // },
];
