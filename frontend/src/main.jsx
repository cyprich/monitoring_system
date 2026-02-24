import ReactDOM from "react-dom/client";
import {BrowserRouter, Route, Routes} from "react-router";

import './index.css'
import App from './pages/App.jsx'
import Header from "./Header.jsx";
import Footer from "./Footer.jsx";

const root = document.getElementById("root");

ReactDOM.createRoot(root).render(
    <BrowserRouter>
        <Header/>
        <Routes>
            <Route path={"/"} element={<App/>}/>
        </Routes>
        <Footer/>
    </BrowserRouter>
)