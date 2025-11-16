import { Cpu, Gpu, HardDrive, Info, MemoryStick } from 'lucide-react'

import { useState, useEffect, type ReactNode } from 'react';

interface DeviceInfo {
    hostname: string;
    os: string;
    kernel_version: string;
}

interface CpuInfo {
    name: string;
    base_freq_ghz: number;
    cores: number;
    threads: number;
}

interface RamInfo {
    capacity_gb: number;
    speed_mhz: number;
}

interface StorageDevice {
    model: string;
    capacity_gb: number;
    type: string;
}

interface GpuInfo {
    name: string;
    vram_gb: number;
}

interface DeviceComponents {
    device: DeviceInfo;
    cpu: CpuInfo;
    ram: RamInfo;
    storage: StorageDevice[];
    gpu: GpuInfo;
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
    // 2. 状態管理（データ、ローディング、エラー）
    const [components, setComponents] = useState<DeviceComponents | null>(null);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);

    // 3. データ取得処理
    useEffect(() => {
        const fetchDeviceComponents = async () => {
            try {
                const response = await fetch('/api/device-components');
                if (!response.ok) {
                    throw new Error('Failed to fetch device components');
                }
                const data: DeviceComponents = await response.json();

                // const data: DeviceComponents = {
                //     "device": {
                //         "hostname": "DESKTOP",
                //         "os": "Windows 11 Pro",
                //         "kernel_version": "28000"
                //     },
                //     "cpu": {
                //         "name": "Intel 114514",
                //         "base_freq_ghz": 2.5,
                //         "boost_freq_ghz": 3.1,
                //         "cores": 4,
                //         "threads": 8
                //     },
                //     "ram": {
                //         "capacity_gb": 16,
                //         "type": "DDR4",
                //         "speed_mhz": 3200
                //     },
                //     "storage": [
                //         { "model": "sandisk", "capacity_gb": 256, "type": "HDD" },
                //         { "model": "western digital", "capacity_gb": 512, "type": "SSD" }
                //     ],
                //     "gpu": {
                //         "name": "NVIDIA RTX PRO 6000 Blackwell Workstation Edition",
                //         "vram_gb": 96
                //     }
                // };

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
                            <p>{components.cpu.base_freq_ghz}GHz</p>
                            <p>{components.cpu.cores} Cores / {components.cpu.threads} Threads</p>
                        </div>
                    </ComponentCard>
                    <ComponentCard icon={<MemoryStick />} title="RAM">
                        <p className="text-lg font-semibold scroll-m-20 mt-2">{components.ram.capacity_gb}GB</p>
                        <div className="mt-1">
                            <p>{components.ram.speed_mhz}MHz</p>
                        </div>
                    </ComponentCard>
                    <ComponentCard icon={<HardDrive />} title="Storage">
                        <p className="text-lg font-semibold scroll-m-20 mt-2">
                            {components.storage.reduce((acc, dev) => acc + dev.capacity_gb, 0)}GB
                        </p>
                        <div className="mt-1">
                            {components.storage.map((disk, index) => (
                                <p key={index}>{disk.model} ({disk.capacity_gb}GB)</p>
                            ))}
                        </div>
                    </ComponentCard>
                    <ComponentCard icon={<Gpu />} title="GPU">
                        <p className="text-lg font-semibold scroll-m-20 mt-2" title={components.gpu.name}>{components.gpu.name}</p>
                        <div className="mt-1">
                            <p>{components.gpu.vram_gb}GB</p>
                        </div>
                    </ComponentCard>
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
                                <div className="flex"><span className="w-32">RAM</span><span className="font-medium">{components.ram.capacity_gb}GB</span></div>
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
                                <div className="flex"><span className="w-32">Kernel Version</span><span className="font-medium">{components.device.kernel_version}</span></div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </>
    );
}
