import * as C from './style';
import Netsafe from '../../assets/netsafe-png.png';

export const AlertPage = () => {
	return (
		<C.Container>
			<C.image src={Netsafe} />
			<C.WarnTitle>Atividade Pausada</C.WarnTitle>
			<C.WarnDescription>
				Parece que foi detetada uma ação suspeita durante a realização da sua atividade.
			</C.WarnDescription>
			<C.Bottom>Aguarde o contacto do responsável pela a atividade.</C.Bottom>
		</C.Container>
	);
};
