import styled, { keyframes } from 'styled-components';

const redBackground = keyframes`
    from {
        background-color: #FF5C5C;
    }
    to {
        background-color: #FF1C1C;
    }
`;

export const Container = styled.div`
    display: flex;
    height: calc(100vh - 34px);
    animation: ${redBackground} 400ms infinite alternate;
    background-color: #FF1C1C;

    padding: 30px;
`;
