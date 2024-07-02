import ReactDOM from 'react-dom/client';
import './global.css';
import { TitleBar } from './components/titlebar';
import { Toaster } from '@/components/ui/sonner';
import { RoutesList } from './router/routes';

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
	<>
		<TitleBar />
		<Toaster />
		<RoutesList />
	</>,
);
