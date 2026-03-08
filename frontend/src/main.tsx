import { createRoot } from 'react-dom/client'
import './index.css'
import App from './pages/App.tsx'
import {BrowserRouter, Route, Routes} from 'react-router';
import Header from "./Header.tsx";
import Footer from "./Footer.tsx";

createRoot(document.getElementById('root')!).render(
    <BrowserRouter>
        <Header/>
        <Routes>
            <Route path={"/"} element={<App/>}/>
        </Routes>
        <Footer/>
    </BrowserRouter>
)
