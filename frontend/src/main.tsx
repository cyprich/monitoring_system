import { createRoot } from 'react-dom/client'
import './index.css'
import Dashboard from './pages/Dashboard.tsx'
import {BrowserRouter, Route, Routes} from 'react-router';
import {Header} from "./components/Header.tsx";
import {Footer} from "./components/Footer.tsx";

createRoot(document.getElementById('root')!).render(
    <BrowserRouter>
        <Header/>
        <Routes>
            <Route path={"/"} element={<Dashboard/>}/>
        </Routes>
        <Footer/>
    </BrowserRouter>
)
