import { Computer, ChevronRight } from "lucide-react"

import {
    Sidebar,
    SidebarContent,
    SidebarGroup,
    SidebarGroupContent,
    SidebarGroupLabel, SidebarHeader,
    SidebarMenu,
    SidebarMenuButton,
    SidebarMenuItem, SidebarMenuSub, SidebarMenuSubItem,
} from "@/components/ui/sidebar"
import {
    Collapsible,
    CollapsibleContent,
    CollapsibleTrigger,
} from "@/components/ui/collapsible"
// import { routes } from "@/lib/routes"
import { Home, List, Server } from 'lucide-react';
import {useEffect, useState} from "react";

interface ServerInfo {
    id: string,
    hostname: string,
    ip_address: string,
    os_type: string,
    tags: string | null,
    auth_profile_id: string,
    port: number,
    bastion_server_id: string | null,
    wol_mac_address: string | null,
}

function Servers() {
    const [components, setComponents] = useState<ServerInfo[] | null>(null);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);

    const fetchServerInformation = async () => {
        try {
            const response = await fetch('/api/v1/servers');
            if (!response.ok) {
                throw new Error('Failed to fetch device components');
            }
            const data: ServerInfo[] = await response.json();

            setComponents(data);
        } catch (err) {
            setError(err instanceof Error ? err.message : 'An unknown error occurred');
        } finally {
            setLoading(false);
        }
    };

    useEffect(() => {
        fetchServerInformation();
    }, []);

    if (loading) {
        return "Loading...";
    }

    if (error) {
        return `Error: ${error}`;
    }

    if (!components) {
        return "No component data available.";
    }

    return (
        components.map((info) => {
            return <SidebarMenuSubItem><a href={"/servers/" + info.id}>{info.hostname}</a></SidebarMenuSubItem>
        })
    )
}

export function AppSidebar() {
    return (
        <Sidebar>
            <SidebarHeader>
                <div className="w-full flex items-center gap-2 p-1.5">
                    <Computer className="bg-black text-white p-1.5 items-center justify-center my-auto w-8 h-8 rounded-lg" />
                    <span>未設定</span>
                </div>
            </SidebarHeader>
            <SidebarContent>
                <SidebarGroup>
                    <SidebarGroupLabel>Guardian Web</SidebarGroupLabel>
                    <SidebarGroupContent>
                        <SidebarMenu>
                            <Collapsible defaultOpen={false} className="group/collapsible">
                                {/*{routes*/}
                                {/*    .filter((route) => route.showInSidebar)*/}
                                {/*    .map((route) => (*/}
                                {/*        <SidebarMenuItem key={route.path}>*/}
                                {/*            <SidebarMenuButton asChild>*/}
                                {/*                <a href={route.path}>*/}
                                {/*                    <route.icon className="text-black hover:text-black" />*/}
                                {/*                    <span className="text-black hover:text-black">{route.title}</span>*/}
                                {/*                </a>*/}
                                {/*            </SidebarMenuButton>*/}
                                {/*        </SidebarMenuItem>*/}
                                {/*    ))}*/}
                                <SidebarMenuItem>
                                    <SidebarMenuButton asChild>
                                        <a href="/">
                                            <Home className="text-black hover:text-black" />
                                            <span className="text-black hover:text-black">Dashboard</span>
                                        </a>
                                    </SidebarMenuButton>
                                </SidebarMenuItem>
                                <SidebarMenuItem>
                                    <SidebarMenuButton asChild>
                                        <a href="/list">
                                            <List className="text-black hover:text-black" />
                                            <span className="text-black hover:text-black">Server List</span>
                                        </a>
                                    </SidebarMenuButton>
                                </SidebarMenuItem>
                                <SidebarMenuItem>
                                    <CollapsibleTrigger asChild>
                                        <SidebarMenuButton>
                                            <Server className="text-black hover:text-black" />
                                            <span className="text-black hover:text-black">Servers</span>
                                            <ChevronRight className="ml-auto transition-transform group-data-[state=open]/collapsible:rotate-90" />
                                        </SidebarMenuButton>
                                    </CollapsibleTrigger>
                                    <CollapsibleContent>
                                        <SidebarMenuSub>
                                            <Servers />
                                        </SidebarMenuSub>
                                    </CollapsibleContent>
                                </SidebarMenuItem>
                            </Collapsible>
                        </SidebarMenu>
                    </SidebarGroupContent>
                </SidebarGroup>
            </SidebarContent>
        </Sidebar>
    )
}