import {Button} from "@heroui/react";
import {useNavigate} from "react-router";

export function Header() {
    const navigate = useNavigate();

    return (
        <header className={"fixed top-0 left-0 h-16"}>
            <Button variant={"ghost"} onClick={() => navigate("/")}>Home</Button>
        </header>
    )
}