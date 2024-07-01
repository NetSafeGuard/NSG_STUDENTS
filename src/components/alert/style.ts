import styled from 'styled-components';

export const Container = styled.div`
    display: flex;
    flex-direction: column;
    height: calc(100vh - 32px);
    align-items: center;
    background-color: #da3639;
    padding-top: 10vh;
`;

export const image = styled.img`
    width: 300px;
    opacity: 0.5;
    margin-bottom: 50px;
`;

export const WarnTitle = styled.div`
    font-size: 38px;
    color: #fbd0d0;
    letter-spacing: 3px;
    font-weight: 500;
`;

export const WarnDescription = styled.p`
    margin-top: 20px;
    font-size: 17px;
    color: #e98b89;
`;

export const Bottom = styled.span`
    position:absolute;
    font-size: 15px;
    color: #e98b89;
    bottom: 45px;
`;
