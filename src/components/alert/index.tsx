import * as C from './style';
import { ExclamationTriangleIcon } from '@radix-ui/react-icons';

import { Alert, AlertDescription, AlertTitle } from '@/components/ui/alert';

export const AlertPage = () => {
	return (
		<C.Container>
			<Alert variant="default" style={{ width: '50%', height: '100px' }}>
				<ExclamationTriangleIcon className="h-4 w-4" />
				<AlertTitle>Alerta</AlertTitle>
				<AlertDescription>
					{' '}
					Parece que foi detetada uma ação suspeita durante a realização da sua atividade.
				</AlertDescription>
			</Alert>
		</C.Container>
	);
};
