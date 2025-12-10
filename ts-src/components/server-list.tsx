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
    server_name: string,
    server_address: string,
}

interface DataTableProps<TData, TValue> {
    columns: ColumnDef<TData, TValue>[]
    data: TData[]
}

export const columns: ColumnDef<ServerInfo>[] = [
    {
        accessorKey: "server_name",
        header: "Server name",
    },
    {
        accessorKey: "server_address",
        header: "Server address",
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
                const response = await fetch('/api/server-list');
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
            <Dialog>
                <form>
                    <DialogTrigger asChild>
                        <Button variant="outline">Register server</Button>
                    </DialogTrigger>
                    <DialogContent className="sm:max-w-[425px]">
                        <DialogHeader>
                            <DialogTitle>Register server</DialogTitle>
                            <DialogDescription>
                                Please enter the server name, server address, and port to register the server.
                            </DialogDescription>
                        </DialogHeader>
                        <div className="grid gap-4">
                            <div className="grid gap-3">
                                <Label htmlFor="name-1">Server name</Label>
                                <Input id="name-1" name="name" />
                            </div>
                            <div className="grid gap-3">
                                <Label htmlFor="username-1">Server address</Label>
                                <Input id="username-1" name="address" />
                            </div>
                            <div className="grid gap-3">
                                <Label htmlFor="username-1">Port</Label>
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
            <DataTable columns={columns} data={components} />
        </>
    );
}
