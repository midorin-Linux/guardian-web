import { Server, ListCheck } from 'lucide-react'

import { useState, useEffect, type ReactNode } from 'react';

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
    is_online?: boolean;
}

interface ComponentCardProps {
    icon: ReactNode;
    title: string;
    children: ReactNode;
}

function ComponentCard({ icon, title, children }: ComponentCardProps) {
    return (
        <div className="rounded border bg-white p-4 text-sm">
            <div className="flex items-center gap-2 mb-0 border-b pb-2">
                {icon}
                <span className="scroll-m-20 text-xl font-semibold tracking-tight">
                    {title}
                </span>
            </div>
            <div>
                {children}
            </div>
        </div>
    );
}

export function Dashboard() {
    const [components, setComponents] = useState<ServerInfo[] | null>(null);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        const fetchAndCheckHealth = async () => {
            try {
                const response = await fetch('/api/v1/servers');
                if (!response.ok) throw new Error('Failed to fetch device components');
                const data: ServerInfo[] = await response.json();

                const healthChecks = data.map(async (server) => {
                    try {
                        const res = await fetch(`/api/v1/servers/${server.id}/health`);
                        return { ...server, is_online: res.ok };
                    } catch {
                        return { ...server, is_online: false };
                    }
                });

                const results = await Promise.all(healthChecks);
                setComponents(results);
            } catch (err) {
                setError(err instanceof Error ? err.message : 'An unknown error occurred');
            } finally {
                setLoading(false);
            }
        };

        fetchAndCheckHealth();
    }, []);

    if (loading) {
        return <div>Loading...</div>;
    }

    if (error) {
        return <div>Error: {error}</div>;
    }

    if (!components) {
        return <div>No component data available.</div>;
    }

    const onlineCount = components.filter(c => c.is_online).length;
    const uptimePercentage = components.length > 0
        ? Math.round((onlineCount / components.length) * 100)
        : 0;

    return (
        <>
            <div className="scroll-m-20">
                <ComponentCard icon={<ListCheck />} title={"稼働率"}>
                    <p className="text-lg font-semibold scroll-m-20 mt-2">{uptimePercentage}%</p>
                    <div className="mt-1">
                        <p>{onlineCount}/{components.length} 台が稼働中です</p>
                    </div>
                </ComponentCard>
                <hr className="my-2" />
                <div className="grid grid-cols-subgrid md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
                    {components.map((info) => (
                        <ComponentCard icon={<Server />} title={info.hostname}>
                            <p className="text-lg font-semibold scroll-m-20 mt-2">{info.ip_address}</p>
                            <div className="mt-1">
                                <p>UUID: {info.id}</p>
                            </div>
                        </ComponentCard>
                    ))}
                </div>
            </div>
        </>
    );
}
