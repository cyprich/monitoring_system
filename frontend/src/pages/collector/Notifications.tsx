import {Button, Pagination, Table} from "@heroui/react";
import {ChevronLeft, ChevronRight} from "@gravity-ui/icons";
import type {Notification} from "../../types/Notifications.ts";
import {useMemo, useState} from "react";
import axios from "axios";
import {TableEmptyContent} from "../../components/TableEmptyContent.tsx";
import {TableActions} from "../../components/TableActions.tsx";

interface NotificationProps {
    notifications: Notification[],
    setNotifications: (value: Notification[]) => void,
    collector_id: number
}

const ROWS_PER_PAGE = 10;

export default function Notifications(props: NotificationProps) {
    const [page, setPage] = useState<number>(1);
    const totalPages = Math.ceil(props.notifications.length / ROWS_PER_PAGE);
    const pages = Array.from({length: totalPages}, (_, i) => i + 1);

    const paginatedItems = useMemo(() => {
        const start = (page - 1) * ROWS_PER_PAGE;
        return props.notifications.slice(start, start + ROWS_PER_PAGE);
    }, [props.notifications, page]);

    const start = (page - 1) * ROWS_PER_PAGE + 1;
    const end = Math.min(page * ROWS_PER_PAGE, props.notifications.length);

    // TODO url
    const base_url = `http://localhost:5000/collector/${props.collector_id}/notifications`

    function remove_notification(id: number | null) {
        let url = base_url;
        if (typeof id === "number") {
            url += `/${id}`
        }

        axios.delete(`${url}`).then(() => {
            let newNotifications: Notification[];

            if (typeof id === "number") {
                newNotifications = props.notifications.filter(n => n.id !== id);
            } else {
                newNotifications = []
            }

            props.setNotifications(newNotifications)
        })
    }

    // TODO show when user does not have any registered thresholds = no notifications

    return (
        <div className={"flex flex-col gap-4"}>
            <Table>
                <Table.ResizableContainer>
                    <Table.Content aria-label="Notifications">
                        <Table.Header>
                            <Table.Column isRowHeader>
                                Cause
                                <Table.ColumnResizer/>
                            </Table.Column>
                            <Table.Column>
                                Description
                                <Table.ColumnResizer/>
                            </Table.Column>
                            <Table.Column>
                                Date and Time
                                <Table.ColumnResizer/>
                            </Table.Column>
                            <Table.Column>
                                Actions
                                <Table.ColumnResizer/>
                            </Table.Column>
                        </Table.Header>
                        <Table.Body renderEmptyState={() => (
                            <TableEmptyContent text={["No notifications found"]} icon={"check"}/>
                        )}>
                            {
                                paginatedItems.map((n, i) => {
                                    return <Table.Row key={i}>
                                        <Table.Cell>
                                            <p>{n.cause}</p>
                                        </Table.Cell>
                                        <Table.Cell>
                                            <p>{n.description || ""}</p>
                                        </Table.Cell>
                                        <Table.Cell>
                                            <p>{n.time}</p>
                                        </Table.Cell>
                                        <TableActions deleteOnClick={() => {remove_notification(n.id)}}/>
                                    </Table.Row>
                                })
                            }
                        </Table.Body>
                    </Table.Content>
                </Table.ResizableContainer>
                <Table.Footer className={"flex items-center justify-between"}>
                    {
                        props.notifications.length <= ROWS_PER_PAGE
                            ? <p className={"flex font-light text-sm"}>
                                {props.notifications.length} result{props.notifications.length !== 1 && "s"}
                            </p>
                            : <Pagination>
                                <Pagination.Summary>
                                    Results {start} to {end} of {props.notifications.length}
                                </Pagination.Summary>
                                <Pagination.Content>
                                    <Pagination.Item>
                                        <Pagination.Previous
                                            isDisabled={ page === 1 }
                                            onPress={() => setPage(p => Math.max(1, p - 1))}
                                        >
                                            <ChevronLeft/>
                                            Prev
                                        </Pagination.Previous>
                                    </Pagination.Item>
                                    {
                                        pages.map((p) => (
                                            <Pagination.Item key={p}>
                                                <Pagination.Link isActive={p === page} onPress={() => setPage(p)}>
                                                    {p}
                                                </Pagination.Link>
                                            </Pagination.Item>
                                        ))
                                    }
                                    <Pagination.Item>
                                        <Pagination.Next
                                            isDisabled={ page === totalPages }
                                            onPress={() => setPage(p => Math.min(totalPages, p + 1))}
                                        >
                                            Next
                                            <ChevronRight/>
                                        </Pagination.Next>

                                    </Pagination.Item>
                                </Pagination.Content>
                            </Pagination>
                    }
                </Table.Footer>
            </Table>
            <Button variant={"danger-soft"} onClick={() => { remove_notification(null) }}>Remove all</Button>
        </div>
    )
}
