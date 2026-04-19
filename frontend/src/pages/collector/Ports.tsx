import {Label, Switch, Table} from "@heroui/react";
import {TableEmptyContent} from "../../components/TableEmptyContent.tsx";
import { useEffect, useState } from "react";
import type {PortsInterface, PortsNotificationSettings} from "../../types/PortsInterface.ts";
import axios from "axios";
import { getBaseUrl } from "../../helpFunctions.ts";

interface PortsProps {
    collector_id: number
    ports: PortsInterface[]
}

export default function Ports(props: PortsProps) {
    const [isOpenedSwitchSelected, setIsOpenedSwitchSelected] = useState<boolean>(false)
    const [isClosedSwitchSelected, setIsClosedSwitchSelected] = useState<boolean>(false)

    const url = getBaseUrl() + `/collector/${props.collector_id}/ports/notifications_settings`

    useEffect(() => {
        axios
            .get<PortsNotificationSettings>(url)
            .then(resp => {
                setIsOpenedSwitchSelected(resp.data.show_for_opened)
                setIsClosedSwitchSelected(resp.data.show_for_closed)
            })
    }, [url]);

    function handleToggle(newValue: boolean, portState: "opened" | "closed") {
        if (portState === "opened") {
            setIsOpenedSwitchSelected(newValue)
        } else {
            setIsClosedSwitchSelected(newValue)
        }

        axios
            .put(url + "/" + portState, {value: newValue})
            .then(() => {})
            .catch(e => console.error(e))
    }

    return (
        <div className={"flex flex-col gap-4"}>
            <Table>
                <Table.ResizableContainer>
                    <Table.Content aria-label="Ports">
                        <Table.Header>
                            <Table.Column isRowHeader>
                                IP Address
                                <Table.ColumnResizer/>
                            </Table.Column>
                            <Table.Column>
                                Port
                                <Table.ColumnResizer/>
                            </Table.Column>
                            <Table.Column>
                                Protocol
                                <Table.ColumnResizer/>
                            </Table.Column>
                            <Table.Column>
                                Last Update
                                <Table.ColumnResizer/>
                            </Table.Column>
                        </Table.Header>
                        <Table.Body renderEmptyState={() => (
                            <TableEmptyContent text={["No ports found"]} icon={"tray"}/>
                        )}>
                            {
                                props.ports.map((p, i) => {
                                    let c = "w-5 h-5 rounded-full "
                                    switch (p.protocol.toLowerCase()) {
                                        case "tcp":
                                            c += " bg-blue-400"
                                            break
                                        case "udp":
                                            c += " bg-green-400"
                                            break
                                    }

                                    return <Table.Row key={i}>
                                        <Table.Cell>{p.address}</Table.Cell>
                                        <Table.Cell>{p.port}</Table.Cell>
                                        <Table.Cell className={"flex gap-2"}>
                                            <div className={c}/>
                                            {p.protocol.toUpperCase()}
                                        </Table.Cell>
                                        <Table.Cell>{p.last_update}</Table.Cell>
                                    </Table.Row>
                                })
                            }
                        </Table.Body>
                    </Table.Content>
                </Table.ResizableContainer>
                {
                    props.ports.length > 0 &&
                    <Table.Footer>
                        <p className={"font-light text-sm"}>{props.ports.length} listening port{props.ports.length !== 1 && "s"}</p>
                    </Table.Footer>
                }
            </Table>
            <div className={"flex flex-col gap-2"}>
                <Switch
                    isSelected={isOpenedSwitchSelected}
                    onChange={val => handleToggle(val, "opened")}
                >
                    <Switch.Control>
                        <Switch.Thumb/>
                    </Switch.Control>
                    <Switch.Content>
                        <Label>
                            <p className={"font-light"}>Notifications for newly <span className={"font-medium"}>opened</span> ports</p>
                        </Label>
                    </Switch.Content>
                </Switch>
                <Switch
                    isSelected={isClosedSwitchSelected}
                    onChange={val => handleToggle(val, "closed")}
                >
                    <Switch.Control>
                        <Switch.Thumb/>
                    </Switch.Control>
                    <Switch.Content>
                        <Label>
                            <p className={"font-light"}>Notifications for newly <span className={"font-medium"}>closed</span> ports</p>
                        </Label>
                    </Switch.Content>
                </Switch>
            </div>
        </div>
    )
}