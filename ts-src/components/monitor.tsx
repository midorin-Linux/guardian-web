import { useSSE } from "@/hooks/use-sse.ts";
import { LineChart, Line, XAxis, YAxis, ResponsiveContainer, CartesianGrid, Tooltip } from 'recharts';
import { useState, useEffect } from 'react';

interface MonitorData {
    cpu: number;
    ram: number;
}

interface CpuChartData {
    time: string;
    cpu: number;
}

interface RamChartData {
    time: string;
    ram: number;
}

const MAX_DATA_POINTS = 20;

const createInitialCpuData = (): CpuChartData[] =>
    Array.from({ length: MAX_DATA_POINTS }, () => ({ time: '', cpu: 0 }));

const createInitialRamData = (): RamChartData[] =>
    Array.from({ length: MAX_DATA_POINTS }, () => ({ time: '', ram: 0 }));

export function Monitor() {
    const sse = useSSE<string>("/api/monitor/stream");
    const [cpuChartData, setCpuChartData] = useState<CpuChartData[]>(createInitialCpuData);
    const [ramChartData, setRamChartData] = useState<RamChartData[]>(createInitialRamData);

    useEffect(() => {
        if (sse.data) {
            try {
                let monitorData: MonitorData;

                if (typeof sse.data === 'object') {
                    monitorData = sse.data as MonitorData;
                } else if (typeof sse.data === 'string') {
                    const trimmedData = sse.data.trim();
                    if (!trimmedData) return;

                    monitorData = JSON.parse(trimmedData);
                } else {
                    console.error('Unexpected data type:', typeof sse.data);
                    return;
                }

                const now = new Date();
                const timeStr = `${now.getHours().toString().padStart(2, '0')}:${now.getMinutes().toString().padStart(2, '0')}:${now.getSeconds().toString().padStart(2, '0')}`;

                if (typeof monitorData.cpu === 'number' && !isNaN(monitorData.cpu)) {
                    const newCpuEntry: CpuChartData = {
                        time: timeStr,
                        cpu: monitorData.cpu,
                    };
                    setCpuChartData(prevData => [...prevData.slice(1), newCpuEntry]);
                }

                if (typeof monitorData.ram === 'number' && !isNaN(monitorData.ram)) {
                    const newRamEntry: RamChartData = {
                        time: timeStr,
                        ram: monitorData.ram,
                    };
                    setRamChartData(prevData => [...prevData.slice(1), newRamEntry]);
                }
            } catch (error) {
                console.error('Failed to parse monitor data:', error);
                console.error('Raw data was:', sse.data);
            }
        }
    }, [sse.data]);

    return (
        <div className="space-y-8">
            <div className="w-full h-96">
                <h3 className="scroll-m-20 text-xl font-semibold tracking-tight mb-4">CPU Monitor</h3>
                <ResponsiveContainer width="100%" height="100%">
                    <LineChart data={cpuChartData} margin={{ top: 5, right: 20, left: 0, bottom: 5 }}>
                        <CartesianGrid strokeDasharray="3 3" />
                        <XAxis dataKey="time" />
                        <YAxis domain={[0, 100]} label={{ value: 'Usage (%)', angle: -90, position: 'insideLeft' }} />
                        <Tooltip />
                        <Line type="monotone" dataKey="cpu" stroke="#8884d8" isAnimationActive={false} dot={false} />
                    </LineChart>
                </ResponsiveContainer>
            </div>

            <div className="w-full h-96">
                <h3 className="scroll-m-20 text-xl font-semibold tracking-tight mb-4">RAM Monitor</h3>
                <ResponsiveContainer width="100%" height="100%">
                    <LineChart data={ramChartData} margin={{ top: 5, right: 20, left: 0, bottom: 5 }}>
                        <CartesianGrid strokeDasharray="3 3" />
                        <XAxis dataKey="time" />
                        <YAxis domain={[0, 100]} label={{ value: 'Usage (%)', angle: -90, position: 'insideLeft' }} />
                        <Tooltip />
                        <Line type="monotone" dataKey="ram" stroke="#82ca9d" isAnimationActive={false} dot={false} />
                    </LineChart>
                </ResponsiveContainer>
            </div>
        </div>
    );
}