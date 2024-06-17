import * as C from './style.ts';
import { useForm } from 'react-hook-form';
import { yupResolver } from '@hookform/resolvers/yup';
import * as yup from 'yup';
import { Error } from './components/error/index.tsx';
import { useEffect, useState } from 'react';
import { getSocket } from './services/socket.ts';
import type { Socket } from 'socket.io-client';
import { Loading } from './components/loading/index.tsx';
import { toast } from 'sonner';

interface LoginData {
	usercode: string;
	activitycode: string;
}

interface Activity {
	title: string;
	description: string;
	startDate: Date;
	endDate: Date;
	groups: string[];
	createdAt?: Date;
	redirectUrl: string;
	id?: number;
}

const App = () => {
	const [socket, setSocket] = useState<Socket | null>(null);
	const [isConnected, setIsConnected] = useState<boolean>(false);
	const [activity, setActivity] = useState<Activity | null>(null);

	useEffect(() => {
		const setupSocket = async () => {
			const socket = await getSocket();
			setSocket(socket);
		};

		setupSocket();
	}, []);

	useEffect(() => {
		if (socket) {
			setIsConnected(true);
			socket.on('joined', activity => {
				setActivity(activity);
        window.location.href = activity.redirectUrl;
			});
		}
	}, [socket]);

	const LoginSchema = yup.object().shape({
		usercode: yup.string().required('Código de aluno obrigatório'),
		activitycode: yup.string().required('Código de atividade obrigatório'),
	});

	const Login = (data: LoginData) => {
		socket!.emit('joinActivity', data, (error: string) => {
			if (error) toast.error('Erro', { description: error });
		});
	};

	const {
		register,
		handleSubmit,
		watch,
		formState: { errors },
	} = useForm<LoginData>({
		resolver: yupResolver(LoginSchema),
	});

	if (!isConnected || !socket) {
		return <Loading />;
	}

	if (activity)
		return (
			<C.Container>
				<h1>...</h1>
			</C.Container>
		);

	return (
		<>
			<C.Container>
				<C.Left />
				<C.Right>
					<C.Title>Autenticação</C.Title>
					<C.Form onSubmit={handleSubmit(Login)}>
						<C.Label htmlFor="usercode">
							Código de aluno {errors.usercode && <Error error={'*'} />}
						</C.Label>
						<C.Input
							type="password"
							id="usercode"
							placeholder="Insira o código de aluno fornecido"
							{...register('usercode')}
							onFocus={() => !!watch('usercode')}
						/>

						<C.Label htmlFor="activitycode">
							Código da atividade {errors.activitycode && <Error error={'*'} />}
						</C.Label>
						<C.Input
							type="password"
							id="activitycode"
							placeholder="Insira o código da atividade fornecido"
							{...register('activitycode')}
							onFocus={() => !!watch('activitycode')}
						/>

						<C.Button>Entrar</C.Button>

						<C.Credits>
							<span>Desenvolvido por João Silva </span>
							<span> | </span>
							<span>2024</span>
						</C.Credits>
					</C.Form>
				</C.Right>
			</C.Container>
		</>
	);
};

export default App;
