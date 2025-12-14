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

    // 3. データ取得処理
    useEffect(() => {
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

        fetchServerInformation();
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
            <DataTable columns={columns} data={components} />
            <Dialog>
                <form>
                    <DialogTrigger asChild>
                        <Button variant="outline">Register server</Button>
                    </DialogTrigger>
                    <DialogContent className="sm:max-w-[425px]">
                        <DialogHeader>
                            <DialogTitle>Register server</DialogTitle>
                            <DialogDescription>
                                Please enter the hostname, IP address, and port of ssh to register the server.
                            </DialogDescription>
                        </DialogHeader>
                        <div className="grid gap-4">
                            <div className="grid gap-3">
                                <Label htmlFor="name-1">Hostname</Label>
                                <Input id="name-1" name="hostname" />
                            </div>
                            <div className="grid gap-3">
                                <Label htmlFor="username-1">IP address</Label>
                                <Input id="username-1" name="ip_address" />
                            </div>
                            <div className="grid gap-3">
                                <Label htmlFor="username-1">SSH port</Label>
                                <Input id="username-1" name="port" />
                            </div>
                        </div>
                        <DialogFooter>
                            <DialogClose asChild>
                                <Button variant="outline">Cancel</Button>
                            </DialogClose>
                            <Button type="submit">Save changes</Button>
                        </DialogFooter>
                    </DialogContent>
                </form>
            </Dialog>
        </>
    );
}
