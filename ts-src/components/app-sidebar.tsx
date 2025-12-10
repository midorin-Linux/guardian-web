import { Activity, Computer, Home, List } from "lucide-react"

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

const items = [
    {
        title: "Dashboard",
        url: "/",
        icon: Home,
    },
    {
        title: "Server list",
        url: "/list",
        icon: List,
    },
    {
        title: "Device Info",
        url: "/info",
        icon: Computer
    },
    {
        title: "Monitor",
        url: "/monitor",
        icon: Activity
    }
]

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
                            {items.map((item) => (
                                <SidebarMenuItem key={item.title}>
                                    <SidebarMenuButton asChild>
                                        <a href={item.url}>
                                            <item.icon className="text-black hover:text-black" />
                                            <span className="text-black hover:text-black">{item.title}</span>
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