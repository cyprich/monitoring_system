import { createRoot } from 'react-dom/client'
import './index.css'
import {BrowserRouter, Route, Routes} from 'react-router';
import Home from "./pages/Home.tsx";
import Collector from "./pages/Collector.tsx";
import Sidebar from "./components/Sidebar.tsx";
import Account from "./pages/Account.tsx";
import Notifications from "./pages/Notifications.tsx";
import Settings from "./pages/Settings.tsx";
import About from "./pages/About.tsx";

createRoot(document.getElementById('root')!).render(
    <BrowserRouter>
        <div className={"flex"}>
            <Sidebar/>
                <Routes>
                    <Route path={"/"} element={<Home/>}/>
                    <Route path={"/collector/:id"} element={<Collector/>}/>
                    <Route path={"/account"} element={<Account/>}/>
                    <Route path={"/notifications"} element={<Notifications/>}/>
                    <Route path={"/settings"} element={<Settings/>}/>
                    <Route path={"/about"} element={<About/>}/>
                </Routes>
        </div>
        {/*<Footer/>*/}
    </BrowserRouter>
)
