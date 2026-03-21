import { createRoot } from 'react-dom/client'
import './index.css'
import {BrowserRouter, Route, Routes} from 'react-router';
import {Header} from "./components/Header.tsx";
import {Footer} from "./components/Footer.tsx";
import Home from "./pages/Home.tsx";
import Collector from "./pages/Collector.tsx";

createRoot(document.getElementById('root')!).render(
    <BrowserRouter>
        <Header/>
        <Routes>
            <Route path={"/"} element={<Home/>}/>
            <Route path={"/collector/:id"} element={<Collector/>}/>
        </Routes>
        <Footer/>
    </BrowserRouter>
)
