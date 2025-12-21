import { SidebarProvider, SidebarTrigger } from "@/components/ui/sidebar"
import { AppSidebar } from "@/components/app-sidebar"
import './App.css'
import { BrowserRouter as Router, Routes, Route, useLocation } from 'react-router-dom';
import { routes } from '@/lib/routes';

const Header = () => {
    const location = useLocation();
    const currentRoute = routes.find(route => route.path === location.pathname);
    const title = currentRoute?.title || 'Dashboard';

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
                                {routes.map((route) => (
                                    <Route
                                        key={route.path}
                                        path={route.path}
                                        element={<route.element />}
                                    />
                                ))}
                            </Routes>
                        </div>
                    </div>
                </main>
            </SidebarProvider>
        </Router>
    )
}
