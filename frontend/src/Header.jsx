import {Avatar, Button} from "@heroui/react";
import {Moon, Person} from '@gravity-ui/icons'

function Header() {
    return <header className={"border-b justify-between"}>
        <div className={"flex gap-8 items-center"}>
            <Button variant={"tertiary"}>Dashboard</Button>
            {/*<Button variant={"ghost"}>Account</Button>*/}
            <Button variant={"ghost"}>Settings</Button>
        </div>
        <div>
            <Button isIconOnly variant={"primary"} size={"lg"}>
                <Person/>
            </Button>
        </div>
    </header>
}

export default Header