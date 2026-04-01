import {Button, EmptyState, Pagination, Table} from "@heroui/react";
import {ChevronLeft, ChevronRight, CircleCheckFill, TrashBin} from "@gravity-ui/icons";
import type {Notification} from "../../types/Notifications.ts";
import {useMemo, useState} from "react";
import axios from "axios";

interface NotificationProps {
    notifications: Notification[],
    setNotifications: (value: Notification[]) => void,
    collector_id: number
}

const ROWS_PER_PAGE = 8;

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
    const url = `http://localhost:5000/collector/${props.collector_id}/notifications`

    function remove_notification(id: number) {
        const newNotifications = props.notifications.filter((n) => n.id !== id);
        props.setNotifications(newNotifications)
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
                                Timestamp
                                <Table.ColumnResizer/>
                            </Table.Column>
                            <Table.Column>
                                Threshold
                                <Table.ColumnResizer/>
                            </Table.Column>
                            <Table.Column>
                                Measured Values
                                <Table.ColumnResizer/>
                            </Table.Column>
                            <Table.Column>
                                Actions
                                <Table.ColumnResizer/>
                            </Table.Column>
                        </Table.Header>
                        <Table.Body renderEmptyState={() => (
                            <EmptyState
                                className={"flex flex-col gap-1 justify-center items-center bg-background  rounded-2xl py-8"}>
                                <CircleCheckFill className={"size-16 opacity-80"}/>
                                <span>No notifications found</span>
                            </EmptyState>
                        )}>
                            {
                                paginatedItems.map((n, i) => {
                                    const threshold = Math.floor(n.threshold_value) === n.threshold_value ? n.threshold_value : n.threshold_value.toFixed(2)
                                    const measured = n.measured_values.map((val) => (
                                        Math.floor(val) === val ? val : val.toFixed(2)
                                    ))

                                    return <Table.Row key={i}>
                                        <Table.Cell>
                                            <p>{n.component_name}</p>
                                        </Table.Cell>
                                        <Table.Cell>
                                            <p>{n.timestamp}</p>
                                        </Table.Cell>
                                        <Table.Cell>
                                            <p>{threshold}</p>
                                        </Table.Cell>
                                        <Table.Cell>
                                            <p>{measured.join(", ")}</p>
                                        </Table.Cell>
                                        <Table.Cell
                                            className={"flex gap-4 items-center *:transition-all " +
                                                "*:w-max *:h-max *:p-2 *:rounded-lg *:cursor-pointer " +
                                                " *:active:scale:95"}
                                        >
                                            <div
                                                className={"bg-red-100 hover:bg-red-200 " +
                                                    "hover:*:text-red-600"}
                                                onClick={() => {
                                                    axios.delete(`${url}/${n.id}`).then(() => {
                                                        remove_notification(n.id)
                                                    })
                                                }}
                                            >
                                                <TrashBin className={"size-5 text-red-500"}/>
                                            </div>
                                        </Table.Cell>
                                    </Table.Row>
                                })
                            }
                        </Table.Body>
                    </Table.Content>
                </Table.ResizableContainer>
                <Table.Footer className={"flex items-center justify-between"}>
                    {
                        props.notifications.length <= ROWS_PER_PAGE
                            ? <p className={"flex font-light text-sm"}>{props.notifications.length} result{props.notifications.length !== 1 && "s"}</p>
                            : <Pagination>
                                <Pagination.Summary>
                                    Results {start} to {end} of {props.notifications.length}
                                </Pagination.Summary>
                                <Pagination.Content>
                                    <Pagination.Item>
                                        <Pagination.Previous
                                            isDisabled={ page === 1 }
                                            onPress={() => setPage((p) => Math.max(1, p - 1))}
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
                                            onPress={() => setPage((p) => Math.min(totalPages, p + 1))}
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
            <Button variant={"danger-soft"} onClick={() => {
                axios.delete(url).then(() => {
                    props.setNotifications([])
                })
            }}>Remove all</Button>
        </div>
    )
}
