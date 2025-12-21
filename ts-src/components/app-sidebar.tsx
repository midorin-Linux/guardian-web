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
                                            {/*ToDo: データを取得して表示する*/}
                                            <SidebarMenuSubItem>192.168.100.2</SidebarMenuSubItem>
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