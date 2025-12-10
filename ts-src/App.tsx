import { SidebarProvider, SidebarTrigger } from "@/components/ui/sidebar"
import { AppSidebar } from "@/components/app-sidebar"
import './App.css'
import { BrowserRouter as Router, Routes, Route, useLocation } from 'react-router-dom';
import { Dashboard } from "@/components/dashboard.tsx";
import { ServerList } from "@/components/server-list.tsx";
import { DeviceInfo } from "@/components/device_info.tsx";
import { Monitor } from "@/components/monitor.tsx";

const pageTitles: { [key: string]: string } = {
    '/': 'Dashboard',
    'list': 'Server List',
    '/info': 'Device Info',
    '/monitor': 'Monitor'
};

const Header = () => {
    const location = useLocation();
    const title = pageTitles[location.pathname] || 'Dashboard';

    return (
        <div className="flex fixed bg-white w-full h-12 border-b">
            <SidebarTrigger className="ml-2 my-auto" />
            <h2 className="scroll-m-20 text-xl font-semibold tracking-tight ml-5 my-auto">{title}</h2>
        </div>
    );
}

export default function App() {
    return (
        <Router>
            <SidebarProvider>
                <AppSidebar />
                <main className="w-full bg-white">
                    <div>
                        <Header />
                        <div className="mx-auto px-5 pt-14 pb-4">
                            <Routes>
                                <Route path="/" element={<Dashboard />} />
                                <Route path="/list" element={<ServerList />} />
                                <Route path="/info" element={<DeviceInfo />} />
                                <Route path="/monitor" element={<Monitor />} />
                            </Routes>
                        </div>
                    </div>
                </main>
            </SidebarProvider>
        </Router>
    )
}
