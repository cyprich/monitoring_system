// import ChartAreaStackedNormalizedIcon from '@gravity-ui/icons/svgs/chart-area-stacked-normalized.svg';
import HouseIcon from '@gravity-ui/icons/svgs/house.svg';
import LayoutSideContentLeftIcon from '@gravity-ui/icons/svgs/layout-side-content-left.svg';
import PersonIcon from '@gravity-ui/icons/svgs/person.svg';
import GearIcon from '@gravity-ui/icons/svgs/gear.svg';
import CircleInfoIcon from '@gravity-ui/icons/svgs/circle-info.svg';
import BellIcon from '@gravity-ui/icons/svgs/bell.svg';
import {useState} from "react";
import {Link} from "react-router";

export default function Sidebar() {
    const [isExpanded, setIsExpanded] = useState<boolean>(false)

    return (
        <>
            <aside
                className={`sticky top-0 left-0 ${isExpanded ? "w-56" : "w-16"} h-screen max-h-screen flex flex-col gap-2 items-start px-5 py-4 bg-background-secondary z-40 duration-100`}>
                <SidebarItem
                    name={""}
                    icon={LayoutSideContentLeftIcon}
                    expanded={isExpanded}
                    onClick={() => setIsExpanded(!isExpanded)}
                    className={`mb-8 ${isExpanded ? "cursor-w-resize!" : "cursor-e-resize!"}`}/>
                <SidebarItem
                    name={"Home"}
                    link={"/"}
                    icon={HouseIcon}
                    expanded={isExpanded}/>
                <SidebarItem
                    name={"Account"}
                    link={"/account"}
                    icon={PersonIcon}
                    expanded={isExpanded}/>
                <SidebarItem
                    name={"Notifications"}
                    link={"/notifications"}
                    icon={BellIcon}
                    expanded={isExpanded}/>
                <SidebarItem
                    name={"Settings"}
                    link={"/settings"}
                    icon={GearIcon}
                    expanded={isExpanded}/>

                <div className={"size-full"}/>
                <SidebarItem
                    name={"About"}
                    link={"/about"}
                    icon={CircleInfoIcon}
                    expanded={isExpanded}/>
            </aside>
        </>
    )
}

interface SidebarItemProps {
    name: string,
    icon: string,
    expanded: boolean,
    link?: string,
    className?: string,
    onClick?: () => void,
}

function SidebarItem(props: SidebarItemProps) {
    const globalClassName = `${props.className} w-full py-2 z-50 clickable`

    const content = (
        <div onClick={props.onClick} className={`${globalClassName} flex gap-4 group`} title={props.name}>
            <img src={props.icon} alt={""} className={"size-6"}/>
            <p className={`${props.expanded ? "block opacity-100" : "hidden opacity-0"} group-hover:underline underline-offset-4`}>{props.name}</p>
        </div>
    )

    return (
        <>
            {
                props.link
                    ? <Link to={props.link} className={globalClassName}>{content}</Link>
                    : <>{content}</>
            }
        </>
    )
}


