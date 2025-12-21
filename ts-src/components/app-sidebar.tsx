import { Computer } from "lucide-react"

import {
    Sidebar,
    SidebarContent,
    SidebarGroup,
    SidebarGroupContent,
    SidebarGroupLabel, SidebarHeader,
    SidebarMenu,
    SidebarMenuButton,
    SidebarMenuItem,
} from "@/components/ui/sidebar"
import { routes } from "@/lib/routes"

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
                    <SidebarGroupLabel>Management Website</SidebarGroupLabel>
                    <SidebarGroupContent>
                        <SidebarMenu>
                            {routes
                                .filter((route) => route.showInSidebar)
                                .map((route) => (
                                    <SidebarMenuItem key={route.path}>
                                        <SidebarMenuButton asChild>
                                            <a href={route.path}>
                                                <route.icon className="text-black hover:text-black" />
                                                <span className="text-black hover:text-black">{route.title}</span>
                                            </a>
                                        </SidebarMenuButton>
                                    </SidebarMenuItem>
                                ))}
                        </SidebarMenu>
                    </SidebarGroupContent>
                </SidebarGroup>
            </SidebarContent>
        </Sidebar>
    )
}