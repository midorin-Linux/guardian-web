import {
    type ColumnDef,
    flexRender,
    getCoreRowModel,
    useReactTable,
} from "@tanstack/react-table"
import {
    Table,
    TableBody,
    TableCell,
    TableHead,
    TableHeader,
    TableRow,
} from "@/components/ui/table"
import {
    Dialog,
    DialogClose,
    DialogContent,
    DialogDescription,
    DialogFooter,
    DialogHeader,
    DialogTitle,
    DialogTrigger,
} from "@/components/ui/dialog"
import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"

import { useState, useEffect } from 'react';

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

interface DataTableProps<TData, TValue> {
    columns: ColumnDef<TData, TValue>[]
    data: TData[]
}

export const columns: ColumnDef<ServerInfo>[] = [
    {
        accessorKey: "hostname",
        header: "Hostname",
    },
    {
        accessorKey: "ip_address",
        header: "IP address",
    },
    {
        accessorKey: "id",
        header: "UUID",
    },
]

export function DataTable<TData, TValue>({columns, data,}: DataTableProps<TData, TValue>) {
    const table = useReactTable({
        data,
        columns,
        getCoreRowModel: getCoreRowModel(),
    })

    return (
        <div className="overflow-hidden rounded-md border">
            <Table>
                <TableHeader>
                    {table.getHeaderGroups().map((headerGroup) => (
                        <TableRow key={headerGroup.id}>
                            {headerGroup.headers.map((header) => {
                                return (
                                    <TableHead key={header.id}>
                                        {header.isPlaceholder
                                            ? null
                                            : flexRender(
                                                header.column.columnDef.header,
                                                header.getContext()
                                            )}
                                    </TableHead>
                                )
                            })}
                        </TableRow>
                    ))}
                </TableHeader>
                <TableBody>
                    {table.getRowModel().rows?.length ? (
                        table.getRowModel().rows.map((row) => (
                            <TableRow
                                key={row.id}
                                data-state={row.getIsSelected() && "selected"}
                            >
                                {row.getVisibleCells().map((cell) => (
                                    <TableCell key={cell.id}>
                                        {flexRender(cell.column.columnDef.cell, cell.getContext())}
                                    </TableCell>
                                ))}
                            </TableRow>
                        ))
                    ) : (
                        <TableRow>
                            <TableCell colSpan={columns.length} className="h-24 text-center">
                                No results.
                            </TableCell>
                        </TableRow>
                    )}
                </TableBody>
            </Table>
        </div>
    )
}

export function ServerList() {
    // 2. 状態管理（データ、ローディング、エラー）
    const [components, setComponents] = useState<ServerInfo[] | null>(null);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);
    const [open, setOpen] = useState(false);

    // 3. データ取得処理
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

    const handleSubmit = async (event: React.FormEvent<HTMLFormElement>) => {
        event.preventDefault();
        const formData = new FormData(event.currentTarget);
        const data = Object.fromEntries(formData.entries());

        // ポート番号を数値に変換
        const payload = {
            ...data,
            port: Number(data.port),
        };

        try {
            const response = await fetch('/api/v1/servers', {
                method: 'POST',
                headers: { 'Content-Type': 'application/json' },
                body: JSON.stringify(payload),
            });

            if (response.ok) {
                await fetchServerInformation(); // リストを更新
                setOpen(false); // ダイアログを閉じる
            }
        } catch (error) {
            console.error('Failed to register server:', error);
        }
    };

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
            <DataTable columns={columns} data={components} />
            <div className="mt-2 flex justify-end">
                <Dialog open={open} onOpenChange={setOpen}>
                    <DialogTrigger asChild>
                        <Button variant="outline">Register server</Button>
                    </DialogTrigger>
                    <DialogContent className="sm:max-w-[425px]">
                        <form onSubmit={handleSubmit}>
                            <DialogHeader>
                                <DialogTitle>Register server</DialogTitle>
                                <DialogDescription>
                                    Please enter the hostname, IP address, and port of ssh to register the server.
                                </DialogDescription>
                            </DialogHeader>
                            <div className="grid gap-4 py-4">
                                <div className="grid gap-3">
                                    <Label htmlFor="name-1">Hostname</Label>
                                    <Input id="name-1" name="hostname" required />
                                </div>
                                <div className="grid gap-3">
                                    <Label htmlFor="ip-address">IP address</Label>
                                    <Input id="ip-address" name="ip_address" required />
                                </div>
                                <div className="grid gap-3">
                                    <Label htmlFor="port">SSH port</Label>
                                    <Input id="port" name="port" type="number" defaultValue="22" required />
                                </div>
                            </div>
                            <DialogFooter>
                                <DialogClose asChild>
                                    <Button type="button" variant="outline">Cancel</Button>
                                </DialogClose>
                                <Button type="submit">Save changes</Button>
                            </DialogFooter>
                        </form>
                    </DialogContent>
                </Dialog>
            </div>
        </>
    );
}
