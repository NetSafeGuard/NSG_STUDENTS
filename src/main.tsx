import ReactDOM from "react-dom/client";
import App from "./App";
import './global.css';
import { TitleBar } from "./components/titlebar";
import { Toaster } from '@/components/ui/sonner';

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
	<>
		<TitleBar />
    	<Toaster />
		<App />
	</>,
);
