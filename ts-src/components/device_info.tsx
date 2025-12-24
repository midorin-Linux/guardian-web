import { Cpu, Gpu, HardDrive, Info, MemoryStick } from 'lucide-react'

import { useState, useEffect, type ReactNode } from 'react';
import { useParams } from "react-router-dom";

interface DeviceInfo {
    hostname: string;
    kernel: string;
    os: string;
}

interface CpuInfo {
    base_freq_mhz: number;
    cores: number;
    name: string;
    threads: number;
}

interface RamInfo {
    total_bytes: number;
}

interface StorageDevice {
    device: string;
    mount: string;
    total_bytes: number;
}

interface GpuInfo {
    driver_version: string;
    name: string;
    video_ram_mb: number;
}

interface DeviceComponents {
    device: DeviceInfo;
    cpu: CpuInfo;
    memory: RamInfo;
    disk: StorageDevice[];
    gpu: GpuInfo[];
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

export function DeviceInfo() {
    const { serverId } = useParams<{ serverId: string }>();

    const [components, setComponents] = useState<DeviceComponents | null>(null);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);

    useEffect(() => {
        const fetchDeviceComponents = async () => {
            try {
                const response = await fetch('/api/v1/servers/' + serverId + '/specs');
                if (!response.ok) {
                    throw new Error('Failed to fetch device components');
                }
                const data: DeviceComponents = await response.json();

                setComponents(data);
            } catch (err) {
                setError(err instanceof Error ? err.message : 'An unknown error occurred');
            } finally {
                setLoading(false);
            }
        };

        fetchDeviceComponents();
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

    return (
        <>
            <div className="scroll-m-20">
                <div className="grid grid-cols-subgrid md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4 mt-2 mb-4">
                    <ComponentCard icon={<Cpu />} title="CPU">
                        <p className="text-lg font-semibold scroll-m-20 mt-2">{components.cpu.name}</p>
                        <div className="mt-1">
                            <p>{components.cpu.base_freq_mhz / 1000}GHz</p>
                            <p>{components.cpu.cores} Cores / {components.cpu.threads} Threads</p>
                        </div>
                    </ComponentCard>
                    <ComponentCard icon={<MemoryStick />} title="RAM">
                        <p className="text-lg font-semibold scroll-m-20 mt-2">{components.memory.total_bytes / 1024 / 1024 / 1024}GB</p>
                    </ComponentCard>
                    <ComponentCard icon={<HardDrive />} title="Storage">
                        <p className="text-lg font-semibold scroll-m-20 mt-2">
                            {components.disk.reduce((acc, dev) => acc + dev.device, '')}
                        </p>
                        <div className="mt-1">
                            {components.disk.map((disk, index) => (
                                <p key={index}>{disk.mount} ({disk.total_bytes / 1024 / 1024 / 1024}GB)</p>
                            ))}
                        </div>
                    </ComponentCard>
                    {components.gpu.map((gpu) => (
                        <ComponentCard icon={<Gpu />} title="GPU">
                            <p className="text-lg font-semibold scroll-m-20 mt-2" title={gpu.name}>{gpu.name}</p>
                            <div className="mt-1">
                                <p>{gpu.video_ram_mb / 1024 / 1024}GB</p>
                            </div>
                        </ComponentCard>
                    ))}
                </div>
                <div className="rounded border bg-white pl-4 py-2 text-sm mt-5">
                    <p>{components.device.hostname}</p>
                </div>
                <div>
                    <div className="flex items-center gap-2 rounded-tr rounded-tl border bg-white p-4 mt-2">
                        <Info className="w-5 h-5" />
                        <span className="scroll-m-20 text-base racking-tight">Device Information</span>
                    </div>
                    <div className="rounded-br rounded-bl border-l border-b border-r bg-white px-4 py-2 text-sm">
                        <div className="flex items-center bg-white p-2">
                            <div className="flex flex-col">
                                <div className="flex"><span className="w-32">Device Name</span><span className="font-medium">{components.device.hostname}</span></div>
                                <div className="flex"><span className="w-32">Processor</span><span className="font-medium">{components.cpu.name}</span></div>
                                <div className="flex"><span className="w-32">RAM</span><span className="font-medium">{components.memory.total_bytes / 1024 / 1024 / 1024}GB</span></div>
                            </div>
                        </div>
                    </div>
                </div>
                <div>
                    <div className="flex items-center gap-2 rounded-tr rounded-tl border bg-white p-4 mt-2">
                        <Info className="w-5 h-5" />
                        <span className="scroll-m-20 text-base racking-tight">
                            System Information
                        </span>
                    </div>
                    <div className="rounded-br rounded-bl border-l border-b border-r bg-white px-4 py-2 text-sm">
                        <div className="flex items-center bg-white p-2">
                            <div className="flex flex-col">
                                <div className="flex"><span className="w-32">OS</span><span className="font-medium">{components.device.os}</span></div>
                                <div className="flex"><span className="w-32">Kernel Version</span><span className="font-medium">{components.device.kernel}</span></div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </>
    );
}
