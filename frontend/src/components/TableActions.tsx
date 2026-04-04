import {Table} from "@heroui/react";
import {Pencil, TrashBin} from "@gravity-ui/icons";

export interface TableActionsProps {
    deleteOnClick: () => void,
    showEdit?: boolean,
    editOnClick?: () => void,
}

export function TableActions(props: TableActionsProps) {
    return (
        <Table.Cell className={"flex gap-4 items-center " +
            "*:transition-all *:w-max *:h-max *:p-2 *:rounded-lg *:cursor-pointer *:active:scale:95"}>
            {
                props.showEdit && props.editOnClick &&
                <div className={"bg-gray-200 hover:bg-gray-300"} onClick={props.editOnClick}>
                    <Pencil className={"size-5"}/>
                </div>
            }
            <div className={"bg-red-100 hover:bg-red-200 hover:*:text-red-600"} onClick={props.deleteOnClick}>
                <TrashBin className={"size-5 text-red-500"}/>
            </div>
        </Table.Cell>
    )
}